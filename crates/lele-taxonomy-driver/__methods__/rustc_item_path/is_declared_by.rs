use lele_rust_function_taxonomy::lele_config;

use crate::roots;

pub fn is_declared_by(path: &roots::RustcItemPath, list: &[lele_config::CallPath]) -> bool {
    list.iter()
        .any(|declared| path.as_str() == declared.as_str() || path.ends_with(declared.as_str()))
}

#[cfg(test)]
mod tests {
    use super::is_declared_by;
    use crate::roots;
    use lele_rust_function_taxonomy::lele_config;

    #[test]
    fn test_usage() {
        let declared = [lele_config::CallPath("std::fs::read".to_string())];
        assert!(is_declared_by(
            &roots::RustcItemPath("std::fs::read".to_string()),
            &declared
        ));
        assert!(is_declared_by(
            &roots::RustcItemPath("my_crate::std::fs::read".to_string()),
            &declared
        ));
        assert!(!is_declared_by(
            &roots::RustcItemPath("std::fs::write".to_string()),
            &declared
        ));
    }
}
