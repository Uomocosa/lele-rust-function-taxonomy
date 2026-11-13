use std::path::Path;

use lele_rust_function_taxonomy::lele_config;

use crate::build_graph;

pub fn boundary_for<'a>(
    config: &'a build_graph::ReportConfig,
    rel: &Path,
) -> Option<&'a lele_config::BoundaryName> {
    config
        .honest_boundaries
        .iter()
        .find(|b| b.folders.iter().any(|f| rel.starts_with(f.as_path())))
        .map(|b| &b.name)
}

#[cfg(test)]
mod tests {
    use super::boundary_for;
    use crate::build_graph;
    use lele_rust_function_taxonomy::lele_config;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_usage() {
        let config = build_graph::ReportConfig {
            honest_boundaries: vec![lele_config::BoundaryEntry {
                name: lele_config::BoundaryName("core".to_string()),
                why: "w".to_string(),
                folders: vec![lele_config::FolderPath(PathBuf::from("src/core"))],
                cannot_use: Vec::new(),
                require: None,
            }],
            declared_honest: Vec::new(),
            declared_dishonest: Vec::new(),
        };
        let found = boundary_for(&config, Path::new("src/core/a.rs"));
        assert_eq!(found.map(|name| name.0.as_str()), Some("core"));
        assert!(boundary_for(&config, Path::new("src/other/a.rs")).is_none());
    }
}
