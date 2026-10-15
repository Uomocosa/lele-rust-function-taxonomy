use lele_rust_function_taxonomy::lele_config;
use lele_rust_function_taxonomy::Diagnostic;
use lele_rust_function_taxonomy::Tax001;
use lele_rust_function_taxonomy::Witness;

use crate::basic::structs::Built;

pub fn render(
    location: &str,
    path_str: &str,
    witness: &Witness,
    built: &Built,
    boundary: &lele_config::BoundaryName,
) -> String {
    let chain: Vec<String> = witness
        .chain
        .iter()
        .map(|id| built.graph.path_of(*id).to_string())
        .collect();
    let hops = chain.join(" -> ");
    format!(
        "{location}: error[{}]: `{path_str}` is dishonest: {hops} ({}); boundary \"{}\" requires honest functions",
        Tax001.code(),
        witness.cause,
        boundary.as_str()
    )
}

// no test_usage necessary
