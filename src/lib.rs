//! Pebble v1: compact context transfer records for documents and corpus slices.
//!
//! Pebbles are optimized for agents and corpus tooling, not human presentation.
//! PROOF can emit Pebbles from compiled source documents; CROP can emit the same
//! schema for views and corpus slices.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SCHEMA: &str = "pebble.v1";
pub const DOCUMENT_KIND: &str = "document";
pub const MARKDOWN_FORMAT: &str = "markdown";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PebbleDocument {
    pub schema: String,
    pub kind: String,
    pub title: String,
    pub source: String,
    pub format: String,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
    pub sections: Vec<PebbleSection>,
    #[serde(default)]
    pub refs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PebbleSection {
    pub id: String,
    pub path: Vec<String>,
    pub level: usize,
    pub line: usize,
    #[serde(default)]
    pub metadata: BTreeMap<String, String>,
    pub text: String,
}

impl PebbleDocument {
    pub fn from_markdown(
        markdown: &str,
        fallback_title: &str,
        source: impl Into<String>,
        refs: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let parsed = parse_markdown_frontmatter(markdown);
        let title = parsed
            .metadata
            .get("title")
            .cloned()
            .unwrap_or_else(|| document_title(parsed.content, fallback_title));
        Self {
            schema: SCHEMA.to_string(),
            kind: DOCUMENT_KIND.to_string(),
            title,
            source: source.into(),
            format: MARKDOWN_FORMAT.to_string(),
            metadata: parsed.metadata.clone(),
            sections: markdown_sections_with_metadata(
                parsed.content,
                &parsed.metadata,
                parsed.start_line,
            ),
            refs: refs.into_iter().map(Into::into).collect(),
        }
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        serde_json::from_str(json)
    }
}

pub fn document_title(markdown: &str, fallback: &str) -> String {
    markdown
        .lines()
        .find_map(|line| {
            let trimmed = line.trim();
            trimmed
                .strip_prefix("# ")
                .map(str::trim)
                .filter(|title| !title.is_empty())
        })
        .unwrap_or(fallback)
        .to_string()
}

pub fn markdown_sections(markdown: &str) -> Vec<PebbleSection> {
    markdown_sections_with_metadata(markdown, &BTreeMap::new(), 1)
}

fn markdown_sections_with_metadata(
    markdown: &str,
    metadata: &BTreeMap<String, String>,
    start_line: usize,
) -> Vec<PebbleSection> {
    let mut sections = Vec::new();
    let mut current_start = start_line;
    let mut current_level = 0usize;
    let mut current_path: Vec<String> = Vec::new();
    let mut current_text = String::new();
    let mut heading_stack: Vec<(usize, String)> = Vec::new();

    for (index, line) in markdown.lines().enumerate() {
        let line_number = index + start_line;
        if let Some((level, heading)) = parse_heading(line) {
            push_section(
                &mut sections,
                current_start,
                current_level,
                &current_path,
                &current_text,
                metadata,
            );
            while heading_stack
                .last()
                .is_some_and(|(stack_level, _)| *stack_level >= level)
            {
                heading_stack.pop();
            }
            heading_stack.push((level, heading.to_string()));
            current_path = heading_stack
                .iter()
                .map(|(_, heading)| heading.clone())
                .collect();
            current_start = line_number;
            current_level = level;
            current_text.clear();
        }
        current_text.push_str(line);
        current_text.push('\n');
    }

    push_section(
        &mut sections,
        current_start,
        current_level,
        &current_path,
        &current_text,
        metadata,
    );

    if sections.is_empty() {
        sections.push(PebbleSection {
            id: "document".to_string(),
            path: Vec::new(),
            level: 0,
            line: start_line,
            metadata: metadata.clone(),
            text: String::new(),
        });
    }

    sections
}

fn push_section(
    sections: &mut Vec<PebbleSection>,
    line: usize,
    level: usize,
    path: &[String],
    text: &str,
    metadata: &BTreeMap<String, String>,
) {
    let text = text.trim().to_string();
    if text.is_empty() {
        return;
    }
    let base = path.last().map_or("preamble", String::as_str);
    let id = unique_section_id(sections, base);
    sections.push(PebbleSection {
        id,
        path: path.to_vec(),
        level,
        line,
        metadata: metadata.clone(),
        text,
    });
}

struct ParsedMarkdown<'a> {
    metadata: BTreeMap<String, String>,
    content: &'a str,
    start_line: usize,
}

fn parse_markdown_frontmatter(markdown: &str) -> ParsedMarkdown<'_> {
    let mut lines = markdown.split_inclusive('\n');
    let Some(first_line) = lines.next() else {
        return ParsedMarkdown {
            metadata: BTreeMap::new(),
            content: markdown,
            start_line: 1,
        };
    };
    if trim_line_end(first_line).trim() != "---" {
        return ParsedMarkdown {
            metadata: BTreeMap::new(),
            content: markdown,
            start_line: 1,
        };
    }

    let mut metadata = BTreeMap::new();
    let mut consumed_bytes = first_line.len();
    let mut start_line = 2usize;

    for line_with_newline in lines {
        let line = trim_line_end(line_with_newline);
        if line.trim() == "---" {
            consumed_bytes = consumed_bytes.saturating_add(line_with_newline.len());
            return ParsedMarkdown {
                metadata,
                content: markdown.get(consumed_bytes..).unwrap_or(""),
                start_line: start_line + 1,
            };
        }
        if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            if !key.is_empty() {
                metadata.insert(key.to_string(), clean_frontmatter_value(value));
            }
        }
        consumed_bytes = consumed_bytes.saturating_add(line_with_newline.len());
        start_line += 1;
    }

    ParsedMarkdown {
        metadata: BTreeMap::new(),
        content: markdown,
        start_line: 1,
    }
}

