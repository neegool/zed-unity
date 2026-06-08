//! Document handling for USS Language Server
//!
//! Manages document state, text operations, and document-related LSP features.

use once_cell::sync::Lazy;
use regex::Regex;
use ropey::Rope;
use tower_lsp::lsp_types::*;

/// Represents an open USS document
#[derive(Debug)]
pub struct Document {
    /// The document content as a rope for efficient editing
    pub content: Rope,
    /// Document version for sync
    pub version: i32,
}

impl Document {
    /// Create a new document from text content
    pub fn new(text: String, version: i32) -> Self {
        Self {
            content: Rope::from_str(&text),
            version,
        }
    }

    /// Set the entire document content
    pub fn set_content(&mut self, text: String) {
        self.content = Rope::from_str(&text);
    }

    /// Apply an incremental change to the document
    pub fn apply_change(&mut self, range: Range, new_text: &str) {
        let start_idx = self.position_to_offset(range.start);
        let end_idx = self.position_to_offset(range.end);

        if let (Some(start), Some(end)) = (start_idx, end_idx) {
            self.content.remove(start..end);
            self.content.insert(start, new_text);
        }
    }

    /// Convert a position to a character offset
    pub fn position_to_offset(&self, position: Position) -> Option<usize> {
        let line = position.line as usize;
        if line >= self.content.len_lines() {
            return None;
        }

        let line_start = self.content.line_to_char(line);
        let col = position.character as usize;
        let line_len = self.content.line(line).len_chars();

        if col > line_len {
            Some(line_start + line_len)
        } else {
            Some(line_start + col)
        }
    }

    /// Convert a character offset to a position
    pub fn offset_to_position(&self, offset: usize) -> Position {
        let line = self.content.char_to_line(offset);
        let line_start = self.content.line_to_char(line);
        let character = offset - line_start;

        Position {
            line: line as u32,
            character: character as u32,
        }
    }

    /// Get the word at a position
    pub fn get_word_at_position(&self, position: Position) -> Option<String> {
        let offset = self.position_to_offset(position)?;
        let text = self.content.to_string();
        let bytes = text.as_bytes();

        // Find word boundaries
        let mut start = offset;
        let mut end = offset;

        // Search backward for word start
        while start > 0 {
            let c = bytes[start - 1] as char;
            if !c.is_alphanumeric() && c != '-' && c != '_' {
                break;
            }
            start -= 1;
        }

        // Search forward for word end
        while end < bytes.len() {
            let c = bytes[end] as char;
            if !c.is_alphanumeric() && c != '-' && c != '_' {
                break;
            }
            end += 1;
        }

        if start < end {
            Some(text[start..end].to_string())
        } else {
            None
        }
    }

    /// Get the line at a position
    pub fn get_line(&self, line: u32) -> Option<String> {
        let line = line as usize;
        if line >= self.content.len_lines() {
            return None;
        }
        Some(self.content.line(line).to_string())
    }

    #[allow(dead_code)]
    /// Get the text before the cursor on the current line
    pub fn get_text_before_cursor(&self, position: Position) -> Option<String> {
        let line_text = self.get_line(position.line)?;
        let col = position.character as usize;
        if col > line_text.len() {
            Some(line_text)
        } else {
            Some(line_text[..col].to_string())
        }
    }

    /// Get the full document text
    pub fn get_text(&self) -> String {
        self.content.to_string()
    }

    /// Get the number of lines
    pub fn line_count(&self) -> usize {
        self.content.len_lines()
    }
}

/// Format an entire USS document
pub fn format_document(doc: &Document, options: &FormattingOptions) -> Vec<TextEdit> {
    let text = doc.get_text();
    let formatted = format_uss(&text, options);

    if formatted == text {
        return vec![];
    }

    vec![TextEdit {
        range: Range {
            start: Position {
                line: 0,
                character: 0,
            },
            end: doc.offset_to_position(doc.content.len_chars()),
        },
        new_text: formatted,
    }]
}

/// Format a range of a USS document
pub fn format_range(doc: &Document, range: Range, options: &FormattingOptions) -> Vec<TextEdit> {
    let start_offset = doc.position_to_offset(range.start).unwrap_or(0);
    let end_offset = doc
        .position_to_offset(range.end)
        .unwrap_or(doc.content.len_chars());

    let text = doc.get_text();
    let slice = &text[start_offset..end_offset];
    let formatted = format_uss(slice, options);

    if formatted == slice {
        return vec![];
    }

    vec![TextEdit {
        range,
        new_text: formatted,
    }]
}

