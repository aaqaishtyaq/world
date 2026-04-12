use std::error::Error;
use std::fs;
use std::path::Path;

use once_cell::sync::Lazy;
use pulldown_cmark::{CodeBlockKind, Event, Options, Parser, Tag, TagEnd};
use regex::Regex;

use crate::classify::{is_diagram_block, nonempty_line_count};
use crate::model::{PageTally, ReadingStats};

static SHORTCODE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)\{\{.*?\}\}").expect("valid shortcode regex"));
static HTML_COMMENT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?s)<!--.*?-->").expect("valid HTML comment regex"));
static WORD_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b[\p{L}\p{N}_']+\b").expect("valid word regex"));

pub fn compute_stats(path: &Path) -> Result<ReadingStats, Box<dyn Error>> {
    let source = fs::read_to_string(path)?;
    let body = strip_front_matter(&source);
    let sanitized = sanitize_markdown(body);
    let tally = tally_markdown(&sanitized);

    Ok(ReadingStats::from_tally(tally))
}

fn strip_front_matter(source: &str) -> &str {
    let Some(rest) = source
        .strip_prefix("+++\n")
        .or_else(|| source.strip_prefix("---\n"))
    else {
        return source;
    };

    if let Some((_, body)) = rest.split_once("\n+++\n") {
        return body;
    }
    if let Some((_, body)) = rest.split_once("\n---\n") {
        return body;
    }

    source
}

fn sanitize_markdown(body: &str) -> String {
    let without_comments = HTML_COMMENT_RE.replace_all(body, " ");
    SHORTCODE_RE
        .replace_all(&without_comments, " ")
        .into_owned()
}

fn tally_markdown(markdown: &str) -> PageTally {
    let mut tally = PageTally::default();
    let parser = Parser::new_ext(markdown, Options::all());

    let mut image_depth = 0usize;
    let mut in_code_block: Option<(String, String)> = None;

    for event in parser {
        match event {
            Event::Start(Tag::Image { .. }) => {
                image_depth += 1;
            }
            Event::End(TagEnd::Image) => {
                image_depth = image_depth.saturating_sub(1);
            }
            Event::Start(Tag::CodeBlock(kind)) => {
                let info = match kind {
                    CodeBlockKind::Fenced(info) => info.into_string(),
                    CodeBlockKind::Indented => String::new(),
                };
                in_code_block = Some((info, String::new()));
            }
            Event::End(TagEnd::CodeBlock) => {
                if let Some((info, content)) = in_code_block.take() {
                    let nonempty_lines = nonempty_line_count(&content);
                    if nonempty_lines == 0 {
                        continue;
                    }

                    if is_diagram_block(&info, &content) {
                        tally.diagram_lines += nonempty_lines;
                    } else {
                        tally.code_lines += nonempty_lines;
                    }
                }
            }
            Event::Text(text) => {
                if let Some((_, content)) = in_code_block.as_mut() {
                    content.push_str(&text);
                } else if image_depth == 0 {
                    tally.prose_words += count_words(&text);
                }
            }
            Event::Code(text) => {
                if image_depth == 0 {
                    tally.prose_words += count_words(&text);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if let Some((_, content)) = in_code_block.as_mut() {
                    content.push('\n');
                }
            }
            _ => {}
        }
    }

    tally
}

fn count_words(text: &str) -> usize {
    WORD_RE.find_iter(text).count()
}
