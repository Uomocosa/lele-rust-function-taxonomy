use lele_rust_function_taxonomy::lele_config;
use lele_rust_function_taxonomy::Diagnostic;
use lele_rust_function_taxonomy::Tax001;
use lele_rust_function_taxonomy::Witness;

use crate::crate_function_index::CrateFunctionIndex;

pub struct Finding<'a> {
    pub location: String,
    pub path_str: String,
    pub witness: &'a Witness,
    pub crate_functions: &'a CrateFunctionIndex,
    pub boundary: &'a lele_config::BoundaryName,
}

impl Finding<'_> {
    pub fn render(&self) -> String {
        let hops = self.hops();
        format!(
            "{}: error[{}]: `{}` is dishonest: {hops} ({}); boundary \"{}\" requires honest functions",
            self.location,
            Tax001.code(),
            self.path_str,
            self.witness.cause,
            self.boundary.as_str()
        )
    }

    pub(crate) fn hops(&self) -> String {
        let chain: Vec<String> = self
            .witness
            .chain
            .iter()
            .map(|id| self.crate_functions.graph.path_of(*id).to_string())
            .collect();
        chain.join(" -> ")
    }
}

// no test_usage necessary
