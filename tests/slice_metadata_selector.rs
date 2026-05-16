#![cfg(feature = "slice")]

use pebble::PebbleDocument;

#[test]
fn slice_selects_pebble_documents_by_metadata() {
    let ready = PebbleDocument::from_markdown(
        "---\nstatus: ready\ntags: [proof, guide]\n---\n\n# Guide\n\nReady proof guidance.",
        "Guide",
        "guide.md",
        [".proof/side-info/links.json"],
    );

    let draft = PebbleDocument::from_markdown(
        "---\nstatus: draft\ntags: [proof]\n---\n\n# Draft\n\nDraft proof guidance.",
        "Draft",
        "draft.md",
        Vec::<String>::new(),
    );

    let documents = [ready, draft];
    let selected = PebbleDocument::select_documents(
        &documents,
        "metadata.status eq 'ready' and metadata.tags contains 'proof'",
    )
    .unwrap()
    .into_iter()
    .map(|document| document.source.as_str())
    .collect::<Vec<_>>();

    assert_eq!(selected, ["guide.md"]);
}

#[test]
fn slice_selects_pebble_sections_by_metadata() {
    let doc = PebbleDocument::from_markdown(
        "---\nstatus: ready\n---\n\n# Guide\n\nIntro.\n\n## Steps\n\nDo the work.",
        "Guide",
        "guide.md",
        Vec::<String>::new(),
    );

    let documents = [doc];
    let selected = PebbleDocument::select_sections(
        &documents,
        "document.source eq 'guide.md' and section.metadata.status eq 'ready'",
    )
    .unwrap()
    .into_iter()
    .map(|selected| format!("{}#{}", selected.document.source, selected.section.id))
    .collect::<Vec<_>>();

    assert_eq!(selected, ["guide.md#guide", "guide.md#steps"]);
}
