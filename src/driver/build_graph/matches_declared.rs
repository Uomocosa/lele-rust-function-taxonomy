use lele_rust_function_taxonomy::lele_config;

pub fn matches_declared(path: &str, list: &[lele_config::CallPath]) -> bool {
    list.iter()
        .any(|d| path == d.as_str() || path.ends_with(d.as_str()))
}

#[cfg(test)]
mod tests {
    use super::matches_declared;
    use lele_rust_function_taxonomy::lele_config;

    #[test]
    fn test_usage() {
        let list = [lele_config::CallPath("std::fs::read".to_string())];
        assert!(matches_declared("my_crate::std::fs::read", &list));
        assert!(!matches_declared("std::fs::write", &list));
    }
}
