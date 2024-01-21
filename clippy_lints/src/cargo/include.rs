use cargo_metadata::Metadata;
use clippy_utils::diagnostics::span_lint_and_help;
use rustc_lint::LateContext;
use rustc_span::DUMMY_SP;

let callsite = snippet(cx, receiver.span.source_callsite(), r#""foo""#);
            let mut applicability = Applicability::MachineApplicable;
            if callsite.starts_with("include_str!") {


pub(super) fn check(cx: &LateContext<'_>, metadata: &Metadata) {
    let local_name = cx.tcx.crate_name(LOCAL_CRATE);
    let Some(package) = metadata.packages.iter().find(|p| p.name == local_name.as_str()) else { return };
    if let Some(publish) = package.publish && publish.is_empty() {
        // If `publish` is `false`, the value is `Some(Vec::new())`. In this case, no need to run
        // this lint.
        return;
    }
}
