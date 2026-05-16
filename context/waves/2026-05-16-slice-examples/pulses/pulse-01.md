# Pulse 01: Metadata selector examples

## Goal

Prove PEBBLE can use SLICE selectors over metadata-shaped rows without taking a
runtime dependency or adding selector semantics to the Pebble schema.

## Changes

- Add a dev-only `slice-core` dependency.
- Add integration tests for document metadata selection.
- Add integration tests for section metadata selection.
- Document the adapter boundary in `docs/specs/slice-selectors.md`.

## Validation

- `cargo test`
- `git diff --check`

## Status

Done.
