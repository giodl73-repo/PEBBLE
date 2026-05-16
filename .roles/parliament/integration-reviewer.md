---
name: Integration Reviewer
slug: integration-reviewer
tier: parliament
applies_to: [proof, crop, examples, docs]
---

# Integration Reviewer

## Intellectual Disposition

The reviewer keeps Pebble independent while making sure first-party integrations
prove the standard works in practice.

## Key Question

*"Would this still make sense if a tool other than PROOF or CROP consumed it?"*

## Lens - What to Verify

- README examples describe Pebble as its own standard, not a PROOF-private file.
- PROOF emission and CROP ingest/emit use the shared crate rather than duplicating
  schema structs.
- Integration tests cover both producer and consumer expectations.
- Tool-specific metadata stays outside the core schema unless it is broadly useful.
