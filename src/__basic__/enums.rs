use crate::FailureReason;

#[derive(Debug)]
pub enum Outcome {
    NothingToCheck,
    Clean,
    Failed(FailureReason),
}
