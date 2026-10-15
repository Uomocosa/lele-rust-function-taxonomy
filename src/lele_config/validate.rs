use std::path::Path;

use crate::lele_config;

pub fn validate(
    manifest_path: &Path,
    config: &lele_config::Config,
) -> Result<(), lele_config::LeleTomlError> {
    let root = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let file = lele_config::ConfigSource(lele_config::display_path(manifest_path));
    for boundary in &config.boundaries {
        if boundary.folders.is_empty() {
            return Err(lele_config::LeleTomlError::Invalid {
                file: file.clone(),
                reason: lele_config::ErrorReason(format!(
                    "boundary `{}` has no folders",
                    boundary.name.as_str()
                )),
            });
        }
        for folder in &boundary.folders {
            if !root.join(folder.as_path()).is_dir() {
                return Err(lele_config::LeleTomlError::Invalid {
                    file: file.clone(),
                    reason: lele_config::ErrorReason(format!(
                        "boundary `{}` names folder `{}` which does not exist",
                        boundary.name.as_str(),
                        folder.display()
                    )),
                });
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate;
    use crate::lele_config;

    #[test]
    fn test_usage() {
        let tmp = tempfile::tempdir().unwrap();
        let manifest = tmp.path().join("Cargo.toml");
        std::fs::write(&manifest, "").unwrap();
        std::fs::write(
            tmp.path().join("lele.toml"),
            "[[lele.boundary]]\nname = \"b\"\nwhy = \"w\"\nfolders = [\"src/nope\"]\nrequire = \"honest\"\n",
        )
        .unwrap();
        let loaded = lele_config::load(&manifest).unwrap();
        assert!(validate(&manifest, &loaded).is_err());
    }
}
