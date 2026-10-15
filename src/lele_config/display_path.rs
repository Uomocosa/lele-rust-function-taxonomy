use std::path::Path;

pub fn display_path(manifest_path: &Path) -> String {
    manifest_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .map_or_else(
            || "lele.toml".to_string(),
            |name| format!("{name}/lele.toml"),
        )
}

// no test_usage necessary