/// Format USS content
fn format_uss(text: &str, options: &FormattingOptions) -> String {
    let indent = if options.insert_spaces {
        " ".repeat(options.tab_size as usize)
    } else {
        "\t".to_string()
    };

    let mut result = String::new();
    let mut indent_level: usize = 0;
    let mut in_comment = false;
    let mut prev_char = '\0';

    for c in text.chars() {
        // Handle block comments
        if prev_char == '/' && c == '*' {
            in_comment = true;
        } else if prev_char == '*' && c == '/' {
            in_comment = false;
        }

        if in_comment {
            result.push(c);
            prev_char = c;
            continue;
        }

        match c {
            '{' => {
                // Ensure space before brace
                if !result.ends_with(' ') && !result.ends_with('\n') {
                    result.push(' ');
                }
                result.push(c);
                result.push('\n');
                indent_level += 1;
            }
            '}' => {
                // Trim trailing whitespace
                while result.ends_with(' ') || result.ends_with('\t') {
                    result.pop();
                }
                if !result.ends_with('\n') {
                    result.push('\n');
                }
                indent_level = indent_level.saturating_sub(1);
                result.push_str(&indent.repeat(indent_level));
                result.push(c);
                result.push('\n');
            }
            ';' => {
                result.push(c);
                result.push('\n');
            }
            ':' => {
                result.push(c);
                // Add space after colon in property declarations
                if !result.ends_with(": ") {
                    result.push(' ');
                }
            }
            '\n' => {
                // Already handled by other cases
                if !result.ends_with('\n') {
                    result.push('\n');
                }
            }
            ' ' | '\t' => {
                // Handle indentation at line start
                if result.ends_with('\n') {
                    result.push_str(&indent.repeat(indent_level));
                } else if !result.ends_with(' ') && !result.ends_with('\t') {
                    result.push(' ');
                }
            }
            _ => {
                // Add indentation if at line start
                if result.ends_with('\n') {
                    result.push_str(&indent.repeat(indent_level));
                }
                result.push(c);
            }
        }

        prev_char = c;
    }

    result
}

#[allow(dead_code)]
/// Regex for matching USS variables
static VAR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"--[\w-]+").unwrap());

#[allow(dead_code)]
/// Regex for matching var() usage
static VAR_USAGE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"var\s*\(\s*(--[\w-]+)\s*\)").unwrap());

#[allow(dead_code)]
/// Regex for matching class selectors
static CLASS_SELECTOR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\.[\w-]+").unwrap());

#[allow(dead_code)]
/// Regex for matching ID selectors
static ID_SELECTOR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"#[\w-]+").unwrap());

/// Regex for matching hex colors
static HEX_COLOR_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"#([0-9A-Fa-f]{3,8})\b").unwrap());

/// Regex for matching rgb/rgba colors
static RGBA_COLOR_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"rgba?\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*(?:,\s*([\d.]+)\s*)?\)").unwrap()
});

/// Find definition of a variable or selector
pub fn find_definition(doc: &Document, position: Position, uri: &str) -> Option<Location> {
    let word = doc.get_word_at_position(position)?;
    let text = doc.get_text();

    // Check if it's a variable reference
    if word.starts_with("--") {
        // Look for variable definition (e.g., --var-name: value;)
        let pattern = format!(r"({})\s*:", regex::escape(&word));
        if let Ok(re) = Regex::new(&pattern) {
            if let Some(m) = re.find(&text) {
                let start = doc.offset_to_position(m.start());
                let end = doc.offset_to_position(m.end() - 1); // Exclude colon

                return Some(Location {
                    uri: uri.parse().ok()?,
                    range: Range { start, end },
                });
            }
        }
    }

    // Check if it's a class selector
    if word.starts_with('.') {
        let pattern = format!(r"({})\s*\{{", regex::escape(&word));
        if let Ok(re) = Regex::new(&pattern) {
            if let Some(m) = re.find(&text) {
                let start = doc.offset_to_position(m.start());
                let end = doc.offset_to_position(m.start() + word.len());

                return Some(Location {
                    uri: uri.parse().ok()?,
                    range: Range { start, end },
                });
            }
        }
    }

    None
}

/// Find all references to a variable or selector
pub fn find_references(doc: &Document, position: Position, uri: &str) -> Vec<Location> {
    let mut refs = Vec::new();
    let word = match doc.get_word_at_position(position) {
        Some(w) => w,
        None => return refs,
    };
    let text = doc.get_text();

    // Find all occurrences of the word
    let pattern = regex::escape(&word);
    if let Ok(re) = Regex::new(&format!(r"\b{}\b", pattern)) {
        for m in re.find_iter(&text) {
            let start = doc.offset_to_position(m.start());
            let end = doc.offset_to_position(m.end());

            if let Ok(url) = uri.parse() {
                refs.push(Location {
                    uri: url,
                    range: Range { start, end },
                });
            }
        }
    }

    refs
}

