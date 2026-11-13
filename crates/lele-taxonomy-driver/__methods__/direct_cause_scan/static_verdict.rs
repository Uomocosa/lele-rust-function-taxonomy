use lele_rust_function_taxonomy::DirectCause;

pub fn static_verdict(
    name: String,
    is_mutable: bool,
    is_freeze: bool,
    is_logging: bool,
) -> Option<DirectCause> {
    if is_mutable {
        return Some(DirectCause::HiddenStaticWrite(name));
    }
    if is_freeze || is_logging {
        return None;
    }
    Some(DirectCause::HiddenStaticRead(name))
}

#[cfg(test)]
mod tests {
    use super::static_verdict;
    use lele_rust_function_taxonomy::DirectCause;

    #[test]
    fn test_usage() {
        assert_eq!(
            static_verdict("s".to_string(), true, true, true),
            Some(DirectCause::HiddenStaticWrite("s".to_string()))
        );
        assert_eq!(static_verdict("s".to_string(), false, true, false), None);
        assert_eq!(static_verdict("s".to_string(), false, false, true), None);
        assert_eq!(
            static_verdict("s".to_string(), false, false, false),
            Some(DirectCause::HiddenStaticRead("s".to_string()))
        );
    }
}
