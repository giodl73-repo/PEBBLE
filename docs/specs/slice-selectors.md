# SLICE selector examples

Pebble does not own a selector language. Consumers that need portable filtering
can project `pebble.v1` documents or sections into rows and evaluate selectors
with SLICE.

## Boundary

- Pebble owns the `pebble.v1` schema, document/section chunking, metadata maps,
  refs, and JSON serialization.
- SLICE owns parsing, typed field catalogs, diagnostics, requirements, and row
  predicate evaluation.
- Adapters own projection choices. For example, Pebble metadata values are
  strings; an adapter may query `metadata.tags contains 'proof'` directly or
  project tag-like strings into arrays before using `has`.

## Examples

Document metadata:

```text
metadata.status eq 'ready' and metadata.tags contains 'proof'
```

Section metadata:

```text
document.source eq 'guide.md' and section.metadata.status eq 'ready'
```

The checked test `tests/slice_metadata_selector.rs` demonstrates both patterns
without adding a runtime dependency from Pebble to SLICE.
