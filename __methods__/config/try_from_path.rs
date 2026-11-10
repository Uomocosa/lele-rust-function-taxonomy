use std::path::Path;

use crate::lele_config;

pub fn try_from_path(
    manifest_path: &Path,
) -> Result<lele_config::Config, lele_config::LeleTomlError> {
    let manifest = lele_config::Manifest::from(manifest_path);
    let path = manifest.lele_toml_path();
    let Ok(content) = std::fs::read_to_string(&path) else {
        return Ok(lele_config::Config::default());
    };
    let file = manifest.config_source();
    let parsed: lele_config::LeleToml = toml::from_str(&content)
        .map_err(|toml_error| lele_config::LeleTomlError::InvalidToml { file, toml_error })?;
    Ok(lele_config::Config {
        boundaries: parsed.lele.boundary,
        declared_honest: parsed.honesty.declared_honest,
        declared_dishonest: parsed.honesty.declared_dishonest,
    })
}

#[cfg(test)]
mod tests {
    use super::try_from_path;
    use crate::lele_config;

    #[test]
    fn test_usage() {
        let tmp = tempfile::tempdir().unwrap();
        let manifest = tmp.path().join("Cargo.toml");
        std::fs::write(&manifest, "").unwrap();
        std::fs::write(
            tmp.path().join("lele.toml"),
            "[honesty]\nhonesty_depth = 1\n",
        )
        .unwrap();
        assert!(lele_config::Config::try_from(manifest.as_path()).is_err());
        let _ = try_from_path;
    }
}
