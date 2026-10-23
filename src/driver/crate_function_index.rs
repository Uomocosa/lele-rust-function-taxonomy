use std::collections::HashMap;

use lele_rust_function_taxonomy::HonestyGraph;
use lele_rust_function_taxonomy::NodeId;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::mir::TerminatorKind;
use rustc_middle::ty::{self, Instance, TyCtxt};
use rustc_span::Span;

use crate::build_graph;

pub struct CrateFunctionIndex {
    pub graph: HonestyGraph,
    pub paths: HashMap<LocalDefId, String>,
    pub node_ids: HashMap<LocalDefId, NodeId>,
    pub spans: HashMap<LocalDefId, Span>,
}

impl CrateFunctionIndex {
    pub fn from_crate(tcx: TyCtxt<'_>, config: &build_graph::ReportConfig) -> Self {
        let owners = sorted_function_owners(tcx);
        let crate_functions = collect_nodes(tcx, config, &owners);
        with_calls_and_closures_linked(tcx, &owners, crate_functions)
    }
}

// needed helper: every function-like body in path order, so node ids stay deterministic
fn sorted_function_owners(tcx: TyCtxt<'_>) -> Vec<(String, LocalDefId)> {
    let mut owners: Vec<(String, LocalDefId)> = tcx
        .hir_body_owners()
        .filter(|def_id| is_function_like(tcx, *def_id))
        .map(|def_id| (tcx.def_path_str(def_id.to_def_id()), def_id))
        .collect();
    owners.sort_by(|(left, _), (right, _)| left.cmp(right));
    owners
}

// needed helper: one graph node per function-like body, keyed by its sorted index
fn collect_nodes(
    tcx: TyCtxt<'_>,
    config: &build_graph::ReportConfig,
    owners: &[(String, LocalDefId)],
) -> CrateFunctionIndex {
    let mut graph = HonestyGraph::default();
    let mut paths: HashMap<LocalDefId, String> = HashMap::new();
    let mut node_ids: HashMap<LocalDefId, NodeId> = HashMap::new();
    let mut spans: HashMap<LocalDefId, Span> = HashMap::new();
    let scan = build_graph::DirectCauseScan { tcx, config };

    for (index, (path_str, def_id)) in owners.iter().enumerate() {
        let id = NodeId(u32::try_from(index).unwrap_or(u32::MAX));
        node_ids.insert(*def_id, id);
        paths.insert(*def_id, path_str.clone());
        if let Some(span) = tcx.def_ident_span(def_id.to_def_id()) {
            spans.insert(*def_id, span);
        }
        let direct = scan.of(*def_id);
        graph.add_node(id, path_str.clone(), direct);
    }

    CrateFunctionIndex {
        graph,
        paths,
        node_ids,
        spans,
    }
}

// needed helper: caller -> callee edges, including closures defined inside the body
fn with_calls_and_closures_linked(
    tcx: TyCtxt<'_>,
    owners: &[(String, LocalDefId)],
    mut crate_functions: CrateFunctionIndex,
) -> CrateFunctionIndex {
    for (_, def_id) in owners {
        let Some(caller) = crate_functions.node_ids.get(def_id).copied() else {
            continue;
        };
        for callee_def in body_calls(tcx, *def_id, &crate_functions.node_ids) {
            if let Some(callee) = crate_functions.node_ids.get(&callee_def).copied() {
                crate_functions.graph.add_edge(caller, callee);
            }
        }
        for child in closures_of(tcx, *def_id) {
            if let Some(target) = crate_functions.node_ids.get(&child).copied() {
                crate_functions.graph.add_edge(caller, target);
            }
        }
    }
    crate_functions
}

fn is_function_like(tcx: TyCtxt<'_>, def_id: LocalDefId) -> bool {
    matches!(
        tcx.def_kind(def_id),
        DefKind::Fn | DefKind::AssocFn | DefKind::Closure | DefKind::SyntheticCoroutineBody
    )
}

// needed helper: resolved callee DefIds of a body, keyed by the local map
fn body_calls(
    tcx: TyCtxt<'_>,
    def_id: LocalDefId,
    local: &HashMap<LocalDefId, NodeId>,
) -> Vec<LocalDefId> {
    let body = tcx.optimized_mir(def_id);
    let env = body.typing_env(tcx);
    let mut out = Vec::new();
    for bb in body.basic_blocks.iter() {
        let Some(term) = &bb.terminator else {
            continue;
        };
        if let TerminatorKind::Call { func, .. } | TerminatorKind::TailCall { func, .. } =
            &term.kind
        {
            let ty = func.ty(body, tcx);
            if let ty::FnDef(callee, args) = ty.kind() {
                let resolved = match Instance::try_resolve(tcx, env, *callee, args.skip_binder()) {
                    Ok(Some(instance)) => instance.def_id(),
                    _ => *callee,
                };
                if let Some(local_id) = resolved.as_local() {
                    if local.contains_key(&local_id) {
                        out.push(local_id);
                    }
                }
            }
        }
    }
    out
}

// needed helper: closures/coroutines physically defined inside a body
fn closures_of(tcx: TyCtxt<'_>, def_id: LocalDefId) -> Vec<LocalDefId> {
    let mut out = Vec::new();
    for child in tcx.hir_body_owners() {
        if !matches!(
            tcx.def_kind(child),
            DefKind::Closure | DefKind::SyntheticCoroutineBody
        ) {
            continue;
        }
        if tcx.local_parent(child) == def_id {
            out.push(child);
        }
    }
    out
}

// no test_usage necessary
