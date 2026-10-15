pub trait Diagnostic {
    fn code(&self) -> &'static str;
    fn summary(&self) -> &'static str;
}

// no test_usage necessary
