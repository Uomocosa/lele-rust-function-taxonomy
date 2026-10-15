#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DirectCause {
    IoRoot(String),
    HiddenStaticRead(String),
    HiddenStaticWrite(String),
    ThreadLocal(String),
    DeclaredDishonest(String),
}

#[rustfmt::skip]
impl std::fmt::Display for DirectCause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoRoot(path) => write!(f, "calls `{path}` (hidden read/write)"),
            Self::HiddenStaticRead(name) => write!(f, "reads non-Freeze static `{name}`"),
            Self::HiddenStaticWrite(name) => write!(f, "writes `static mut {name}`"),
            Self::ThreadLocal(name) => write!(f, "accesses thread-local `{name}`"),
            Self::DeclaredDishonest(name) => write!(f, "`{name}` is declared dishonest"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DirectCause;

    #[test]
    fn test_usage() {
        let cause = DirectCause::IoRoot("std::fs::read".to_string());
        assert_eq!(
            cause.to_string(),
            "calls `std::fs::read` (hidden read/write)"
        );
        let cause = DirectCause::DeclaredDishonest("my_crate::f".to_string());
        assert_eq!(cause.to_string(), "`my_crate::f` is declared dishonest");
    }
}
