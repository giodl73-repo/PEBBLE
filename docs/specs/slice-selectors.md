# SLICE selector examples

Pebble does not own a selector language. Consumers that need portable filtering
can enable Pebble's optional `slice` feature to project `pebble.v1` documents or
sections into rows and evaluate selectors with SLICE.

## Boundary

- Pebble owns the `pebble.v1` schema, document/section chunking, metadata maps,
  refs, and JSON serialization.
- SLICE owns parsing, typed field catalogs, diagnostics, requirements, and row
  predicate evaluation.
- Pebble's optional helper owns only a simple built-in projection. Consumers that
  need custom tag arrays or domain-specific fields should still project their own
  rows before using SLICE.

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
with the optional `slice` feature enabled.