fn trim_line_end(line: &str) -> &str {
    line.trim_end_matches(['\r', '\n'])
}

fn clean_frontmatter_value(value: &str) -> String {
    let value = value.trim();
    value
        .strip_prefix('"')
        .and_then(|value| value.strip_suffix('"'))
        .or_else(|| {
            value
                .strip_prefix('\'')
                .and_then(|value| value.strip_suffix('\''))
        })
        .unwrap_or(value)
        .to_string()
}

fn unique_section_id(sections: &[PebbleSection], heading: &str) -> String {
    let base = slugify(heading);
    let base = if base.is_empty() {
        "section".to_string()
    } else {
        base
    };
    let mut id = base.clone();
    let mut suffix = 2usize;
    while sections.iter().any(|section| section.id == id) {
        id = format!("{}-{}", base, suffix);
        suffix += 1;
    }
    id
}

pub fn slugify(text: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = false;
    for c in text.chars().flat_map(char::to_lowercase) {
        if c.is_ascii_alphanumeric() {
            slug.push(c);
            last_dash = false;
        } else if !last_dash && !slug.is_empty() {
            slug.push('-');
            last_dash = true;
        }
    }
    if last_dash {
        slug.pop();
    }
    slug
}

fn parse_heading(line: &str) -> Option<(usize, &str)> {
    let hashes = line.chars().take_while(|&c| c == '#').count();
    if !(1..=6).contains(&hashes) {
        return None;
    }
    let rest = line.get(hashes..)?;
    if !rest.starts_with(' ') {
        return None;
    }
    Some((hashes, rest.trim()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chunks_markdown_by_heading_path() {
        let pebble = PebbleDocument::from_markdown(
            "# Guide\n\nIntro.\n\n## Steps\n\n- one\n- two\n",
            "fallback",
            "guide.source.md",
            [".proof/side-info/links.json"],
        );

        assert_eq!(pebble.schema, SCHEMA);
        assert_eq!(pebble.title, "Guide");
        assert_eq!(pebble.refs, vec![".proof/side-info/links.json"]);
        assert!(pebble.metadata.is_empty());
        assert_eq!(pebble.sections[0].id, "guide");
        assert!(pebble.sections[0].metadata.is_empty());
        assert_eq!(pebble.sections[0].path, vec!["Guide"]);
        assert_eq!(pebble.sections[1].id, "steps");
        assert_eq!(pebble.sections[1].path, vec!["Guide", "Steps"]);
        assert!(pebble.sections[1].text.contains("- one"));
    }

    #[test]
    fn round_trips_json() {
        let pebble = PebbleDocument::from_markdown(
            "Body only\n",
            "Fallback",
            "body.md",
            Vec::<String>::new(),
        );
        let json = pebble.to_json().unwrap();
        let loaded = PebbleDocument::from_json(&json).unwrap();

        assert_eq!(loaded, pebble);
        assert_eq!(loaded.sections[0].id, "preamble");
    }

    #[test]
    fn duplicate_headings_get_stable_suffixes() {
        let sections = markdown_sections("# Item\n\nA\n\n# Item\n\nB\n");

        assert_eq!(sections[0].id, "item");
        assert_eq!(sections[1].id, "item-2");
    }

    #[test]
    fn frontmatter_becomes_document_and_section_metadata() {
        let pebble = PebbleDocument::from_markdown(
            "---\ntitle: Frontmatter Guide\ntags: [proof, guide]\nstatus: ready\n---\n\n# Body Title\n\nContent.",
            "Fallback",
            "guide.source.md",
            Vec::<String>::new(),
        );

        assert_eq!(pebble.title, "Frontmatter Guide");
        assert_eq!(
            pebble.metadata.get("tags").map(String::as_str),
            Some("[proof, guide]")
        );
        assert_eq!(
            pebble.sections[0]
                .metadata
                .get("status")
                .map(String::as_str),
            Some("ready")
        );
        assert_eq!(pebble.sections[0].line, 7);
        assert!(!pebble.sections[0].text.contains("status: ready"));
    }

    #[test]
    fn deserializes_legacy_json_without_metadata() {
        let json = r##"{
          "schema": "pebble.v1",
          "kind": "document",
          "title": "Legacy",
          "source": "legacy.md",
          "format": "markdown",
          "sections": [
            {
              "id": "legacy",
              "path": ["Legacy"],
              "level": 1,
              "line": 1,
              "text": "# Legacy\n"
            }
          ],
          "refs": []
        }"##;

        let loaded = PebbleDocument::from_json(json).unwrap();

        assert!(loaded.metadata.is_empty());
        assert!(loaded.sections[0].metadata.is_empty());
    }
}
