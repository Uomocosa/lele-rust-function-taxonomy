use lele_rust_function_taxonomy::lele_config;
use lele_rust_function_taxonomy::DirectCause;
use rustc_middle::mir::{Operand, Rvalue};
use rustc_middle::ty::{self, TyCtxt};

use crate::build_graph;
use crate::roots;

pub fn static_cause<'tcx>(
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
            let static_path = roots::RustcItemPath(tcx.def_path_str(static_id));
            if static_path.is_declared_by(declared_honest) {
                return None;
            }
            let name = tcx.def_path_str(static_id);
            let is_mutable = tcx.is_mutable_static(static_id);
            let ty = tcx
                .type_of(static_id)
                .instantiate_identity()
                .skip_normalization();
            let is_freeze = ty.is_freeze(tcx, env);
            let is_logging = build_graph::DirectCauseScan::is_logging_static(tcx, ty);
            build_graph::DirectCauseScan::static_verdict(name, is_mutable, is_freeze, is_logging)
        }
        Rvalue::ThreadLocalRef(def_id) => Some(DirectCause::ThreadLocal(tcx.def_path_str(*def_id))),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::static_cause;

    #[test]
    fn test_usage() {
        let _ = static_cause;
    }
}
