use std::collections::BTreeMap;

use pebble::PebbleDocument;
use serde_json::{json, Value};
use slice_core::{FieldCatalog, ValueType};

#[test]
fn slice_selects_pebble_documents_by_metadata() {
    let mut ready = PebbleDocument::from_markdown(
        "# Guide\n\nReady proof guidance.",
        "Guide",
        "guide.md",
        [".proof/side-info/links.json"],
    );
    ready.metadata = metadata(&[("status", "ready"), ("tags", "[proof, guide]")]);

    let mut draft = PebbleDocument::from_markdown(
        "# Draft\n\nDraft proof guidance.",
        "Draft",
        "draft.md",
        Vec::<String>::new(),
    );
    draft.metadata = metadata(&[("status", "draft"), ("tags", "[proof]")]);

    let selected = select_document_sources(
        &[ready, draft],
        "metadata.status eq 'ready' and metadata.tags contains 'proof'",
    );

    assert_eq!(selected, ["guide.md"]);
}

#[test]
fn slice_selects_pebble_sections_by_metadata() {
    let mut doc = PebbleDocument::from_markdown(
        "# Guide\n\nIntro.\n\n## Steps\n\nDo the work.",
        "Guide",
        "guide.md",
        Vec::<String>::new(),
    );
    doc.sections[0].metadata = metadata(&[("status", "ready")]);
    doc.sections[1].metadata = metadata(&[("status", "draft")]);

    let selected = select_section_ids(
        &[doc],
        "document.source eq 'guide.md' and section.metadata.status eq 'ready'",
    );

    assert_eq!(selected, ["guide.md#guide"]);
}

fn select_document_sources(docs: &[PebbleDocument], expr: &str) -> Vec<String> {
    let mut catalog = FieldCatalog::new();
    catalog
        .insert("metadata.status", ValueType::String)
        .insert("metadata.tags", ValueType::String);
    let selector = slice_core::compile(expr, &catalog).unwrap();

    docs.iter()
        .filter(|doc| selector.matches(&document_row(doc)))
        .map(|doc| doc.source.clone())
        .collect()
}

fn select_section_ids(docs: &[PebbleDocument], expr: &str) -> Vec<String> {
    let mut catalog = FieldCatalog::new();
    catalog
        .insert("document.source", ValueType::String)
        .insert("section.metadata.status", ValueType::String);
    let selector = slice_core::compile(expr, &catalog).unwrap();

    docs.iter()
        .flat_map(|doc| {
            doc.sections
                .iter()
                .filter(|section| selector.matches(&section_row(doc, section)))
                .map(|section| format!("{}#{}", doc.source, section.id))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn document_row(doc: &PebbleDocument) -> Value {
    json!({
        "schema": doc.schema,
        "kind": doc.kind,
        "source": doc.source,
        "metadata": doc.metadata,
    })
}

fn section_row(doc: &PebbleDocument, section: &pebble::PebbleSection) -> Value {
    json!({
        "document": {
            "source": doc.source,
            "metadata": doc.metadata,
        },
        "section": {
            "id": section.id,
            "metadata": section.metadata,
            "level": section.level,
        },
    })
}

fn metadata(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
    entries
        .iter()
        .map(|(key, value)| ((*key).to_string(), (*value).to_string()))
        .collect()
}
