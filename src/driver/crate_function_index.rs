use std::collections::HashMap;

use lele_rust_function_taxonomy::HonestyGraph;
use lele_rust_function_taxonomy::NodeId;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_middle::mir::{BasicBlockData, Body, Operand, TerminatorKind};
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
        let owners = Self::sorted_function_owners(tcx);
        let crate_functions = Self::from_tcx_config_and_owners(tcx, config, &owners);
        Self::with_calls_and_closures_linked(tcx, &owners, crate_functions)
    }

    pub(crate) fn from_tcx_config_and_owners(
        tcx: TyCtxt<'_>,
        config: &build_graph::ReportConfig,
        owners: &[(String, LocalDefId)],
    ) -> CrateFunctionIndex {
        let (mut graph, mut paths, mut node_ids, mut spans, scan) = (
            HonestyGraph::default(),
            HashMap::new(),
            HashMap::new(),
            HashMap::new(),
            build_graph::DirectCauseScan { tcx, config },
        );

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

    pub(crate) fn with_calls_and_closures_linked(
        tcx: TyCtxt<'_>,
        owners: &[(String, LocalDefId)],
        mut crate_functions: CrateFunctionIndex,
    ) -> CrateFunctionIndex {
        for (_, def_id) in owners {
            let Some(caller) = crate_functions.node_ids.get(def_id).copied() else {
                continue;
            };
            for callee_def in Self::body_calls(tcx, *def_id, &crate_functions.node_ids) {
                if let Some(callee) = crate_functions.node_ids.get(&callee_def).copied() {
                    crate_functions.graph.add_edge(caller, callee);
                }
            }
            for child in Self::closures_of(tcx, *def_id) {
                if let Some(target) = crate_functions.node_ids.get(&child).copied() {
                    crate_functions.graph.add_edge(caller, target);
                }
            }
        }
        crate_functions
    }
}

#[rustfmt::skip]
impl CrateFunctionIndex {
    pub(crate) fn sorted_function_owners(tcx: TyCtxt<'_>) -> Vec<(String, LocalDefId)> {
        let mut owners: Vec<(String, LocalDefId)> = tcx
            .hir_body_owners()
            .filter(|def_id| Self::is_function_like(tcx, *def_id))
            .map(|def_id| (tcx.def_path_str(def_id.to_def_id()), def_id))
            .collect();
        owners.sort_by(|(left, _), (right, _)| left.cmp(right));
        owners
    }

    pub(crate) fn is_function_like(tcx: TyCtxt<'_>, def_id: LocalDefId) -> bool {
        matches!(
            tcx.def_kind(def_id),
            DefKind::Fn | DefKind::AssocFn | DefKind::Closure | DefKind::SyntheticCoroutineBody
        )
    }

    pub(crate) fn body_calls(
        tcx: TyCtxt<'_>,
        def_id: LocalDefId,
        local: &HashMap<LocalDefId, NodeId>,
    ) -> Vec<LocalDefId> {
        let body = tcx.optimized_mir(def_id);
        let env = body.typing_env(tcx);
        body.basic_blocks
            .iter()
            .filter_map(|bb| Self::local_call_in_block(tcx, body, env, bb, local))
            .collect()
    }

    pub(crate) fn local_call_in_block<'tcx>(
        tcx: TyCtxt<'tcx>,
        body: &Body<'tcx>,
        env: ty::TypingEnv<'tcx>,
        bb: &BasicBlockData<'tcx>,
        local: &HashMap<LocalDefId, NodeId>,
    ) -> Option<LocalDefId> {
        let func = Self::call_func_of_block(bb)?;
        Self::local_callee_of_func(tcx, body, env, func, local)
    }

    pub(crate) fn call_func_of_block<'a, 'tcx>(
        bb: &'a BasicBlockData<'tcx>,
    ) -> Option<&'a Operand<'tcx>> {
        let term = bb.terminator.as_ref()?;
        if let TerminatorKind::Call { func, .. } | TerminatorKind::TailCall { func, .. } =
            &term.kind
        {
            Some(func)
        } else {
            None
        }
    }

    pub(crate) fn local_callee_of_func<'tcx>(
        tcx: TyCtxt<'tcx>,
        body: &Body<'tcx>,
        env: ty::TypingEnv<'tcx>,
        func: &Operand<'tcx>,
        local: &HashMap<LocalDefId, NodeId>,
    ) -> Option<LocalDefId> {
        let ty = func.ty(body, tcx);
        let resolved = Self::resolved_callee_from_ty(tcx, env, ty)?;
        resolved.as_local().filter(|id| local.contains_key(id))
    }

    pub(crate) fn resolved_callee_from_ty<'tcx>(
        tcx: TyCtxt<'tcx>,
        env: ty::TypingEnv<'tcx>,
        ty: ty::Ty<'tcx>,
    ) -> Option<DefId> {
        let ty::FnDef(callee, args) = ty.kind() else {
            return None;
        };
        Some(match Instance::try_resolve(tcx, env, *callee, args.skip_binder()) {
            Ok(Some(instance)) => instance.def_id(),
            _ => *callee,
        })
    }

    pub(crate) fn closures_of(tcx: TyCtxt<'_>, def_id: LocalDefId) -> Vec<LocalDefId> {
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
}

// no test_usage necessary
