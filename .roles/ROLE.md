# Pebble Role Index

Pebble defines the `pebble.v1` compact context-transfer schema and Rust crate.
Use these roles when changing schema fields, sectioning behavior, serialization,
or PROOF/CROP integration examples.

## Parliament

| File | Role | Primary tension |
|---|---|---|
| `parliament/schema-steward.md` | Schema Steward | Stable interchange contract vs. useful evolution |
| `parliament/compactness-reviewer.md` | Compactness Reviewer | Transfer size vs. sufficient provenance |
| `parliament/integration-reviewer.md` | Integration Reviewer | Standalone standard vs. PROOF/CROP assumptions |

## Review order

1. Use Schema Steward for field names, versioning, and compatibility.
2. Use Compactness Reviewer for payload shape, duplication, and token footprint.
3. Use Integration Reviewer for README examples and cross-tool behavior.
