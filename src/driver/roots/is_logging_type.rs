pub fn is_logging_type(type_path: &str) -> bool {
    type_path.starts_with("tracing_core::callsite::")
        || type_path.starts_with("tracing::")
        || type_path.starts_with("log::")
}

#[cfg(test)]
mod tests {
    use super::is_logging_type;

    #[test]
    fn test_usage() {
        assert!(is_logging_type("tracing_core::callsite::DefaultCallsite"));
        assert!(is_logging_type("tracing::foo::Bar"));
        assert!(is_logging_type("log::foo::Baz"));
        assert!(!is_logging_type("std::sync::OnceLock"));
    }
}
