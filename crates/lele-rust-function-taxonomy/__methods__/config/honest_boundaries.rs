use crate::lele_config;

pub fn honest_boundaries(
    config: &lele_config::Config,
) -> impl Iterator<Item = &lele_config::BoundaryEntry> {
    config
        .boundaries
        .iter()
        .filter(|b| b.require == Some(lele_config::Requirement::Honest))
}

#[cfg(test)]
mod tests {
    use super::honest_boundaries;
    use crate::lele_config;

    #[test]
    fn test_usage() {
        let tmp = tempfile::tempdir().unwrap();
        let manifest = tmp.path().join("Cargo.toml");
        std::fs::write(&manifest, "").unwrap();
        std::fs::write(
            tmp.path().join("lele.toml"),
            "[[lele.boundary]]\nname = \"core\"\nwhy = \"w\"\nfolders = [\"src/core\"]\nrequire = \"honest\"\n\n[[lele.boundary]]\nname = \"plain\"\nwhy = \"w\"\nfolders = [\"src/plain\"]\n",
        )
        .unwrap();
        let config = lele_config::Config::try_from(manifest.as_path()).unwrap();
        let names: Vec<String> = honest_boundaries(&config)
            .map(|b| b.name.as_str().to_string())
            .collect();
        assert_eq!(names, vec!["core".to_string()]);
    }
}
