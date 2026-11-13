use rustc_hir::def::DefKind;

pub fn is_function_kind(kind: DefKind) -> bool {
    matches!(
        kind,
        DefKind::Fn | DefKind::AssocFn | DefKind::Closure | DefKind::SyntheticCoroutineBody
    )
}

#[cfg(test)]
mod tests {
    use super::is_function_kind;
    use rustc_hir::def::DefKind;

    #[test]
    fn test_usage() {
        assert!(is_function_kind(DefKind::Fn));
        assert!(is_function_kind(DefKind::Closure));
        assert!(!is_function_kind(DefKind::Struct));
        assert!(!is_function_kind(DefKind::Mod));
    }
}
