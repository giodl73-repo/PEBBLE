# Pebble performance research

## Research question

How should `pebble.v1` improve transfer size, CROP runtime, and context quality
before broader adoption by repos beyond PROOF and CROP?

Decision supported: keep `pebble.v1` stable as compact JSON for first adoption,
then prototype bundle and section-link improvements behind explicit compatibility
boundaries.

## Local evidence

**PEBBLE-01 — Canonical form is compact JSON.**

- Source: `README.md`, "Pebble v1 standards".
- Behavior: canonical wire/artifact/cache form is UTF-8 compact JSON without BOM;
  pretty JSON is only for examples, review, and debugging.
- Implication: all size benchmarks and generated fixtures should use compact JSON.
- Confidence: high.

**PEBBLE-02 — Current document shape repeats metadata per file.**

- Source: `src/lib.rs`, `PebbleDocument` and `PebbleSection`.
- Behavior: every file repeats `schema`, `kind`, `format`, source metadata, and
  section object keys.
- Implication: single-document Pebbles are simple and portable, but many-file
  corpora pay repeated key overhead. A future bundle form can hoist common values
  and keep `document` unchanged.
- Confidence: high.

**PEBBLE-03 — CROP runs faster over Pebble because the graph is smaller.**

- Source command:
  `.\examples\bench-fixtures\generate.ps1 -Size all -Force; .\examples\bench-fixtures\run.ps1 -Size all -Runs 3`
  in `C:\src\crop`.
- Result snapshot after compact fixture generation:

| Size | Mode | Bytes | Units | Edges | Selected tokens | Quality | Avg total |
|---|---:|---:|---:|---:|---:|---:|---:|
| small | md | 5,752 | 72 | 252 | 792 | 0.540741 | 11.333ms |
| small | pebble | 9,216 | 32 | 102 | 688 | 0.700000 | 4.333ms |
| medium | md | 45,056 | 576 | 2,132 | 1,679 | 0.526125 | 68.000ms |
| medium | pebble | 72,960 | 256 | 872 | 1,458 | 0.676488 | 28.000ms |
| large | md | 179,552 | 2,304 | 8,528 | 1,679 | 0.526125 | 143.667ms |
| large | pebble | 291,072 | 1,024 | 3,488 | 1,458 | 0.676488 | 74.333ms |

- Implication: Pebble improves runtime and benchmark quality by giving CROP fewer,
  cleaner section units, even while current JSON bytes are larger than Markdown.
- Confidence: medium. The generated fixture is useful for regression, but real
  MAXIM-scale corpora should be measured before broad claims.

**PEBBLE-04 — Quality gains come from section boundaries, not compression.**

- Source: `src/lib.rs`, `markdown_sections()` and CROP benchmark fixture output.
- Behavior: Pebble sections preserve heading paths and stable IDs; CROP does not
  need to rediscover paragraphs/frontmatter from raw Markdown.
- Implication: quality improvements should focus on section granularity, per
  section provenance, and graph edges before changing serialization format.
- Confidence: high.

## External format notes

**PEBBLE-05 — JSON Lines is attractive for multi-document streaming, not for a
single canonical document.**

- Source: <https://jsonlines.org/>.
- Behavior: JSON Lines requires UTF-8, one valid JSON value per line, and `\n`
  terminators; it is convenient for processing records one at a time.
- Implication: a future `pebble.bundle.v1` could use JSONL for many documents or
  sections, while `pebble.v1` remains one compact JSON document.
- Confidence: medium; JSON Lines is practical but not an IETF-standardized media
  type according to the source.

**PEBBLE-06 — CBOR and MessagePack can reduce wire size, but would raise adoption
cost.**

- Sources: RFC 8949 CBOR abstract; <https://msgpack.org/>.
- Behavior: CBOR targets small code size, small messages, and extensibility;
  MessagePack advertises JSON-like cross-language exchange that is faster and
  smaller.
- Implication: binary encodings are good candidates for a later transport profile,
  not the first public standard. Compact JSON keeps the schema inspectable and
  easy to adopt.
- Confidence: medium.

**PEBBLE-07 — Compression belongs at transport/package boundaries.**

- Sources: <https://facebook.github.io/zstd/> and <https://jsonlines.org/>.
- Behavior: Zstandard is a fast compression algorithm with high compression
  ratios; JSON Lines recommends stream compressors such as gzip or bzip2 for
  saved space.
- Implication: support `.pebble.json.zst` or similar as a packaging convention,
  but keep canonical `pebble.v1` bytes uncompressed compact JSON for hashing,
  tests, and simple readers.
- Confidence: medium.

## Recommendations

### Adopt now

1. Keep canonical `pebble.v1` as compact UTF-8 JSON without BOM.
2. Measure compact Pebbles only; do not use pretty JSON for fixture or transfer
   benchmarks.
3. Keep section IDs, heading paths, source lines, and refs mandatory/first-class
   because they explain the current quality improvement.

### Prototype behind a compatibility boundary

1. Add a bundle experiment, likely `pebble.bundle.v1`, that hoists `schema`,
   `kind`, `format`, and repeated key names across many documents.
2. Add section-level refs or links so CROP can connect the exact section that
   references another source instead of treating all document refs as equally
   attached to every section.
3. Add a fixture profile that varies chunk granularity: heading-level, paragraph
   level, and hybrid. This should optimize quality without changing the wire
   format first.
4. Compare compact JSON, JSONL bundle, CBOR, MessagePack, gzip, and zstd on
   generated and real corpora, reporting bytes, parse time, CROP ingest time, and
   selected-context quality.

### Defer or reject for now

1. Do not replace `pebble.v1` with binary-only encoding before PROOF/CROP/MAXIM
   adoption. It would make debugging and third-party adoption harder.
2. Do not alias field names in `pebble.v1`; short keys reduce bytes but weaken
   the public contract. Test bundle/key-table approaches first.
3. Do not bake CROP relevance scores into Pebble. Scores are task-specific and
   belong in CROP outputs, not the portable source/context schema.

## Next validation

1. Generate compact fixtures in CROP and record output:
   `.\examples\bench-fixtures\run.ps1 -Size all -Runs 10`.
2. Add a real MAXIM slice fixture once the MAXIM backfill path emits Pebbles.
3. Prototype bundle output in the Pebble crate without changing
   `PebbleDocument::to_json()`.