/// Rename a variable or selector
pub fn rename(
    doc: &Document,
    position: Position,
    new_name: &str,
    uri: &str,
) -> Option<WorkspaceEdit> {
    let word = doc.get_word_at_position(position)?;
    let text = doc.get_text();

    let mut edits = Vec::new();
    let pattern = regex::escape(&word);

    if let Ok(re) = Regex::new(&format!(r"\b{}\b", pattern)) {
        for m in re.find_iter(&text) {
            let start = doc.offset_to_position(m.start());
            let end = doc.offset_to_position(m.end());

            edits.push(TextEdit {
                range: Range { start, end },
                new_text: new_name.to_string(),
            });
        }
    }

    if edits.is_empty() {
        return None;
    }

    let url: Url = uri.parse().ok()?;
    let mut changes = std::collections::HashMap::new();
    changes.insert(url, edits);

    Some(WorkspaceEdit {
        changes: Some(changes),
        document_changes: None,
        change_annotations: None,
    })
}

/// Extract colors from the document
pub fn get_colors(doc: &Document) -> Vec<ColorInformation> {
    let mut colors = Vec::new();
    let text = doc.get_text();

    // Find hex colors
    for cap in HEX_COLOR_REGEX.captures_iter(&text) {
        if let Some(m) = cap.get(0) {
            let hex = cap.get(1).map(|c| c.as_str()).unwrap_or("");
            if let Some(color) = parse_hex_color(hex) {
                let start = doc.offset_to_position(m.start());
                let end = doc.offset_to_position(m.end());
                colors.push(ColorInformation {
                    range: Range { start, end },
                    color,
                });
            }
        }
    }

    // Find rgb/rgba colors
    for cap in RGBA_COLOR_REGEX.captures_iter(&text) {
        if let Some(m) = cap.get(0) {
            let r: f32 = cap
                .get(1)
                .and_then(|c| c.as_str().parse().ok())
                .unwrap_or(0.0);
            let g: f32 = cap
                .get(2)
                .and_then(|c| c.as_str().parse().ok())
                .unwrap_or(0.0);
            let b: f32 = cap
                .get(3)
                .and_then(|c| c.as_str().parse().ok())
                .unwrap_or(0.0);
            let a: f32 = cap
                .get(4)
                .and_then(|c| c.as_str().parse().ok())
                .unwrap_or(1.0);

            let start = doc.offset_to_position(m.start());
            let end = doc.offset_to_position(m.end());

            colors.push(ColorInformation {
                range: Range { start, end },
                color: Color {
                    red: r / 255.0,
                    green: g / 255.0,
                    blue: b / 255.0,
                    alpha: a,
                },
            });
        }
    }

    colors
}

/// Parse a hex color string to LSP Color
fn parse_hex_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');

    match hex.len() {
        3 => {
            // RGB shorthand
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some(Color {
                red: r as f32 / 255.0,
                green: g as f32 / 255.0,
                blue: b as f32 / 255.0,
                alpha: 1.0,
            })
        }
        4 => {
            // RGBA shorthand
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).ok()?;
            Some(Color {
                red: r as f32 / 255.0,
                green: g as f32 / 255.0,
                blue: b as f32 / 255.0,
                alpha: a as f32 / 255.0,
            })
        }
        6 => {
            // RRGGBB
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Color {
                red: r as f32 / 255.0,
                green: g as f32 / 255.0,
                blue: b as f32 / 255.0,
                alpha: 1.0,
            })
        }
        8 => {
            // RRGGBBAA
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some(Color {
                red: r as f32 / 255.0,
                green: g as f32 / 255.0,
                blue: b as f32 / 255.0,
                alpha: a as f32 / 255.0,
            })
        }
        _ => None,
    }
}

/// Get color presentations for a color
pub fn get_color_presentations(color: Color) -> Vec<ColorPresentation> {
    let r = (color.red * 255.0).round() as u8;
    let g = (color.green * 255.0).round() as u8;
    let b = (color.blue * 255.0).round() as u8;
    let a = color.alpha;

    let mut presentations = Vec::new();

    // Hex format
    if a >= 1.0 {
        presentations.push(ColorPresentation {
            label: format!("#{:02X}{:02X}{:02X}", r, g, b),
            text_edit: None,
            additional_text_edits: None,
        });
    } else {
        let a_byte = (a * 255.0).round() as u8;
        presentations.push(ColorPresentation {
            label: format!("#{:02X}{:02X}{:02X}{:02X}", r, g, b, a_byte),
            text_edit: None,
            additional_text_edits: None,
        });
    }

    // RGB/RGBA format
    if a >= 1.0 {
        presentations.push(ColorPresentation {
            label: format!("rgb({}, {}, {})", r, g, b),
            text_edit: None,
            additional_text_edits: None,
        });
    } else {
        presentations.push(ColorPresentation {
            label: format!("rgba({}, {}, {}, {:.2})", r, g, b, a),
            text_edit: None,
            additional_text_edits: None,
        });
    }

    presentations
}
