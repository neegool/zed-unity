//! Completion provider for USS Language Server
//!
//! Provides intelligent code completion for USS properties, values, selectors, and more.

use crate::document::Document;
use crate::uss_data::{USS_COLORS, USS_PROPERTIES, USS_PSEUDO_CLASSES, USS_UNITS, UXML_ELEMENTS};
use tower_lsp::lsp_types::*;

/// Context for completion
#[derive(Debug, Clone, PartialEq)]
enum CompletionContext {
    /// At the start of a selector
    Selector,
    /// After a class selector dot
    ClassSelector,
    /// After an ID selector hash
    IdSelector,
    /// After a pseudo-class colon
    PseudoClass,
    /// Inside a declaration block, expecting property name
    PropertyName,
    /// After a property colon, expecting value
    PropertyValue(String),
    /// Inside a url() or resource() function
    Url,
    /// Inside a var() function
    Variable,
    /// Unknown context
    Unknown,
}

/// Determine the completion context based on the cursor position
fn get_completion_context(doc: &Document, position: Position) -> CompletionContext {
    let line = match doc.get_line(position.line) {
        Some(l) => l,
        None => return CompletionContext::Unknown,
    };

    let col = position.character as usize;
    let text_before = if col > line.len() {
        &line
    } else {
        &line[..col]
    };

    // Check if we're in a var() function
    if text_before.contains("var(") && !text_before.contains(')') {
        return CompletionContext::Variable;
    }

    // Check if we're in a url() or resource() function
    if (text_before.contains("url(") || text_before.contains("resource("))
        && !text_before.ends_with(')')
    {
        return CompletionContext::Url;
    }

    // Check if we're after a pseudo-class colon
    if text_before.ends_with(':') && !text_before.ends_with("::") {
        // Check if we're inside a declaration block (has property before colon)
        let trimmed = text_before.trim();
        if trimmed.contains('{') || !trimmed.contains(':') || trimmed.ends_with(':') {
            // If the last colon is preceded by a selector-like pattern, it's a pseudo-class
            let before_colon = &text_before[..text_before.len() - 1];
            if before_colon
                .chars()
                .last()
                .map(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == ')')
                .unwrap_or(false)
            {
                // Could be either pseudo-class or property value
                // If there's already a property name on this line, it's a value
                if before_colon.contains(':') {
                    // Already had a colon, this might be double-colon for pseudo-element
                    return CompletionContext::PseudoClass;
                }
            }
        }
        // Check if this looks like a property declaration
        if text_before.trim().chars().filter(|c| *c == ':').count() == 1 {
            // First colon on the line - likely a property value
            let prop_name = text_before.trim().split(':').next().unwrap_or("").trim();
            if !prop_name.is_empty() && !prop_name.starts_with('.') && !prop_name.starts_with('#') {
                return CompletionContext::PropertyValue(prop_name.to_string());
            }
        }
        return CompletionContext::PseudoClass;
    }

    // Check if we're after a class selector dot
    if text_before.ends_with('.') {
        return CompletionContext::ClassSelector;
    }

    // Check if we're after an ID selector hash
    if text_before.ends_with('#') {
        return CompletionContext::IdSelector;
    }

    // Check if we're inside a declaration block
    let full_text = doc.get_text();
    let offset = doc.position_to_offset(position).unwrap_or(0);
    let text_before_full = &full_text[..offset];

    let open_braces = text_before_full.matches('{').count();
    let close_braces = text_before_full.matches('}').count();

    if open_braces > close_braces {
        // We're inside a declaration block

        // Check if we're after a property colon (expecting value)
        if text_before.contains(':') {
            let prop_name = text_before.trim().split(':').next().unwrap_or("").trim();
            if !prop_name.is_empty() {
                return CompletionContext::PropertyValue(prop_name.to_string());
            }
        }

        // Check if we're at the start of a new property
        let trimmed = text_before.trim();
        if trimmed.is_empty()
            || trimmed.ends_with(';')
            || trimmed.ends_with('{')
            || trimmed
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        {
            return CompletionContext::PropertyName;
        }
    } else {
        // We're outside declaration blocks - in selector context
        return CompletionContext::Selector;
    }

    CompletionContext::Unknown
}

/// Get completions based on the current context
pub fn get_completions(doc: &Document, position: Position) -> Vec<CompletionItem> {
    let context = get_completion_context(doc, position);

    match context {
        CompletionContext::Selector => get_selector_completions(),
        CompletionContext::ClassSelector => get_class_selector_completions(doc),
        CompletionContext::IdSelector => get_id_selector_completions(doc),
        CompletionContext::PseudoClass => get_pseudo_class_completions(),
        CompletionContext::PropertyName => get_property_name_completions(),
        CompletionContext::PropertyValue(prop) => get_property_value_completions(&prop),
        CompletionContext::Url => get_url_completions(),
        CompletionContext::Variable => get_variable_completions(doc),
        CompletionContext::Unknown => vec![],
    }
}

