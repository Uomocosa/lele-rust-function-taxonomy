use serde::Deserialize;

use crate::lele_config;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LeleToml {
    #[serde(default)]
    pub lele: LeleSection,
    #[serde(default)]
    pub honesty: HonestySection,
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LeleSection {
    #[serde(default)]
    pub boundary: Vec<BoundaryEntry>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryEntry {
    pub name: lele_config::BoundaryName,
    pub why: String,
    pub folders: Vec<lele_config::FolderPath>,
    #[serde(default)]
    pub cannot_use: Vec<lele_config::CallPath>,
    #[serde(default)]
    pub require: Option<lele_config::Requirement>,
}

#[derive(Debug, Clone, Deserialize, Default)]
#[serde(deny_unknown_fields)]
pub struct HonestySection {
    #[serde(default)]
    pub declared_honest: Vec<lele_config::CallPath>,
    #[serde(default)]
    pub declared_dishonest: Vec<lele_config::CallPath>,
}
