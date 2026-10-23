#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_span;

use std::path::Path;

use derive_more::Deref;
use lele_rust_function_taxonomy::constants;
use lele_rust_function_taxonomy::lele_config;
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::ty::TyCtxt;

mod build_graph;
mod crate_function_index;
mod roots;

use build_graph::ReportConfig;

#[derive(Deref)]
struct ShouldReport(bool);

impl Callbacks for ShouldReport {
    fn after_analysis(&mut self, _compiler: &interface::Compiler, tcx: TyCtxt<'_>) -> Compilation {
        if **self {
            build_graph::TaxonomyReport::emit(tcx);
        }
        Compilation::Continue
    }
}

fn read_config() -> ReportConfig {
    let config = std::env::var_os(constants::ENV_MANIFEST)
        .and_then(|manifest| lele_config::Config::try_from(Path::new(&manifest)).ok())
        .unwrap_or_default();
    ReportConfig {
        honest_boundaries: config.honest_boundaries().cloned().collect(),
        declared_honest: config.declared_honest,
        declared_dishonest: config.declared_dishonest,
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let primary = std::env::var_os("CARGO_PRIMARY_PACKAGE").is_some();
    let report = primary && std::env::var_os(constants::ENV_MANIFEST).is_some();

    let Some(real_rustc) = args.get(1) else {
        std::process::exit(1);
    };
    let rustc_args = args.get(2..).unwrap_or_default();
    let runner_args = args.get(1..).unwrap_or_default();

    if !primary {
        let code = std::process::Command::new(real_rustc)
            .args(rustc_args)
            .status()
            .ok()
            .and_then(|s| s.code())
            .unwrap_or(1);
        std::process::exit(code);
    }

    let mut callbacks = ShouldReport(report);
    let code = rustc_driver::catch_with_exit_code(|| {
        rustc_driver::run_compiler(runner_args, &mut callbacks);
    });
    if code == std::process::ExitCode::SUCCESS {
        std::process::exit(0);
    }
    std::process::exit(1);
}
