use crate::HonestyGraph;
use crate::NodeId;

pub fn callees(graph: &HonestyGraph, caller: NodeId) -> impl Iterator<Item = NodeId> + '_ {
    graph
        .edges
        .iter()
        .filter(move |(c, _)| *c == caller)
        .map(|(_, callee)| *callee)
}

#[cfg(test)]
mod tests {
    use super::callees;
    use crate::HonestyGraph;
    use crate::NodeId;

    #[test]
    fn test_usage() {
        let mut graph = HonestyGraph::default();
        graph.add_edge(NodeId(0), NodeId(1));
        graph.add_edge(NodeId(2), NodeId(1));
        let found: Vec<NodeId> = callees(&graph, NodeId(0)).collect();
        assert_eq!(found, vec![NodeId(1)]);
    }
}
