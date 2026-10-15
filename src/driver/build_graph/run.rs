use std::path::{Path, PathBuf};

use rustc_errors::DiagCtxtHandle;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use crate::build_graph;
use crate::read_config;

pub fn run(tcx: TyCtxt<'_>) {
    let config = read_config();
    let crate_root = crate_root_dir();
    let built = build_graph::graph::graph(tcx, &config);

    let witnesses = built.graph.classify();

    let mut diagnostics: Vec<String> = Vec::new();

    let mut ordered: Vec<(String, LocalDefId)> = built
        .paths
        .iter()
        .map(|(id, path)| (path.clone(), *id))
        .collect();
    ordered.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (path_str, def_id) in ordered {
        let Some(span) = built.spans.get(&def_id).copied() else {
            continue;
        };
        let Some(rel) = span_file(tcx, span, &crate_root) else {
            continue;
        };
        let Some(boundary) = config.boundary_for(&rel) else {
            continue;
        };
        let node = built.node_ids.get(&def_id).copied();
        let Some(node) = node else {
            continue;
        };
        let Some(witness) = witnesses.get(&node) else {
            continue;
        };
        let loc = tcx.sess.source_map().lookup_char_pos(span.lo());
        let location = format!(
            "{}:{}:{}",
            rel.display(),
            loc.line,
            loc.col_display.saturating_add(1)
        );
        diagnostics.push(build_graph::render::render(
            &location, &path_str, witness, &built, boundary,
        ));
    }

    if diagnostics.is_empty() {
        return;
    }
    let dcx: DiagCtxtHandle<'_> = tcx.dcx();
    for message in diagnostics {
        dcx.struct_err(message).emit();
    }
}

// needed helper: crate-relative source path of a span
fn span_file(tcx: TyCtxt<'_>, span: Span, crate_root: &Path) -> Option<PathBuf> {
    let source_map = tcx.sess.source_map();
    let file = source_map.lookup_source_file(span.source_callsite().lo());
    let path = file.name.clone().into_local_path()?;
    let rel = path.strip_prefix(crate_root).unwrap_or(&path);
    Some(rel.to_path_buf())
}

fn crate_root_dir() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

// no test_usage necessary