/// Get selector completions (element types)
fn get_selector_completions() -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Add UXML element types
    for elem in UXML_ELEMENTS.iter() {
        items.push(CompletionItem {
            label: elem.name.to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some(elem.namespace.to_string()),
            documentation: Some(Documentation::String(elem.description.to_string())),
            insert_text: Some(format!("{} {{\n    $0\n}}", elem.name)),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });
    }

    // Add common selector prefixes
    items.push(CompletionItem {
        label: ".".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Class selector".to_string()),
        insert_text: Some(".$1 {\n    $0\n}".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    });

    items.push(CompletionItem {
        label: "#".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("ID selector".to_string()),
        insert_text: Some("#$1 {\n    $0\n}".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    });

    items.push(CompletionItem {
        label: "*".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Universal selector".to_string()),
        insert_text: Some("* {\n    $0\n}".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    });

    items
}

/// Get class selector completions from the document
fn get_class_selector_completions(doc: &Document) -> Vec<CompletionItem> {
    let text = doc.get_text();
    let mut classes = std::collections::HashSet::new();

    // Find all class selectors in the document
    let re = regex::Regex::new(r"\.([a-zA-Z_][\w-]*)").unwrap();
    for cap in re.captures_iter(&text) {
        if let Some(m) = cap.get(1) {
            classes.insert(m.as_str().to_string());
        }
    }

    classes
        .into_iter()
        .map(|class| CompletionItem {
            label: class.clone(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("Class selector".to_string()),
            ..Default::default()
        })
        .collect()
}

/// Get ID selector completions from the document
fn get_id_selector_completions(doc: &Document) -> Vec<CompletionItem> {
    let text = doc.get_text();
    let mut ids = std::collections::HashSet::new();

    // Find all ID selectors in the document
    let re = regex::Regex::new(r"#([a-zA-Z_][\w-]*)").unwrap();
    for cap in re.captures_iter(&text) {
        if let Some(m) = cap.get(1) {
            ids.insert(m.as_str().to_string());
        }
    }

    ids.into_iter()
        .map(|id| CompletionItem {
            label: id.clone(),
            kind: Some(CompletionItemKind::REFERENCE),
            detail: Some("ID selector".to_string()),
            ..Default::default()
        })
        .collect()
}

/// Get pseudo-class completions
fn get_pseudo_class_completions() -> Vec<CompletionItem> {
    USS_PSEUDO_CLASSES
        .iter()
        .map(|pc| CompletionItem {
            label: pc.name.to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Pseudo-class".to_string()),
            documentation: Some(Documentation::String(pc.description.to_string())),
            ..Default::default()
        })
        .collect()
}

/// Get property name completions
fn get_property_name_completions() -> Vec<CompletionItem> {
    USS_PROPERTIES
        .iter()
        .map(|(name, prop)| CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::PROPERTY),
            detail: Some(prop.syntax.to_string()),
            documentation: Some(Documentation::MarkupContent(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    "{}\n\n**Initial:** `{}`\n\n**Inherited:** {}",
                    prop.description,
                    prop.initial,
                    if prop.inherited { "Yes" } else { "No" }
                ),
            })),
            insert_text: Some(format!("{}: $0;", name)),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
}

/// Get property value completions
fn get_property_value_completions(property_name: &str) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Get property-specific values
    if let Some(prop) = USS_PROPERTIES.get(property_name) {
        for value in &prop.values {
            items.push(CompletionItem {
                label: value.to_string(),
                kind: Some(CompletionItemKind::VALUE),
                detail: Some(format!("Value for {}", property_name)),
                ..Default::default()
            });
        }
    }

    // Add color completions for color properties
    if property_name.contains("color") || property_name == "background-color" {
        for (name, hex) in USS_COLORS {
            items.push(CompletionItem {
                label: name.to_string(),
                kind: Some(CompletionItemKind::COLOR),
                detail: Some(hex.to_string()),
                documentation: Some(Documentation::String(format!("Color: {}", hex))),
                ..Default::default()
            });
        }

        // Add rgb/rgba snippet
        items.push(CompletionItem {
            label: "rgb()".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text: Some("rgb(${1:0}, ${2:0}, ${3:0})".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });

        items.push(CompletionItem {
            label: "rgba()".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text: Some("rgba(${1:0}, ${2:0}, ${3:0}, ${4:1})".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });
    }

    // Add url/resource for background-image and font properties
    if property_name.contains("image")
        || property_name.contains("font")
        || property_name == "cursor"
    {
        items.push(CompletionItem {
            label: "url()".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text: Some("url(\"$1\")".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });

        items.push(CompletionItem {
            label: "resource()".to_string(),
            kind: Some(CompletionItemKind::FUNCTION),
            insert_text: Some("resource(\"$1\")".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });
    }

    // Add var() for variable reference
    items.push(CompletionItem {
        label: "var()".to_string(),
        kind: Some(CompletionItemKind::FUNCTION),
        insert_text: Some("var(--$1)".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..Default::default()
    });

    // Add unit completions for numeric properties
    if property_name.contains("width")
        || property_name.contains("height")
        || property_name.contains("margin")
        || property_name.contains("padding")
        || property_name.contains("size")
        || property_name.contains("radius")
        || property_name.contains("spacing")
    {
        for (unit, desc) in USS_UNITS {
            if *unit == "deg" || *unit == "rad" || *unit == "turn" {
                continue; // Skip angle units for size properties
            }
            items.push(CompletionItem {
                label: format!("0{}", unit),
                kind: Some(CompletionItemKind::UNIT),
                detail: Some(desc.to_string()),
                insert_text: Some(format!("${{1:0}}{}", unit)),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }
    }

    // Add angle units for rotation
    if property_name == "rotate" {
        for (unit, desc) in USS_UNITS {
            if *unit == "deg" || *unit == "rad" || *unit == "turn" {
                items.push(CompletionItem {
                    label: format!("0{}", unit),
                    kind: Some(CompletionItemKind::UNIT),
                    detail: Some(desc.to_string()),
                    insert_text: Some(format!("${{1:0}}{}", unit)),
                    insert_text_format: Some(InsertTextFormat::SNIPPET),
                    ..Default::default()
                });
            }
        }
    }

    // Add time units for transitions
    if property_name.contains("duration") || property_name.contains("delay") {
        items.push(CompletionItem {
            label: "0s".to_string(),
            kind: Some(CompletionItemKind::UNIT),
            detail: Some("Seconds".to_string()),
            insert_text: Some("${1:0}s".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });
        items.push(CompletionItem {
            label: "0ms".to_string(),
            kind: Some(CompletionItemKind::UNIT),
            detail: Some("Milliseconds".to_string()),
            insert_text: Some("${1:0}ms".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        });
    }

    items
}

/// Get URL/path completions
fn get_url_completions() -> Vec<CompletionItem> {
    // This would ideally search the Assets folder for actual files
    // For now, provide placeholder suggestions
    vec![
        CompletionItem {
            label: "Assets/".to_string(),
            kind: Some(CompletionItemKind::FOLDER),
            detail: Some("Assets folder".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "project://".to_string(),
            kind: Some(CompletionItemKind::REFERENCE),
            detail: Some("Project-relative path".to_string()),
            ..Default::default()
        },
    ]
}

/// Get variable completions from the document
fn get_variable_completions(doc: &Document) -> Vec<CompletionItem> {
    let text = doc.get_text();
    let mut vars = std::collections::HashSet::new();

    // Find all variable definitions (--var-name: value;)
    let re = regex::Regex::new(r"(--[\w-]+)\s*:").unwrap();
    for cap in re.captures_iter(&text) {
        if let Some(m) = cap.get(1) {
            vars.insert(m.as_str().to_string());
        }
    }

    vars.into_iter()
        .map(|var| CompletionItem {
            label: var.clone(),
            kind: Some(CompletionItemKind::VARIABLE),
            detail: Some("USS variable".to_string()),
            ..Default::default()
        })
        .collect()
}

/// Resolve additional completion item details
pub fn resolve_completion(mut item: CompletionItem) -> CompletionItem {
    // Add more detailed documentation for specific items
    if let Some(kind) = item.kind {
        if kind == CompletionItemKind::PROPERTY {
            if let Some(prop) = USS_PROPERTIES.get(item.label.as_str()) {
                item.documentation = Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!(
                        "## {}\n\n{}\n\n**Syntax:** `{}`\n\n**Initial value:** `{}`\n\n**Inherited:** {}",
                        prop.name,
                        prop.description,
                        prop.syntax,
                        prop.initial,
                        if prop.inherited { "Yes" } else { "No" }
                    ),
                }));
            }
        } else if kind == CompletionItemKind::CLASS {
            // Add element documentation
            if let Some(elem) = UXML_ELEMENTS.iter().find(|e| e.name == item.label) {
                item.documentation = Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format!(
                        "## {}\n\n{}\n\n**Namespace:** `{}`",
                        elem.name, elem.description, elem.namespace
                    ),
                }));
            }
        }
    }

    item
}
