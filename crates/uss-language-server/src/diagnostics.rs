//! Diagnostics provider for USS Language Server
//!
//! Provides syntax and semantic diagnostics for USS documents.

use crate::document::Document;
use crate::uss_data::USS_PROPERTIES;
use once_cell::sync::Lazy;
use regex::Regex;
use tower_lsp::lsp_types::*;

/// Regex patterns for diagnostics
static PROPERTY_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^\s*([\w-]+)\s*:\s*([^;]+);?\s*$").unwrap());

#[allow(dead_code)]
static SELECTOR_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[.#\w\[\]:,\s>+~*-]+\s*\{").unwrap());

static HEX_COLOR_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"#([0-9A-Fa-f]+)\b").unwrap());

#[allow(dead_code)]
static UNCLOSED_BRACE_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{[^}]*$").unwrap());

#[allow(dead_code)]
static UNCLOSED_PAREN_PATTERN: Lazy<Regex> = Lazy::new(|| Regex::new(r"\([^)]*$").unwrap());

#[allow(dead_code)]
static INVALID_SELECTOR_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[^.#\w\[\]:,\s>+~*-]").unwrap());

/// Get diagnostics for a USS document
pub fn get_diagnostics(doc: &Document) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let text = doc.get_text();

    // Track brace balance
    let mut brace_depth = 0;
    let mut in_declaration_block = false;

    for (line_num, line) in text.lines().enumerate() {
        let line_diagnostics =
            check_line(line, line_num, &mut brace_depth, &mut in_declaration_block);
        diagnostics.extend(line_diagnostics);
    }

    // Check for unclosed braces at end of document
    if brace_depth > 0 {
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position {
                    line: doc.line_count().saturating_sub(1) as u32,
                    character: 0,
                },
                end: Position {
                    line: doc.line_count().saturating_sub(1) as u32,
                    character: 0,
                },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("uss".to_string()),
            message: format!(
                "Unclosed brace(s): {} opening brace(s) without closing",
                brace_depth
            ),
            ..Default::default()
        });
    } else if brace_depth < 0 {
        diagnostics.push(Diagnostic {
            range: Range {
                start: Position {
                    line: doc.line_count().saturating_sub(1) as u32,
                    character: 0,
                },
                end: Position {
                    line: doc.line_count().saturating_sub(1) as u32,
                    character: 0,
                },
            },
            severity: Some(DiagnosticSeverity::ERROR),
            source: Some("uss".to_string()),
            message: format!(
                "Extra closing brace(s): {} more closing than opening",
                -brace_depth
            ),
            ..Default::default()
        });
    }

    diagnostics
}

/// Check a single line for diagnostics
fn check_line(
    line: &str,
    line_num: usize,
    brace_depth: &mut i32,
    in_declaration_block: &mut bool,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let trimmed = line.trim();

    // Skip empty lines and comments
    if trimmed.is_empty() || trimmed.starts_with("/*") || trimmed.starts_with("//") {
        return diagnostics;
    }

    // Track brace depth
    let open_braces = line.matches('{').count() as i32;
    let close_braces = line.matches('}').count() as i32;

    *brace_depth += open_braces - close_braces;
    *in_declaration_block = *brace_depth > 0;

    // Check for property declarations inside blocks
    if *in_declaration_block {
        diagnostics.extend(check_property_declaration(line, line_num));
    }

    // Check for invalid hex colors
    diagnostics.extend(check_hex_colors(line, line_num));

    // Check for unclosed parentheses in functions
    diagnostics.extend(check_unclosed_parens(line, line_num));

    // Check for missing semicolons in declarations
    if *in_declaration_block
        && !trimmed.is_empty()
        && trimmed.contains(':')
        && !trimmed.ends_with(';')
        && !trimmed.ends_with('{')
        && !trimmed.ends_with('}')
    {
        // Allow multi-line values, but warn about potential missing semicolons
        // Only warn if this looks like a complete declaration
        let colon_count = trimmed.matches(':').count();
        if colon_count == 1 && !trimmed.contains("url(") && !trimmed.contains("var(") {
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: line_num as u32,
                        character: line.len().saturating_sub(1) as u32,
                    },
                    end: Position {
                        line: line_num as u32,
                        character: line.len() as u32,
                    },
                },
                severity: Some(DiagnosticSeverity::WARNING),
                source: Some("uss".to_string()),
                message: "Missing semicolon at end of declaration".to_string(),
                ..Default::default()
            });
        }
    }

    diagnostics
}

