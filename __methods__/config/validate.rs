use std::path::Path;

use crate::lele_config;

pub fn validate(
    config: &lele_config::Config,
    manifest_path: &Path,
) -> Result<(), lele_config::LeleTomlError> {
    let manifest = lele_config::Manifest::from(manifest_path);
    let root = manifest.parent().unwrap_or_else(|| Path::new("."));
    let file = manifest.config_source();
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
        let loaded = lele_config::Config::try_from(manifest.as_path()).unwrap();
        assert!(validate(&loaded, manifest.as_path()).is_err());
    }
}
