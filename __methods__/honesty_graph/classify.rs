use std::collections::{BTreeMap, VecDeque};

use crate::HonestyGraph;
use crate::NodeId;
use crate::Witness;

pub fn classify(graph: &HonestyGraph) -> BTreeMap<NodeId, Witness> {
    let mut witnesses: BTreeMap<NodeId, Witness> = BTreeMap::new();
    let mut queue: VecDeque<NodeId> = VecDeque::new();

    let mut ordered: Vec<NodeId> = graph.nodes.keys().copied().collect();
    ordered.sort();
    for id in &ordered {
        if let Some(cause) = graph.nodes.get(id).and_then(|n| n.direct.clone()) {
            witnesses.insert(
                *id,
                Witness {
                    chain: vec![*id],
                    cause,
                },
            );
            queue.push_back(*id);
        }
    }

    while let Some(node) = queue.pop_front() {
        let mut callers: Vec<NodeId> = graph
            .reverse
            .get(&node)
            .map(|s| s.iter().copied().collect())
            .unwrap_or_default();
        callers.sort();
        for caller in callers {
            if witnesses.contains_key(&caller) {
                continue;
            }
            let Some(base) = witnesses.get(&node) else {
                continue;
            };
            let mut chain = vec![caller];
            chain.extend(base.chain.iter().copied());
            witnesses.insert(
                caller,
                Witness {
                    chain,
                    cause: base.cause.clone(),
                },
            );
            queue.push_back(caller);
        }
    }

    witnesses
}

#[cfg(test)]
mod tests {
    use super::classify;
    use crate::DirectCause;
    use crate::HonestyGraph;
    use crate::NodeId;

    #[test]
    fn test_usage() {
        let mut graph = HonestyGraph::default();
        graph.add_node(NodeId(0), "a".to_string(), None);
        graph.add_node(NodeId(1), "b".to_string(), None);
        graph.add_edge(NodeId(0), NodeId(1));
        graph.add_edge(NodeId(1), NodeId(0));
        graph.add_node(
            NodeId(2),
            "clock".to_string(),
            Some(DirectCause::IoRoot(
                "std::time::SystemTime::now".to_string(),
            )),
        );
        graph.add_edge(NodeId(1), NodeId(2));
        let witnesses = classify(&graph);
        assert_eq!(witnesses.len(), 3);
        assert_eq!(witnesses.get(&NodeId(0)).map(|x| x.chain.len()), Some(3));
    }

    #[test]
    fn test_usage_witness_shortest() {
        let mut graph = HonestyGraph::default();
        graph.add_node(NodeId(0), "top".to_string(), None);
        graph.add_node(
            NodeId(1),
            "leaf".to_string(),
            Some(DirectCause::HiddenStaticWrite("M".to_string())),
        );
        graph.add_edge(NodeId(0), NodeId(1));
        let witnesses = classify(&graph);
        assert_eq!(
            witnesses.get(&NodeId(0)).map(|x| x.chain.clone()),
            Some(vec![NodeId(0), NodeId(1)])
        );
    }
}
