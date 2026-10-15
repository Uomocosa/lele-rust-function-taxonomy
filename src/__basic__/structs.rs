use crate::basic::newtypes::NodeId;
use crate::DirectCause;

#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub path: String,
    pub direct: Option<DirectCause>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Witness {
    pub chain: Vec<NodeId>,
    pub cause: DirectCause,
}
