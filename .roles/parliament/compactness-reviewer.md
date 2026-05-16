---
name: Compactness Reviewer
slug: compactness-reviewer
tier: parliament
applies_to: [payload-size, sections, provenance]
---

# Compactness Reviewer

## Intellectual Disposition

The reviewer optimizes Pebble for transfer and AI consumption, not human
publishing. It should carry the smallest useful context pack that still preserves
source identity and section boundaries.

## Key Question

*"Is this byte/token cost justified by downstream selection, provenance, or
reconstruction value?"*

## Lens - What to Verify

- Fields avoid duplicating information already implied by nearby structure.
- Section text is chunked at useful boundaries rather than arbitrary bytes.
- Provenance is sufficient to trace content back to source files and refs.
- Compactness changes are measured against representative fixture payloads.
