use std::path::Path;

use lele_rust_function_taxonomy::constants;
use lele_rust_function_taxonomy::lele_config;

pub struct ReportConfig {
    pub honest_boundaries: Vec<lele_config::BoundaryEntry>,
    pub declared_dishonest: Vec<lele_config::CallPath>,
    pub declared_honest: Vec<lele_config::CallPath>,
}

impl ReportConfig {
    pub(crate) fn from_env() -> Self {
        let config = std::env::var_os(constants::ENV_MANIFEST)
            .and_then(|manifest| lele_config::Config::try_from(Path::new(&manifest)).ok())
            .unwrap_or_default();
        Self {
            honest_boundaries: config.honest_boundaries().cloned().collect(),
            declared_honest: config.declared_honest,
            declared_dishonest: config.declared_dishonest,
        }
    }
}

#[rustfmt::skip]
impl ReportConfig {
    pub fn boundary_for(&self, rel: &Path) -> Option<&lele_config::BoundaryName> {
        self.honest_boundaries
            .iter()
            .find(|b| b.folders.iter().any(|f| rel.starts_with(f.as_path())))
            .map(|b| &b.name)
    }
}

// no test_usage necessary
