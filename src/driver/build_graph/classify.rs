use lele_rust_function_taxonomy::lele_config;
use lele_rust_function_taxonomy::DirectCause;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::mir::{Body, Operand, Rvalue, StatementKind, TerminatorKind};
use rustc_middle::ty::{self, Instance, TyCtxt};

use crate::build_graph;
use crate::roots;

pub fn classify(
    tcx: TyCtxt<'_>,
    def_id: LocalDefId,
    config: &build_graph::ReportConfig,
) -> Option<DirectCause> {
    let path_str = tcx.def_path_str(def_id.to_def_id());

    if build_graph::matches_declared(&path_str, &config.declared_honest) {
        return None;
    }
    if build_graph::matches_declared(&path_str, &config.declared_dishonest) {
        return Some(DirectCause::DeclaredDishonest(path_str));
    }

    let body = tcx.optimized_mir(def_id);
    let env = body.typing_env(tcx);

    for bb in body.basic_blocks.iter() {
        for stmt in &bb.statements {
            if let StatementKind::Assign(assign) = &stmt.kind {
                let (_, rvalue) = &**assign;
                if let Some(cause) = static_cause(tcx, env, rvalue, &config.declared_honest) {
                    return Some(cause);
                }
            }
        }
        let Some(term) = &bb.terminator else {
            continue;
        };
        match &term.kind {
            TerminatorKind::Call { func, .. } | TerminatorKind::TailCall { func, .. } => {
                if let Some(cause) = call_cause(tcx, body, func, &config.declared_honest) {
                    return Some(cause);
                }
            }
            _ => {}
        }
    }
    None
}

fn static_cause<'tcx>(
    tcx: TyCtxt<'tcx>,
    env: ty::TypingEnv<'tcx>,
    rvalue: &Rvalue<'tcx>,
    declared_honest: &[lele_config::CallPath],
) -> Option<DirectCause> {
    match rvalue {
        Rvalue::Use(op, _) | Rvalue::Repeat(op, _) | Rvalue::Cast(_, op, _) => {
            let Operand::Constant(constant) = op else {
                return None;
            };
            let static_id = constant.check_static_ptr(tcx)?;
            if build_graph::matches_declared(&tcx.def_path_str(static_id), declared_honest) {
                return None;
            }
            let name = tcx.def_path_str(static_id);
            if tcx.is_mutable_static(static_id) {
                return Some(DirectCause::HiddenStaticWrite(name));
            }
            let ty = tcx
                .type_of(static_id)
                .instantiate_identity()
                .skip_normalization();
            if ty.is_freeze(tcx, env) {
                return None;
            }
            if is_logging_static(tcx, ty) {
                return None;
            }
            Some(DirectCause::HiddenStaticRead(name))
        }
        Rvalue::ThreadLocalRef(def_id) => Some(DirectCause::ThreadLocal(tcx.def_path_str(*def_id))),
        _ => None,
    }
}

// needed helper: tracing/log callsite statics are sanctioned (logging is honest everywhere)
fn is_logging_static<'tcx>(tcx: TyCtxt<'tcx>, ty: ty::Ty<'tcx>) -> bool {
    match ty.kind() {
        ty::Adt(adt, _) => roots::is_logging_type(&tcx.def_path_str(adt.did())),
        ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => is_logging_static(tcx, *inner),
        ty::Array(inner, _) | ty::Slice(inner) => is_logging_static(tcx, *inner),
        _ => false,
    }
}

fn call_cause<'tcx>(
    tcx: TyCtxt<'tcx>,
    body: &Body<'tcx>,
    func: &Operand<'tcx>,
    declared_honest: &[lele_config::CallPath],
) -> Option<DirectCause> {
    let ty = func.ty(body, tcx);
    let ty::FnDef(def_id, args) = ty.kind() else {
        return None;
    };
    let env = body.typing_env(tcx);
    let callee = match Instance::try_resolve(tcx, env, *def_id, args.skip_binder()) {
        Ok(Some(instance)) => instance.def_id(),
        _ => *def_id,
    };
    let path = tcx.def_path_str(callee);
    if build_graph::matches_declared(&path, declared_honest) {
        return None;
    }
    roots::is_io_root(&path, &[]).map(DirectCause::IoRoot)
}

// no test_usage necessary
