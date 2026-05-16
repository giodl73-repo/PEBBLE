# Pebble

Pebble is a compact context transfer format for documents, corpora, and AI
agents. The current schema is `pebble.v1`.

Pebbles are not a human publishing target. They are small, provenance-bearing
records that preserve source identity, section boundaries, stable IDs, line
numbers, resolved text, and dependency references.

PROOF emits Pebbles from compiled `.source.md` files. CROP can emit the same
schema for corpus views and slices, so both tools can exchange context without
inventing separate pack formats.

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
