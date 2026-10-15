use crate::Diagnostic;

pub struct Tax001;

#[rustfmt::skip]
impl Diagnostic for Tax001 {
    fn code(&self) -> &'static str {
        "TAX001"
    }

    fn summary(&self) -> &'static str {
        "a function inside a require=honest boundary reaches hidden I/O"
    }
}

// no test_usage necessary
