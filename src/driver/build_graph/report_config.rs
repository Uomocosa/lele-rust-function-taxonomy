use std::path::Path;

use lele_rust_function_taxonomy::lele_config;

pub struct ReportConfig {
    pub honest_boundaries: Vec<lele_config::BoundaryEntry>,
    pub declared_dishonest: Vec<lele_config::CallPath>,
    pub declared_honest: Vec<lele_config::CallPath>,
}

#[rustfmt::skip]
impl ReportConfig {
    // needed helper: the boundary describing a crate-relative file, if any
    pub fn boundary_for(&self, rel: &Path) -> Option<&lele_config::BoundaryName> {
        self.honest_boundaries
            .iter()
            .find(|b| b.folders.iter().any(|f| rel.starts_with(f.as_path())))
            .map(|b| &b.name)
    }
}

// no test_usage necessary
