//! Hover provider for USS Language Server
//!
//! Provides hover information for USS properties, values, selectors, and elements.

use crate::document::Document;
use crate::uss_data::{USS_COLORS, USS_PROPERTIES, USS_PSEUDO_CLASSES, USS_UNITS, UXML_ELEMENTS};
use tower_lsp::lsp_types::*;

/// Get hover information at a position
pub fn get_hover(doc: &Document, position: Position) -> Option<Hover> {
    let word = doc.get_word_at_position(position)?;
    let line = doc.get_line(position.line)?;

    // Check what context we're in
    let hover_content = get_hover_content(&word, &line, position);

    hover_content.map(|content| Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: content,
        }),
        range: None,
    })
}

/// Get hover content based on the word and context
fn get_hover_content(word: &str, line: &str, position: Position) -> Option<String> {
    // Check if it's a USS property
    if let Some(prop) = USS_PROPERTIES.get(word) {
        return Some(format!(
            "## {}\n\n{}\n\n**Syntax:** `{}`\n\n**Initial:** `{}`\n\n**Inherited:** {}",
            prop.name,
            prop.description,
            prop.syntax,
            prop.initial,
            if prop.inherited { "Yes" } else { "No" }
        ));
    }

    // Check if it's a Unity element type
    if let Some(elem) = UXML_ELEMENTS.iter().find(|e| e.name == word) {
        return Some(format!(
            "## {}\n\n{}\n\n**Namespace:** `{}`\n\n[Unity Documentation](https://docs.unity3d.com/ScriptReference/UIElements.{}.html)",
            elem.name, elem.description, elem.namespace, elem.name
        ));
    }

    // Check if it's a pseudo-class
    if word.starts_with(':') || line[..position.character as usize].ends_with(':') {
        let pseudo_name = word.trim_start_matches(':');
        if let Some(pc) = USS_PSEUDO_CLASSES.iter().find(|p| p.name == pseudo_name) {
            return Some(format!("## :{}\n\n{}", pc.name, pc.description));
        }
    }

    // Check if it's a named color
    if let Some((name, hex)) = USS_COLORS.iter().find(|(n, _)| *n == word) {
        return Some(format!(
            "## Color: {}\n\n**Hex:** `{}`\n\n<div style=\"width: 50px; height: 50px; background-color: {};\"></div>",
            name, hex, hex
        ));
    }

    // Check if it's a unit
    for (unit, desc) in USS_UNITS {
        if word.ends_with(unit) {
            return Some(format!("## Unit: {}\n\n{}", unit, desc));
        }
    }

    // Check if it's a USS variable
    if word.starts_with("--") {
        return Some(format!(
            "## USS Variable\n\n`{}`\n\nCustom property (variable) defined in this stylesheet.",
            word
        ));
    }

    // Check if it's a class selector
    if let Some(class_name) = word.strip_prefix('.') {
        return Some(format!(
            "## Class Selector\n\n`{}`\n\nMatches elements with the class `{}`.",
            word, class_name
        ));
    }

    // Check if it's an ID selector
    if let Some(id_name) = word.strip_prefix('#') {
        return Some(format!(
            "## ID Selector\n\n`{}`\n\nMatches the element with name `{}`.",
            word, id_name
        ));
    }

    // Check for specific keywords
    match word {
        "flex" => Some("## `flex`\n\nSets the element to use flexbox layout.".to_string()),
        "none" => Some("## `none`\n\nRemoves/hides the element or disables a feature.".to_string()),
        "auto" => Some(
            "## `auto`\n\nAllows the browser/engine to calculate the value automatically."
                .to_string(),
        ),
        "inherit" => {
            Some("## `inherit`\n\nInherits the value from the parent element.".to_string())
        }
        "initial" => Some("## `initial`\n\nResets to the initial/default value.".to_string()),
        "transparent" => {
            Some("## `transparent`\n\nFully transparent color (`rgba(0, 0, 0, 0)`).".to_string())
        }

        // Flex values
        "row" => Some("## `row`\n\nFlex items are laid out horizontally.".to_string()),
        "column" => Some("## `column`\n\nFlex items are laid out vertically.".to_string()),
        "row-reverse" => Some(
            "## `row-reverse`\n\nFlex items are laid out horizontally in reverse order."
                .to_string(),
        ),
        "column-reverse" => Some(
            "## `column-reverse`\n\nFlex items are laid out vertically in reverse order."
                .to_string(),
        ),
        "wrap" => Some("## `wrap`\n\nFlex items wrap to multiple lines.".to_string()),
        "nowrap" => Some("## `nowrap`\n\nFlex items stay on a single line.".to_string()),
        "flex-start" => {
            Some("## `flex-start`\n\nAligns items to the start of the flex container.".to_string())
        }
        "flex-end" => {
            Some("## `flex-end`\n\nAligns items to the end of the flex container.".to_string())
        }
        "center" => Some("## `center`\n\nCenters items in the flex container.".to_string()),
        "stretch" => Some("## `stretch`\n\nStretches items to fill the container.".to_string()),
        "space-between" => Some(
            "## `space-between`\n\nDistributes items evenly with space between them.".to_string(),
        ),
        "space-around" => Some(
            "## `space-around`\n\nDistributes items evenly with space around them.".to_string(),
        ),

        // Position values
        "relative" => {
            Some("## `relative`\n\nPositioned relative to its normal position.".to_string())
        }
        "absolute" => Some(
            "## `absolute`\n\nPositioned relative to the nearest positioned ancestor.".to_string(),
        ),

        // Display/visibility
        "visible" => Some("## `visible`\n\nThe element is visible.".to_string()),
        "hidden" => {
            Some("## `hidden`\n\nThe element is hidden but still takes up space.".to_string())
        }
        "scroll" => Some("## `scroll`\n\nAdds scrollbars when content overflows.".to_string()),

        // Font styles
        "normal" => Some("## `normal`\n\nNormal/default style.".to_string()),
        "bold" => Some("## `bold`\n\nBold font weight.".to_string()),
        "italic" => Some("## `italic`\n\nItalic font style.".to_string()),
        "bold-and-italic" => Some("## `bold-and-italic`\n\nBoth bold and italic.".to_string()),

        // Text alignment (Unity-specific)
        "upper-left" => Some("## `upper-left`\n\nText aligned to top-left.".to_string()),
        "middle-left" => Some("## `middle-left`\n\nText aligned to middle-left.".to_string()),
        "lower-left" => Some("## `lower-left`\n\nText aligned to bottom-left.".to_string()),
        "upper-center" => Some("## `upper-center`\n\nText aligned to top-center.".to_string()),
        "middle-center" => Some("## `middle-center`\n\nText aligned to center.".to_string()),
        "lower-center" => Some("## `lower-center`\n\nText aligned to bottom-center.".to_string()),
        "upper-right" => Some("## `upper-right`\n\nText aligned to top-right.".to_string()),
        "middle-right" => Some("## `middle-right`\n\nText aligned to middle-right.".to_string()),
        "lower-right" => Some("## `lower-right`\n\nText aligned to bottom-right.".to_string()),

        // Background scale modes
        "stretch-to-fill" => {
            Some("## `stretch-to-fill`\n\nStretches the image to fill the element.".to_string())
        }
        "scale-and-crop" => Some(
            "## `scale-and-crop`\n\nScales and crops the image to fill the element.".to_string(),
        ),
        "scale-to-fit" => {
            Some("## `scale-to-fit`\n\nScales the image to fit within the element.".to_string())
        }

        // Timing functions
        "ease" => {
            Some("## `ease`\n\nTransition with slow start, then fast, then slow end.".to_string())
        }
        "linear" => Some("## `linear`\n\nConstant speed transition.".to_string()),
        "ease-in" => Some("## `ease-in`\n\nTransition with slow start.".to_string()),
        "ease-out" => Some("## `ease-out`\n\nTransition with slow end.".to_string()),
        "ease-in-out" => {
            Some("## `ease-in-out`\n\nTransition with slow start and end.".to_string())
        }

        // White-space
        "pre" => Some("## `pre`\n\nPreserves whitespace and line breaks.".to_string()),
        "pre-wrap" => Some("## `pre-wrap`\n\nPreserves whitespace but wraps text.".to_string()),

        // Text overflow
        "clip" => Some("## `clip`\n\nClips overflowing text.".to_string()),
        "ellipsis" => {
            Some("## `ellipsis`\n\nShows ellipsis (...) for overflowing text.".to_string())
        }

        _ => None,
    }
}
