use crate::lele_config;

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub boundaries: Vec<lele_config::BoundaryEntry>,
    pub declared_honest: Vec<lele_config::CallPath>,
    pub declared_dishonest: Vec<lele_config::CallPath>,
}

#[rustfmt::skip]
impl Config {
    pub fn honest_boundaries(&self) -> impl Iterator<Item = &lele_config::BoundaryEntry> {
        self.boundaries
            .iter()
            .filter(|b| b.require == Some(lele_config::Requirement::Honest))
    }

    pub fn has_honest_boundary(&self) -> bool {
        self.honest_boundaries().next().is_some()
    }
}

#[cfg(test)]
mod tests {
    use crate::lele_config;

    #[test]
    fn test_usage() {
        let tmp = tempfile::tempdir().unwrap();
        let manifest = tmp.path().join("Cargo.toml");
        std::fs::write(&manifest, "").unwrap();
        std::fs::write(
            tmp.path().join("lele.toml"),
            "[[lele.boundary]]\nname = \"b\"\nwhy = \"w\"\nfolders = [\"src/core\"]\nrequire = \"honest\"\n",
        )
        .unwrap();
        std::fs::create_dir_all(tmp.path().join("src/core")).unwrap();
        let loaded = lele_config::load(&manifest).unwrap();
        assert!(loaded.has_honest_boundary());
    }
}