/// Check property declarations for validity
fn check_property_declaration(line: &str, line_num: usize) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let trimmed = line.trim();

    // Look for property declarations
    if let Some(caps) = PROPERTY_PATTERN.captures(trimmed) {
        let property_name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let property_value = caps.get(2).map(|m| m.as_str()).unwrap_or("");

        // Check if property is known
        if !property_name.is_empty()
            && !property_name.starts_with("--")
            && !USS_PROPERTIES.contains_key(property_name)
        {
            let start_char = line.find(property_name).unwrap_or(0);
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: line_num as u32,
                        character: start_char as u32,
                    },
                    end: Position {
                        line: line_num as u32,
                        character: (start_char + property_name.len()) as u32,
                    },
                },
                severity: Some(DiagnosticSeverity::WARNING),
                source: Some("uss".to_string()),
                message: format!("Unknown USS property: '{}'", property_name),
                ..Default::default()
            });
        }

        // Check for empty values
        if property_value.trim().is_empty() {
            let colon_pos = line.find(':').unwrap_or(0);
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: line_num as u32,
                        character: colon_pos as u32,
                    },
                    end: Position {
                        line: line_num as u32,
                        character: line.len() as u32,
                    },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("uss".to_string()),
                message: "Property value is empty".to_string(),
                ..Default::default()
            });
        }
    }

    diagnostics
}

/// Check for invalid hex colors
fn check_hex_colors(line: &str, line_num: usize) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    for caps in HEX_COLOR_PATTERN.captures_iter(line) {
        if let Some(hex_match) = caps.get(1) {
            let hex = hex_match.as_str();
            let valid_lengths = [3, 4, 6, 8];

            if !valid_lengths.contains(&hex.len()) {
                let start = hex_match.start().saturating_sub(1); // Include the #
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position {
                            line: line_num as u32,
                            character: start as u32,
                        },
                        end: Position {
                            line: line_num as u32,
                            character: hex_match.end() as u32,
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    source: Some("uss".to_string()),
                    message: format!(
                        "Invalid hex color length: {}. Expected 3, 4, 6, or 8 characters.",
                        hex.len()
                    ),
                    ..Default::default()
                });
            }
        }
    }

    diagnostics
}

/// Check for unclosed parentheses in function calls
fn check_unclosed_parens(line: &str, line_num: usize) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    let open_parens = line.matches('(').count();
    let close_parens = line.matches(')').count();

    if open_parens != close_parens {
        // Find the position of the unmatched paren
        let mut depth = 0;
        let mut last_open = 0;

        for (i, c) in line.char_indices() {
            match c {
                '(' => {
                    depth += 1;
                    last_open = i;
                }
                ')' => {
                    depth -= 1;
                    if depth < 0 {
                        // Extra closing paren
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position {
                                    line: line_num as u32,
                                    character: i as u32,
                                },
                                end: Position {
                                    line: line_num as u32,
                                    character: (i + 1) as u32,
                                },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            source: Some("uss".to_string()),
                            message: "Unmatched closing parenthesis".to_string(),
                            ..Default::default()
                        });
                        depth = 0;
                    }
                }
                _ => {}
            }
        }

        if depth > 0 {
            // Unclosed opening paren
            diagnostics.push(Diagnostic {
                range: Range {
                    start: Position {
                        line: line_num as u32,
                        character: last_open as u32,
                    },
                    end: Position {
                        line: line_num as u32,
                        character: (last_open + 1) as u32,
                    },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                source: Some("uss".to_string()),
                message: "Unclosed parenthesis".to_string(),
                ..Default::default()
            });
        }
    }

    diagnostics
}
