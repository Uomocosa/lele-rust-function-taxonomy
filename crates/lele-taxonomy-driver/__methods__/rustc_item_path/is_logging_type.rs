use crate::roots;

pub fn is_logging_type(path: &roots::RustcItemPath) -> bool {
    path.starts_with("tracing_core::callsite::")
        || path.starts_with("tracing::")
        || path.starts_with("log::")
}

#[cfg(test)]
mod tests {
    use super::is_logging_type;
    use crate::roots;

    #[test]
    fn test_usage() {
        assert!(is_logging_type(&roots::RustcItemPath(
            "tracing::Span".to_string()
        )));
        assert!(is_logging_type(&roots::RustcItemPath(
            "tracing_core::callsite::X".to_string()
        )));
        assert!(is_logging_type(&roots::RustcItemPath(
            "log::info".to_string()
        )));
        assert!(!is_logging_type(&roots::RustcItemPath(
            "std::fs::File".to_string()
        )));
    }
}
