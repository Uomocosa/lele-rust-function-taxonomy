use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rustc_errors::DiagCtxtHandle;
use rustc_hir::def_id::LocalDefId;
use rustc_middle::ty::TyCtxt;
use rustc_span::Span;

use crate::build_graph;
use crate::crate_function_index::CrateFunctionIndex;

pub struct TaxonomyReport;

#[rustfmt::skip]
impl TaxonomyReport {
    pub fn emit(tcx: TyCtxt<'_>) {
        Self::emit_all(tcx, Self::collect_diagnostics(tcx));
    }

    pub(crate) fn collect_diagnostics(tcx: TyCtxt<'_>) -> Vec<String> {
        let config = build_graph::ReportConfig::from_env();
        let crate_root = Self::crate_root_dir();
        Self::diagnostics_with_config(tcx, &config, &crate_root)
    }

    pub(crate) fn diagnostics_with_config(
        tcx: TyCtxt<'_>,
        config: &build_graph::ReportConfig,
        crate_root: &Path,
    ) -> Vec<String> {
        let crate_functions = CrateFunctionIndex::from_crate(tcx, config);
        let witnesses = crate_functions.graph.classify();
        Self::ordered_paths(&crate_functions)
            .into_iter()
            .filter_map(|(path_str, def_id)| {
                Self::diagnostic_for(tcx, config, crate_root, &crate_functions, &witnesses, path_str, def_id)
            })
            .collect()
    }

    pub(crate) fn ordered_paths(
        crate_functions: &CrateFunctionIndex,
    ) -> Vec<(String, LocalDefId)> {
        let mut ordered: Vec<(String, LocalDefId)> = crate_functions
            .paths
            .iter()
            .map(|(id, path)| (path.clone(), *id))
            .collect();
        ordered.sort_by(|(left, _), (right, _)| left.cmp(right));
        ordered
    }

    pub(crate) fn diagnostic_for(
        tcx: TyCtxt<'_>,
        config: &build_graph::ReportConfig,
        crate_root: &Path,
        crate_functions: &CrateFunctionIndex,
        witnesses: &BTreeMap<lele_rust_function_taxonomy::NodeId, lele_rust_function_taxonomy::Witness>,
        path_str: String,
        def_id: LocalDefId,
    ) -> Option<String> {
        let (location, witness, boundary) = Self::diagnostic_parts(
            tcx,
            config,
            crate_root,
            crate_functions,
            witnesses,
            def_id,
        )?;
        Some(
            build_graph::finding::Finding {
                location,
                path_str,
                witness,
                crate_functions,
                boundary,
            }
            .render(),
        )
    }

    pub(crate) fn diagnostic_parts<'a>(
        tcx: TyCtxt<'_>,
        config: &'a build_graph::ReportConfig,
        crate_root: &Path,
        crate_functions: &CrateFunctionIndex,
        witnesses: &'a BTreeMap<lele_rust_function_taxonomy::NodeId, lele_rust_function_taxonomy::Witness>,
        def_id: LocalDefId,
    ) -> Option<(String, &'a lele_rust_function_taxonomy::Witness, &'a lele_rust_function_taxonomy::lele_config::BoundaryName)> {
        let (span, rel, boundary) =
            Self::span_rel_boundary(tcx, config, crate_root, crate_functions, def_id)?;
        let (location, witness) = Self::witness_and_location(
            tcx,
            crate_functions,
            witnesses,
            def_id,
            span,
            &rel,
        )?;
        Some((location, witness, boundary))
    }

    pub(crate) fn span_rel_boundary<'a>(
        tcx: TyCtxt<'_>,
        config: &'a build_graph::ReportConfig,
        crate_root: &Path,
        crate_functions: &CrateFunctionIndex,
        def_id: LocalDefId,
    ) -> Option<(Span, PathBuf, &'a lele_rust_function_taxonomy::lele_config::BoundaryName)> {
        let (span, rel) = Self::span_and_rel(tcx, crate_root, crate_functions, def_id)?;
        let boundary = Self::boundary_of_rel(config, &rel)?;
        Some((span, rel, boundary))
    }

    pub(crate) fn span_and_rel(
        tcx: TyCtxt<'_>,
        crate_root: &Path,
        crate_functions: &CrateFunctionIndex,
        def_id: LocalDefId,
    ) -> Option<(Span, PathBuf)> {
        let span = Self::span_for(crate_functions, def_id)?;
        let rel = Self::rel_for(tcx, crate_root, span)?;
        Some((span, rel))
    }

    pub(crate) fn witness_and_location<'a>(
        tcx: TyCtxt<'_>,
        crate_functions: &CrateFunctionIndex,
        witnesses: &'a BTreeMap<lele_rust_function_taxonomy::NodeId, lele_rust_function_taxonomy::Witness>,
        def_id: LocalDefId,
        span: Span,
        rel: &Path,
    ) -> Option<(String, &'a lele_rust_function_taxonomy::Witness)> {
        let witness = Self::witness_for(crate_functions, witnesses, def_id)?;
        let location = Self::location_for(tcx, span, rel);
        Some((location, witness))
    }

    pub(crate) fn span_for(
        crate_functions: &CrateFunctionIndex,
        def_id: LocalDefId,
    ) -> Option<Span> {
        crate_functions.spans.get(&def_id).copied()
    }

    pub(crate) fn rel_for(tcx: TyCtxt<'_>, crate_root: &Path, span: Span) -> Option<PathBuf> {
        Self::span_file(tcx, span, crate_root)
    }

    pub(crate) fn boundary_of_rel<'a>(
        config: &'a build_graph::ReportConfig,
        rel: &Path,
    ) -> Option<&'a lele_rust_function_taxonomy::lele_config::BoundaryName> {
        config.boundary_for(rel)
    }

    pub(crate) fn witness_for<'a>(
        crate_functions: &CrateFunctionIndex,
        witnesses: &'a BTreeMap<lele_rust_function_taxonomy::NodeId, lele_rust_function_taxonomy::Witness>,
        def_id: LocalDefId,
    ) -> Option<&'a lele_rust_function_taxonomy::Witness> {
        let node = crate_functions.node_ids.get(&def_id).copied()?;
        witnesses.get(&node)
    }

    pub(crate) fn location_for(tcx: TyCtxt<'_>, span: Span, rel: &Path) -> String {
        let loc = tcx.sess.source_map().lookup_char_pos(span.lo());
        format!(
            "{}:{}:{}",
            rel.display(),
            loc.line,
            loc.col_display.saturating_add(1)
        )
    }

    pub(crate) fn emit_all(tcx: TyCtxt<'_>, diagnostics: Vec<String>) {
        if diagnostics.is_empty() {
            return;
        }
        let dcx: DiagCtxtHandle<'_> = tcx.dcx();
        for message in diagnostics {
            dcx.struct_err(message).emit();
        }
    }

    pub(crate) fn span_file(tcx: TyCtxt<'_>, span: Span, crate_root: &Path) -> Option<PathBuf> {
        let file = tcx.sess.source_map().lookup_source_file(span.source_callsite().lo());
        let path = file.name.clone().into_local_path()?;
        Some(path.strip_prefix(crate_root).unwrap_or(&path).to_path_buf())
    }

    pub(crate) fn crate_root_dir() -> PathBuf {
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    }
}

// no test_usage necessary
