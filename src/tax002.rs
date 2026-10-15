use crate::Diagnostic;

pub struct Tax002;

#[rustfmt::skip]
impl Diagnostic for Tax002 {
    fn code(&self) -> &'static str {
        "TAX002"
    }

    fn summary(&self) -> &'static str {
        "the taxonomy config or toolchain could not be used"
    }
}

// no test_usage necessary
