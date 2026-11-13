use std::path::{Component, Path, PathBuf};

pub fn normalize_components(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if matches!(out.components().next_back(), Some(Component::Normal(_))) {
                    out.pop();
                } else {
                    out.push("..");
                }
            }
            other => out.push(other.as_os_str()),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::normalize_components;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_usage() {
        assert_eq!(
            normalize_components(Path::new("crates/x/src/../__methods__/a.rs")),
            PathBuf::from("crates/x/__methods__/a.rs")
        );
        assert_eq!(
            normalize_components(Path::new("a/./b.rs")),
            PathBuf::from("a/b.rs")
        );
        assert_eq!(
            normalize_components(Path::new("crates/x/src/core.rs")),
            PathBuf::from("crates/x/src/core.rs")
        );
        assert_eq!(
            normalize_components(Path::new("../a/b.rs")),
            PathBuf::from("../a/b.rs")
        );
    }
}
