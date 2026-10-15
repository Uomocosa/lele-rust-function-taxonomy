use std::collections::HashMap;

use lele_rust_function_taxonomy::HonestyGraph;
use lele_rust_function_taxonomy::NodeId;
use rustc_hir::def_id::LocalDefId;
use rustc_span::Span;

pub struct Built {
    pub graph: HonestyGraph,
    pub paths: HashMap<LocalDefId, String>,
    pub node_ids: HashMap<LocalDefId, NodeId>,
    pub spans: HashMap<LocalDefId, Span>,
}
