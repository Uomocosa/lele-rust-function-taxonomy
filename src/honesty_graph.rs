use std::collections::{BTreeMap, BTreeSet};

use atomic_delegate_macros::atomic_delegates;

use crate::DirectCause;
use crate::Node;
use crate::NodeId;
use crate::Witness;

#[derive(Debug, Default, Clone)]
pub struct HonestyGraph {
    pub nodes: BTreeMap<NodeId, Node>,
    pub edges: BTreeSet<(NodeId, NodeId)>,
    pub reverse: BTreeMap<NodeId, BTreeSet<NodeId>>,
}

#[rustfmt::skip]
impl HonestyGraph {
    pub fn add_node(&mut self, id: NodeId, path: String, direct: Option<DirectCause>) { self.nodes.entry(id).or_insert(Node { id, path, direct }); }
    pub fn add_edge(&mut self, caller: NodeId, callee: NodeId) { if self.edges.insert((caller, callee)) { self.reverse.entry(callee).or_default().insert(caller); } }
    pub fn callees(&self, caller: NodeId) -> impl Iterator<Item = NodeId> + '_ { self.edges.iter().filter(move |(c, _)| *c == caller).map(|(_, callee)| *callee) }
    pub fn path_of(&self, id: NodeId) -> &str { self.nodes.get(&id).map_or("?", |n| n.path.as_str()) }
}

#[atomic_delegates]
impl HonestyGraph {
    pub fn classify(&self) -> BTreeMap<NodeId, Witness> {}
}

#[cfg(test)]
mod tests {
    use super::HonestyGraph;
    use crate::NodeId;

    #[test]
    fn test_usage() {
        assert_eq!(NodeId(1), NodeId(1));
        let mut graph = HonestyGraph::default();
        graph.add_node(NodeId(0), "a".to_string(), None);
        graph.add_node(NodeId(1), "b".to_string(), None);
        graph.add_edge(NodeId(0), NodeId(1));
        assert_eq!(graph.path_of(NodeId(0)), "a");
        assert_eq!(graph.callees(NodeId(0)).count(), 1);
    }
}
