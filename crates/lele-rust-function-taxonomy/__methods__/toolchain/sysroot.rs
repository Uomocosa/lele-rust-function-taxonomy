pub fn sysroot() -> String {
    option_env!("LELE_TAXONOMY_SYSROOT")
        .map(str::to_string)
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::sysroot;

    #[test]
    fn test_usage() {
        let expected = option_env!("LELE_TAXONOMY_SYSROOT").unwrap_or_default();
        assert_eq!(sysroot(), expected);
        assert_eq!(
            sysroot().is_empty(),
            option_env!("LELE_TAXONOMY_SYSROOT").is_none()
        );
    }
}
