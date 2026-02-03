use std::ops::Range;

use tree_sitter::{Parser, Query, QueryCursor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightKind {
    Heading,
    Emphasis,
    Strong,
    Code,
    Link,
    Punctuation,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighlightSpan {
    pub range: Range<usize>,
    pub kind: HighlightKind,
}

pub fn highlight_markdown(source: &str) -> Vec<HighlightSpan> {
    if source.is_empty() {
        return vec![];
    }

    let mut parser = Parser::new();
    let language = tree_sitter_md::LANGUAGE.into();
    if parser.set_language(&language).is_err() {
        return vec![];
    }

    let tree = match parser.parse(source, None) {
        Some(tree) => tree,
        None => return vec![],
    };

    let query = match Query::new(&language, tree_sitter_md::HIGHLIGHT_QUERY_BLOCK) {
        Ok(query) => query,
        Err(_) => return vec![],
    };

    let mut cursor = QueryCursor::new();
    let mut spans = Vec::new();

    for (m, capture_index) in cursor.captures(&query, tree.root_node(), source.as_bytes()) {
        let capture = m.captures[capture_index];
        let name = query
            .capture_names()
            .get(capture.index as usize)
            .map(String::as_str)
            .unwrap_or("");

        let kind = classify_capture_name(name);
        let range = capture.node.byte_range();
        if range.start < range.end && range.end <= source.len() {
            spans.push(HighlightSpan { range, kind });
        }
    }

    spans.sort_by_key(|s| (s.range.start, s.range.end));
    spans
}

fn classify_capture_name(name: &str) -> HighlightKind {
    let name = name.to_ascii_lowercase();
    if name.contains("heading") {
        HighlightKind::Heading
    } else if name.contains("strong") || name.contains("bold") {
        HighlightKind::Strong
    } else if name.contains("emphasis") || name.contains("italic") {
        HighlightKind::Emphasis
    } else if name.contains("code") {
        HighlightKind::Code
    } else if name.contains("link") || name.contains("url") {
        HighlightKind::Link
    } else if name.contains("punctuation") || name.contains("delimiter") {
        HighlightKind::Punctuation
    } else {
        HighlightKind::Other
    }
}

#[cfg(test)]
mod tests {
    use super::highlight_markdown;

    #[test]
    fn highlight_spans_are_within_bounds() {
        let source = "# Title\n\nHello **world**.\n";
        let spans = highlight_markdown(source);
        for span in spans {
            assert!(span.range.start < span.range.end);
            assert!(span.range.end <= source.len());
        }
    }
}
