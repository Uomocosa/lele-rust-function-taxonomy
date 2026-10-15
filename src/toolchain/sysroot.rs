pub fn sysroot() -> String {
    option_env!("LELE_TAXONOMY_SYSROOT")
        .map(str::to_string)
        .unwrap_or_default()
}

// no test_usage necessary
