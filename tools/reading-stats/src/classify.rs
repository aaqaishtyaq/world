use once_cell::sync::Lazy;
use regex::Regex;

static CODE_TOKENS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"[{}();]|\b(if|else|for|while|return|func|package|import|const|let|var|class|def|SELECT|FROM|WHERE|server|location)\b|=>|:=|::",
    )
    .expect("valid code token regex")
});
static DIAGRAM_TOKENS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"->|<->|<--|-->|==>|<==|<=>|[+]{2,}|[|]{2,}|^\s*[|v^]+\s*$|[┌┐└┘│─├┤┬┴┼]")
        .expect("valid diagram token regex")
});

static DIAGRAM_LANGUAGE_HINTS: &[&str] = &[
    "ascii",
    "blockdiag",
    "diagram",
    "dot",
    "graphviz",
    "mermaid",
    "plantuml",
];

pub fn nonempty_line_count(content: &str) -> usize {
    content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

pub fn is_diagram_block(info: &str, content: &str) -> bool {
    let language = info
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();
    if DIAGRAM_LANGUAGE_HINTS.contains(&language.as_str()) {
        return true;
    }

    let nonempty_lines: Vec<&str> = content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .collect();
    if nonempty_lines.len() < 2 {
        return false;
    }

    let mut diagramish = 0usize;
    let mut codeish = 0usize;

    for line in &nonempty_lines {
        let stripped = line.trim();
        let mut letters = 0usize;
        let mut digits = 0usize;
        let mut symbols = 0usize;

        for ch in stripped.chars() {
            if ch.is_alphabetic() {
                letters += 1;
            } else if ch.is_numeric() {
                digits += 1;
            } else if !ch.is_whitespace() {
                symbols += 1;
            }
        }

        let arrowy = DIAGRAM_TOKENS_RE.is_match(stripped);
        let boxy = stripped
            .chars()
            .any(|ch| matches!(ch, '|' | '+' | '-' | '/' | '\\' | '=' | '<' | '>'));

        if arrowy || (boxy && symbols >= letters + digits) {
            diagramish += 1;
        }
        if CODE_TOKENS_RE.is_match(stripped) {
            codeish += 1;
        }
    }

    diagramish >= usize::max(2, nonempty_lines.len().div_ceil(2)) && diagramish > codeish
}
