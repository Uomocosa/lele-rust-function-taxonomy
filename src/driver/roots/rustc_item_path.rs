use derive_more::Deref;
use lele_rust_function_taxonomy::lele_config;

use crate::roots;

#[derive(Deref)]
pub struct RustcItemPath(pub String);

#[rustfmt::skip]
impl RustcItemPath {
    pub fn is_logging_type(&self) -> bool { self.starts_with("tracing_core::callsite::") || self.starts_with("tracing::") || self.starts_with("log::") }

    pub fn is_declared_by(&self, list: &[lele_config::CallPath]) -> bool { list.iter().any(|declared| self.as_str() == declared.as_str() || self.ends_with(declared.as_str())) }

    pub fn is_io_root(&self, extra: &[String]) -> Option<String> {
        let mut candidates = self_types(self);
        candidates.push(normalize(self));
        first_matching_root(&candidates, extra)
    }
}

// needed helper: strip `::<...>` turbofish so `LocalKey::<T>::with` matches `LocalKey::with`
fn normalize(path: &str) -> String {
    let mut out = String::with_capacity(path.len());
    let mut depth: u32 = 0;
    for ch in path.chars() {
        match ch {
            '<' => {
                depth = depth.saturating_add(1);
                if out.ends_with("::") {
                    out.truncate(out.len().saturating_sub(2));
                }
            }
            '>' => depth = depth.saturating_sub(1),
            _ if depth == 0 => out.push(ch),
            _ => {}
        }
    }
    out
}

// needed helper: trait-impl paths print `<Self as Trait>::method`; also test the Self type
fn self_types(path: &str) -> Vec<String> {
    let Some(rest) = path.strip_prefix('<') else {
        return Vec::new();
    };
    let Some((ty, _)) = rest.split_once(" as ") else {
        return Vec::new();
    };
    vec![format!("{ty}::")]
}

// needed helper: first configured I/O root matching any candidate spelling
fn first_matching_root(candidates: &[String], extra: &[String]) -> Option<String> {
    for candidate in candidates {
        for root in roots::DEFAULT_ROOTS {
            if matches_one(candidate, root) {
                return Some((*root).to_string());
            }
        }
        for root in extra {
            let root_norm = normalize(root);
            if matches_one(candidate, &root_norm) {
                return Some(root.clone());
            }
        }
    }
    None
}

fn matches_one(path: &str, root: &str) -> bool {
    if root.ends_with("::") {
        path.starts_with(root) || path.contains(root)
    } else {
        path == root || path.starts_with(&format!("{root}::"))
    }
}

#[cfg(test)]
mod tests {
    use super::{normalize, RustcItemPath};
    use lele_rust_function_taxonomy::lele_config;

    #[test]
    fn test_usage() {
        assert_eq!(
            normalize("std::thread::LocalKey::<T>::with"),
            "std::thread::LocalKey::with"
        );
        let clock = RustcItemPath("std::time::SystemTime::now".to_string());
        assert!(clock.is_io_root(&[]).is_some());
        let logging = RustcItemPath("tracing::foo::Bar".to_string());
        assert!(logging.is_logging_type());
        let declared = [lele_config::CallPath("std::fs::read".to_string())];
        let read = RustcItemPath("my_crate::std::fs::read".to_string());
        assert!(read.is_declared_by(&declared));
    }
}
