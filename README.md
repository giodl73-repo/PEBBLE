# Pebble

Pebble is a compact context transfer format for documents, corpora, and AI
agents. The current schema is `pebble.v1`.

**Series:** [Standards & Protocols](https://github.com/giodl73-repo/giodl73-repo/blob/main/series/standards-protocols.md).

**Review roles:** This repo uses
[ROLES](https://github.com/giodl73-repo/ROLES), the `.roles` convention for
repository-local review panels.

Pebbles are not a human publishing target. They are small, provenance-bearing
records that preserve source identity, section boundaries, stable IDs, line
numbers, resolved text, and dependency references.

PROOF emits Pebbles from compiled `.source.md` files. CROP can emit the same
schema for corpus views and slices, so both tools can exchange context without
inventing separate pack formats.

## Pebble v1 standards

- **Schema name:** `pebble.v1`.
- **Encoding:** UTF-8 JSON without a byte-order mark.
- **Canonical wire form:** compact JSON. Pretty JSON is allowed for examples,
  review, and debugging, but should not be used for cache keys, transfer-size
  measurements, or durable generated artifacts.
- **Kinds:** `document` for one source document; tools may use other kind strings
  such as `corpus-slice` when the sections come from a selected corpus view.
- **Format:** `markdown` means section text is Markdown content.
- **Sections:** ordered document/corpus chunks with stable `id`, heading `path`,
  heading `level`, source `line`, and resolved `text`.
- **Refs:** optional dependency/source references used for provenance and graph
  construction.

```json
{
  "schema": "pebble.v1",
  "kind": "document",
  "title": "Guide",
  "source": "guide.source.md",
  "format": "markdown",
  "sections": [
    {
      "id": "guide",
      "path": ["Guide"],
      "level": 1,
      "line": 1,
      "text": "# Guide\n\nResolved Markdown text."
    }
  ],
  "refs": [".proof/side-info/links.json"]
}
```

## Rust

```rust
let pebble = pebble::PebbleDocument::from_markdown(
    "# Guide\n\nBody",
    "fallback title",
    "guide.source.md",
    [".proof/side-info/links.json"],
);
let json = pebble.to_json()?;
```

## Research

- [Performance research](docs/research/performance.md) tracks size, speed, and
  quality options before broader adoption.
- [SLICE selector examples](docs/specs/slice-selectors.md) show dev-only
  metadata selector patterns over Pebble-shaped rows without changing
  `pebble.v1`.

## License

[MIT](LICENSE) — © 2026 Gio Della-Libera.
