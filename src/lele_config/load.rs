use std::path::Path;

use crate::lele_config;

// needed helper: keep only the actionable first line of a toml error
fn first_line(message: &str) -> String {
    message
        .lines()
        .find(|l| !l.trim().is_empty())
        .unwrap_or(message)
        .trim()
        .to_string()
}

pub fn load(manifest_path: &Path) -> Result<lele_config::Config, lele_config::LeleTomlError> {
    let path = lele_config::lele_toml_path(manifest_path);
    let Ok(content) = std::fs::read_to_string(&path) else {
        return Ok(lele_config::Config::default());
    };
    let file = lele_config::ConfigSource(lele_config::display_path(manifest_path));
    let parsed: lele_config::LeleToml =
        toml::from_str(&content).map_err(|e| lele_config::LeleTomlError::Invalid {
            file,
            reason: lele_config::ErrorReason(first_line(&e.to_string())),
        })?;
    Ok(lele_config::Config {
        boundaries: parsed.lele.boundary,
        declared_honest: parsed.honesty.declared_honest,
        declared_dishonest: parsed.honesty.declared_dishonest,
    })
}

#[cfg(test)]
mod tests {
    use super::load;

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
        assert!(load(&manifest).is_err());
    }
}
