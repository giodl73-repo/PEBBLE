# Wave: SLICE Examples

## Goal

Show how consumers can apply SLICE selectors to Pebble-shaped document and
section rows without changing `pebble.v1`.

## Pulse table

| Pulse | Title | Status | Outcome |
|------:|-------|--------|---------|
| 01 | Metadata selector examples | done | Added dev-only SLICE tests for document and section metadata selectors. |

## Success criteria

- Pebble remains the schema owner.
- SLICE is only a dev/example dependency.
- Examples cover document metadata and section metadata.
- `cargo test` passes.
