use atomic_delegate_macros::atomic_delegates;
use lele_rust_function_taxonomy::lele_config;
use lele_rust_function_taxonomy::DirectCause;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_middle::mir::{Body, Operand, Rvalue, StatementKind, TerminatorKind};
use rustc_middle::ty::{self, Instance, TyCtxt};

use crate::build_graph;
use crate::roots;

pub struct DirectCauseScan<'tcx, 'config> {
    pub tcx: TyCtxt<'tcx>,
    pub config: &'config build_graph::ReportConfig,
}

impl DirectCauseScan<'_, '_> {
    pub fn of(&self, def_id: LocalDefId) -> Option<DirectCause> {
        let path_str = self.tcx.def_path_str(def_id.to_def_id());
        let item_path = roots::RustcItemPath(path_str.clone());
        match (
            item_path.is_declared_by(&self.config.declared_honest),
            item_path.is_declared_by(&self.config.declared_dishonest),
        ) {
            (true, _) => None,
            (_, true) => Some(DirectCause::DeclaredDishonest(path_str)),
            _ => self.body_cause(def_id),
        }
    }

    pub(crate) fn body_cause(&self, def_id: LocalDefId) -> Option<DirectCause> {
        let body = self.tcx.optimized_mir(def_id);
        let env = body.typing_env(self.tcx);
        Self::first_cause(self.tcx, body, env, &self.config.declared_honest)
    }

    pub(crate) fn first_cause<'tcx>(
        tcx: TyCtxt<'tcx>,
        body: &Body<'tcx>,
        env: ty::TypingEnv<'tcx>,
        declared_honest: &[lele_config::CallPath],
    ) -> Option<DirectCause> {
        for bb in body.basic_blocks.iter() {
            for stmt in &bb.statements {
                if let StatementKind::Assign(assign) = &stmt.kind {
                    let (_, rvalue) = &**assign;
                    if let Some(cause) = Self::static_cause(tcx, env, rvalue, declared_honest) {
                        return Some(cause);
                    }
                }
            }
            let Some(term) = &bb.terminator else {
                continue;
            };
            match &term.kind {
                TerminatorKind::Call { func, .. } | TerminatorKind::TailCall { func, .. } => {
                    if let Some(cause) = Self::call_cause(tcx, body, func, declared_honest) {
                        return Some(cause);
                    }
                }
                _ => {}
            }
        }
        None
    }

    pub(crate) fn is_logging_static<'tcx>(tcx: TyCtxt<'tcx>, ty: ty::Ty<'tcx>) -> bool {
        match ty.kind() {
            ty::Adt(adt, _) => roots::RustcItemPath(tcx.def_path_str(adt.did())).is_logging_type(),
            ty::Ref(_, inner, _) | ty::RawPtr(inner, _) => Self::is_logging_static(tcx, *inner),
            ty::Array(inner, _) | ty::Slice(inner) => Self::is_logging_static(tcx, *inner),
            _ => false,
        }
    }

    pub(crate) fn call_cause<'tcx>(
        tcx: TyCtxt<'tcx>,
        body: &Body<'tcx>,
        func: &Operand<'tcx>,
        declared_honest: &[lele_config::CallPath],
    ) -> Option<DirectCause> {
        let callee = Self::resolved_callee(tcx, body, func)?;
        let callee_path = roots::RustcItemPath(tcx.def_path_str(callee));
        Self::io_root_unless_declared(&callee_path, declared_honest)
    }

    pub(crate) fn resolved_callee<'tcx>(
        tcx: TyCtxt<'tcx>,
        body: &Body<'tcx>,
        func: &Operand<'tcx>,
    ) -> Option<DefId> {
        let ty = func.ty(body, tcx);
        let ty::FnDef(def_id, args) = ty.kind() else {
            return None;
        };
        Some(Self::try_resolved_def(
            tcx,
            body,
            *def_id,
            args.skip_binder(),
        ))
    }

    pub(crate) fn try_resolved_def<'tcx>(
        tcx: TyCtxt<'tcx>,
        body: &Body<'tcx>,
        def_id: DefId,
        args: ty::GenericArgsRef<'tcx>,
    ) -> DefId {
        let env = body.typing_env(tcx);
        match Instance::try_resolve(tcx, env, def_id, args) {
            Ok(Some(instance)) => instance.def_id(),
            _ => def_id,
        }
    }

    pub(crate) fn io_root_unless_declared(
        callee_path: &roots::RustcItemPath,
        declared_honest: &[lele_config::CallPath],
    ) -> Option<DirectCause> {
        if callee_path.is_declared_by(declared_honest) {
            return None;
        }
        callee_path.is_io_root(&[]).map(DirectCause::IoRoot)
    }
}

#[atomic_delegates]
impl DirectCauseScan<'_, '_> {
    pub(crate) fn static_cause<'tcx>(
        tcx: TyCtxt<'tcx>,
        env: ty::TypingEnv<'tcx>,
        rvalue: &Rvalue<'tcx>,
        declared_honest: &[lele_config::CallPath],
    ) -> Option<DirectCause> {
    }
    pub(crate) fn static_verdict(
        name: String,
        is_mutable: bool,
        is_freeze: bool,
        is_logging: bool,
    ) -> Option<DirectCause> {
    }
}

// no test_usage necessary
