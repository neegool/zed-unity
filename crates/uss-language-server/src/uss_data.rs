//! USS Data - Property, Value, and Element definitions for Unity Style Sheets
//!
//! This module contains comprehensive data about USS properties, valid values,
//! Unity UXML elements, pseudo-classes, and related documentation.

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// USS Property definition with metadata
#[derive(Debug, Clone)]
pub struct UssProperty {
    pub name: &'static str,
    pub description: &'static str,
    pub syntax: &'static str,
    pub initial: &'static str,
    pub inherited: bool,
    pub values: Vec<&'static str>,
}

/// Unity UXML element definition
#[derive(Debug, Clone)]
pub struct UxmlElement {
    pub name: &'static str,
    pub description: &'static str,
    pub namespace: &'static str,
}

/// USS pseudo-class definition
#[derive(Debug, Clone)]
pub struct UssPseudoClass {
    pub name: &'static str,
    pub description: &'static str,
}

/// All USS properties
pub static USS_PROPERTIES: Lazy<HashMap<&'static str, UssProperty>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // === Flex Layout Properties ===
    map.insert(
        "flex-direction",
        UssProperty {
            name: "flex-direction",
            description: "Specifies the direction of the main axis in the flex container.",
            syntax: "row | row-reverse | column | column-reverse",
            initial: "column",
            inherited: false,
            values: vec!["row", "row-reverse", "column", "column-reverse"],
        },
    );

    map.insert(
        "flex-wrap",
        UssProperty {
            name: "flex-wrap",
            description: "Controls whether flex items wrap to multiple lines.",
            syntax: "nowrap | wrap | wrap-reverse",
            initial: "nowrap",
            inherited: false,
            values: vec!["nowrap", "wrap", "wrap-reverse"],
        },
    );

    map.insert(
        "flex-grow",
        UssProperty {
            name: "flex-grow",
            description: "Specifies how much the item will grow relative to other flex items.",
            syntax: "<number>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "flex-shrink",
        UssProperty {
            name: "flex-shrink",
            description: "Specifies how much the item will shrink relative to other flex items.",
            syntax: "<number>",
            initial: "1",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "flex-basis",
        UssProperty {
            name: "flex-basis",
            description: "Specifies the initial main size of a flex item.",
            syntax: "<length> | <percentage> | auto",
            initial: "auto",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "align-items",
        UssProperty {
            name: "align-items",
            description: "Aligns flex items along the cross axis.",
            syntax: "auto | flex-start | center | flex-end | stretch",
            initial: "stretch",
            inherited: false,
            values: vec!["auto", "flex-start", "center", "flex-end", "stretch"],
        },
    );

    map.insert(
        "align-self",
        UssProperty {
            name: "align-self",
            description: "Overrides the align-items value for specific flex items.",
            syntax: "auto | flex-start | center | flex-end | stretch",
            initial: "auto",
            inherited: false,
            values: vec!["auto", "flex-start", "center", "flex-end", "stretch"],
        },
    );

    map.insert("align-content", UssProperty {
        name: "align-content",
        description: "Aligns flex lines within the flex container when there is extra space on the cross axis.",
        syntax: "auto | flex-start | center | flex-end | stretch",
        initial: "auto",
        inherited: false,
        values: vec!["auto", "flex-start", "center", "flex-end", "stretch", "space-between", "space-around"],
    });

    map.insert(
        "justify-content",
        UssProperty {
            name: "justify-content",
            description: "Aligns flex items along the main axis.",
            syntax: "flex-start | center | flex-end | space-between | space-around",
            initial: "flex-start",
            inherited: false,
            values: vec![
                "flex-start",
                "center",
                "flex-end",
                "space-between",
                "space-around",
            ],
        },
    );

    // === Dimension Properties ===
    map.insert(
        "width",
        UssProperty {
            name: "width",
            description: "Sets the width of an element.",
            syntax: "<length> | <percentage> | auto",
            initial: "auto",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "height",
        UssProperty {
            name: "height",
            description: "Sets the height of an element.",
            syntax: "<length> | <percentage> | auto",
            initial: "auto",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "min-width",
        UssProperty {
            name: "min-width",
            description: "Sets the minimum width of an element.",
            syntax: "<length> | <percentage> | auto",
            initial: "auto",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "min-height",
        UssProperty {
            name: "min-height",
            description: "Sets the minimum height of an element.",
            syntax: "<length> | <percentage> | auto",
            initial: "auto",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "max-width",
        UssProperty {
            name: "max-width",
            description: "Sets the maximum width of an element.",
            syntax: "<length> | <percentage> | none",
            initial: "none",
            inherited: false,
            values: vec!["none"],
        },
    );

    map.insert(
        "max-height",
        UssProperty {
            name: "max-height",
            description: "Sets the maximum height of an element.",
            syntax: "<length> | <percentage> | none",
            initial: "none",
            inherited: false,
            values: vec!["none"],
        },
    );

    // === Margin Properties ===
    map.insert(
        "margin",
        UssProperty {
            name: "margin",
            description: "Shorthand for setting all margins.",
            syntax: "<length> | <percentage> | auto",
            initial: "0",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "margin-left",
        UssProperty {
            name: "margin-left",
            description: "Sets the left margin of an element.",
            syntax: "<length> | <percentage> | auto",
            initial: "0",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "margin-right",
        UssProperty {
            name: "margin-right",
            description: "Sets the right margin of an element.",
            syntax: "<length> | <percentage> | auto",
            initial: "0",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "margin-top",
        UssProperty {
            name: "margin-top",
            description: "Sets the top margin of an element.",
            syntax: "<length> | <percentage> | auto",
            initial: "0",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "margin-bottom",
        UssProperty {
            name: "margin-bottom",
            description: "Sets the bottom margin of an element.",
            syntax: "<length> | <percentage> | auto",
            initial: "0",
            inherited: false,
            values: vec!["auto"],
        },
    );

    // === Padding Properties ===
    map.insert(
        "padding",
        UssProperty {
            name: "padding",
            description: "Shorthand for setting all padding.",
            syntax: "<length> | <percentage>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "padding-left",
        UssProperty {
            name: "padding-left",
            description: "Sets the left padding of an element.",
            syntax: "<length> | <percentage>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "padding-right",
        UssProperty {
            name: "padding-right",
            description: "Sets the right padding of an element.",
            syntax: "<length> | <percentage>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "padding-top",
        UssProperty {
            name: "padding-top",
            description: "Sets the top padding of an element.",
            syntax: "<length> | <percentage>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "padding-bottom",
        UssProperty {
            name: "padding-bottom",
            description: "Sets the bottom padding of an element.",
            syntax: "<length> | <percentage>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    // === Border Properties ===
    map.insert(
        "border-width",
        UssProperty {
            name: "border-width",
            description: "Sets the width of all borders.",
            syntax: "<length>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-left-width",
        UssProperty {
            name: "border-left-width",
            description: "Sets the width of the left border.",
            syntax: "<length>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-right-width",
        UssProperty {
            name: "border-right-width",
            description: "Sets the width of the right border.",
            syntax: "<length>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-top-width",
        UssProperty {
            name: "border-top-width",
            description: "Sets the width of the top border.",
            syntax: "<length>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-bottom-width",
        UssProperty {
            name: "border-bottom-width",
            description: "Sets the width of the bottom border.",
            syntax: "<length>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-color",
        UssProperty {
            name: "border-color",
            description: "Sets the color of all borders.",
            syntax: "<color>",
            initial: "black",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-left-color",
        UssProperty {
            name: "border-left-color",
            description: "Sets the color of the left border.",
            syntax: "<color>",
            initial: "black",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-right-color",
        UssProperty {
            name: "border-right-color",
            description: "Sets the color of the right border.",
            syntax: "<color>",
            initial: "black",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-top-color",
        UssProperty {
            name: "border-top-color",
            description: "Sets the color of the top border.",
            syntax: "<color>",
            initial: "black",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-bottom-color",
        UssProperty {
            name: "border-bottom-color",
            description: "Sets the color of the bottom border.",
            syntax: "<color>",
            initial: "black",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-radius",
        UssProperty {
            name: "border-radius",
            description: "Sets the radius of all corners.",
            syntax: "<length>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-top-left-radius",
        UssProperty {
            name: "border-top-left-radius",
            description: "Sets the radius of the top-left corner.",
            syntax: "<length>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-top-right-radius",
        UssProperty {
            name: "border-top-right-radius",
            description: "Sets the radius of the top-right corner.",
            syntax: "<length>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-bottom-left-radius",
        UssProperty {
            name: "border-bottom-left-radius",
            description: "Sets the radius of the bottom-left corner.",
            syntax: "<length>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "border-bottom-right-radius",
        UssProperty {
            name: "border-bottom-right-radius",
            description: "Sets the radius of the bottom-right corner.",
            syntax: "<length>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    // === Position Properties ===
    map.insert(
        "position",
        UssProperty {
            name: "position",
            description: "Specifies the positioning method.",
            syntax: "relative | absolute",
            initial: "relative",
            inherited: false,
            values: vec!["relative", "absolute"],
        },
    );

    map.insert(
        "left",
        UssProperty {
            name: "left",
            description: "Sets the left offset for positioned elements.",
            syntax: "<length> | <percentage> | auto",
            initial: "auto",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "right",
        UssProperty {
            name: "right",
            description: "Sets the right offset for positioned elements.",
            syntax: "<length> | <percentage> | auto",
            initial: "auto",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "top",
        UssProperty {
            name: "top",
            description: "Sets the top offset for positioned elements.",
            syntax: "<length> | <percentage> | auto",
            initial: "auto",
            inherited: false,
            values: vec!["auto"],
        },
    );

    map.insert(
        "bottom",
        UssProperty {
            name: "bottom",
            description: "Sets the bottom offset for positioned elements.",
            syntax: "<length> | <percentage> | auto",
            initial: "auto",
            inherited: false,
            values: vec!["auto"],
        },
    );

    // === Text Properties ===
    map.insert(
        "color",
        UssProperty {
            name: "color",
            description: "Sets the text color.",
            syntax: "<color>",
            initial: "black",
            inherited: true,
            values: vec![],
        },
    );

    map.insert(
        "font-size",
        UssProperty {
            name: "font-size",
            description: "Sets the font size.",
            syntax: "<length>",
            initial: "12px",
            inherited: true,
            values: vec![],
        },
    );

    map.insert(
        "-unity-font",
        UssProperty {
            name: "-unity-font",
            description: "Sets the font asset (legacy).",
            syntax: "resource(<path>) | url(<path>)",
            initial: "none",
            inherited: true,
            values: vec!["none"],
        },
    );

    map.insert(
        "-unity-font-definition",
        UssProperty {
            name: "-unity-font-definition",
            description: "Sets the font asset.",
            syntax: "resource(<path>) | url(<path>)",
            initial: "none",
            inherited: true,
            values: vec!["none"],
        },
    );

    map.insert(
        "-unity-font-style",
        UssProperty {
            name: "-unity-font-style",
            description: "Sets the font style.",
            syntax: "normal | bold | italic | bold-and-italic",
            initial: "normal",
            inherited: true,
            values: vec!["normal", "bold", "italic", "bold-and-italic"],
        },
    );

    map.insert("-unity-text-align", UssProperty {
        name: "-unity-text-align",
        description: "Sets the text alignment.",
        syntax: "upper-left | middle-left | lower-left | upper-center | middle-center | lower-center | upper-right | middle-right | lower-right",
        initial: "upper-left",
        inherited: true,
        values: vec!["upper-left", "middle-left", "lower-left", "upper-center", "middle-center", "lower-center", "upper-right", "middle-right", "lower-right"],
    });

    map.insert(
        "-unity-text-outline-width",
        UssProperty {
            name: "-unity-text-outline-width",
            description: "Sets the text outline width.",
            syntax: "<length>",
            initial: "0",
            inherited: true,
            values: vec![],
        },
    );

    map.insert(
        "-unity-text-outline-color",
        UssProperty {
            name: "-unity-text-outline-color",
            description: "Sets the text outline color.",
            syntax: "<color>",
            initial: "black",
            inherited: true,
            values: vec![],
        },
    );

    map.insert(
        "white-space",
        UssProperty {
            name: "white-space",
            description: "Specifies how white space is handled.",
            syntax: "normal | nowrap | pre | pre-wrap",
            initial: "normal",
            inherited: true,
            values: vec!["normal", "nowrap", "pre", "pre-wrap"],
        },
    );

    map.insert(
        "text-overflow",
        UssProperty {
            name: "text-overflow",
            description: "Specifies how overflowed text is handled.",
            syntax: "clip | ellipsis",
            initial: "clip",
            inherited: false,
            values: vec!["clip", "ellipsis"],
        },
    );

    map.insert(
        "letter-spacing",
        UssProperty {
            name: "letter-spacing",
            description: "Sets the spacing between characters.",
            syntax: "<length>",
            initial: "0",
            inherited: true,
            values: vec![],
        },
    );

    map.insert(
        "word-spacing",
        UssProperty {
            name: "word-spacing",
            description: "Sets the spacing between words.",
            syntax: "<length>",
            initial: "0",
            inherited: true,
            values: vec![],
        },
    );

    map.insert(
        "-unity-paragraph-spacing",
        UssProperty {
            name: "-unity-paragraph-spacing",
            description: "Sets the spacing between paragraphs.",
            syntax: "<length>",
            initial: "0",
            inherited: true,
            values: vec![],
        },
    );

    // === Background Properties ===
    map.insert(
        "background-color",
        UssProperty {
            name: "background-color",
            description: "Sets the background color.",
            syntax: "<color>",
            initial: "transparent",
            inherited: false,
            values: vec!["transparent"],
        },
    );

    map.insert(
        "background-image",
        UssProperty {
            name: "background-image",
            description: "Sets the background image.",
            syntax: "resource(<path>) | url(<path>) | none",
            initial: "none",
            inherited: false,
            values: vec!["none"],
        },
    );

    map.insert(
        "-unity-background-scale-mode",
        UssProperty {
            name: "-unity-background-scale-mode",
            description: "Sets how the background image is scaled.",
            syntax: "stretch-to-fill | scale-and-crop | scale-to-fit",
            initial: "stretch-to-fill",
            inherited: false,
            values: vec!["stretch-to-fill", "scale-and-crop", "scale-to-fit"],
        },
    );

    map.insert(
        "-unity-background-image-tint-color",
        UssProperty {
            name: "-unity-background-image-tint-color",
            description: "Sets the tint color for the background image.",
            syntax: "<color>",
            initial: "white",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "-unity-slice-left",
        UssProperty {
            name: "-unity-slice-left",
            description: "Sets the left slice for 9-slice scaling.",
            syntax: "<integer>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "-unity-slice-right",
        UssProperty {
            name: "-unity-slice-right",
            description: "Sets the right slice for 9-slice scaling.",
            syntax: "<integer>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "-unity-slice-top",
        UssProperty {
            name: "-unity-slice-top",
            description: "Sets the top slice for 9-slice scaling.",
            syntax: "<integer>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "-unity-slice-bottom",
        UssProperty {
            name: "-unity-slice-bottom",
            description: "Sets the bottom slice for 9-slice scaling.",
            syntax: "<integer>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "-unity-slice-scale",
        UssProperty {
            name: "-unity-slice-scale",
            description: "Sets the scale for 9-slice scaling.",
            syntax: "<number>",
            initial: "1",
            inherited: false,
            values: vec![],
        },
    );

    // === Visual Properties ===
    map.insert(
        "opacity",
        UssProperty {
            name: "opacity",
            description: "Sets the opacity level.",
            syntax: "<number>",
            initial: "1",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "visibility",
        UssProperty {
            name: "visibility",
            description: "Sets the visibility.",
            syntax: "visible | hidden",
            initial: "visible",
            inherited: true,
            values: vec!["visible", "hidden"],
        },
    );

    map.insert(
        "display",
        UssProperty {
            name: "display",
            description: "Sets the display type.",
            syntax: "flex | none",
            initial: "flex",
            inherited: false,
            values: vec!["flex", "none"],
        },
    );

    map.insert(
        "overflow",
        UssProperty {
            name: "overflow",
            description: "Specifies how overflow is handled.",
            syntax: "visible | hidden | scroll",
            initial: "visible",
            inherited: false,
            values: vec!["visible", "hidden", "scroll"],
        },
    );

    // === Transform Properties ===
    map.insert(
        "rotate",
        UssProperty {
            name: "rotate",
            description: "Sets the rotation.",
            syntax: "<angle>",
            initial: "0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "scale",
        UssProperty {
            name: "scale",
            description: "Sets the scale.",
            syntax: "<number> | <number> <number> | <number> <number> <number>",
            initial: "1 1 1",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "translate",
        UssProperty {
            name: "translate",
            description: "Sets the translation.",
            syntax: "<length> | <length> <length> | <length> <length> <length>",
            initial: "0 0 0",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "transform-origin",
        UssProperty {
            name: "transform-origin",
            description: "Sets the origin for transformations.",
            syntax: "<length> | <percentage> | left | center | right | top | bottom",
            initial: "center",
            inherited: false,
            values: vec!["left", "center", "right", "top", "bottom"],
        },
    );

    // === Transition Properties ===
    map.insert(
        "transition-property",
        UssProperty {
            name: "transition-property",
            description: "Specifies which properties to transition.",
            syntax: "<property-name> | all | none",
            initial: "all",
            inherited: false,
            values: vec!["all", "none"],
        },
    );

    map.insert(
        "transition-duration",
        UssProperty {
            name: "transition-duration",
            description: "Sets the duration of the transition.",
            syntax: "<time>",
            initial: "0s",
            inherited: false,
            values: vec![],
        },
    );

    map.insert(
        "transition-timing-function",
        UssProperty {
            name: "transition-timing-function",
            description: "Sets the timing function for the transition.",
            syntax: "ease | linear | ease-in | ease-out | ease-in-out",
            initial: "ease",
            inherited: false,
            values: vec!["ease", "linear", "ease-in", "ease-out", "ease-in-out"],
        },
    );

    map.insert(
        "transition-delay",
        UssProperty {
            name: "transition-delay",
            description: "Sets the delay before the transition starts.",
            syntax: "<time>",
            initial: "0s",
            inherited: false,
            values: vec![],
        },
    );

    // === Cursor Properties ===
    map.insert(
        "cursor",
        UssProperty {
            name: "cursor",
            description: "Sets the cursor type.",
            syntax: "resource(<path>) | url(<path>) | <cursor-type>",
            initial: "arrow",
            inherited: true,
            values: vec![
                "arrow",
                "text",
                "resize-vertical",
                "resize-horizontal",
                "link",
                "slide-arrow",
                "resize-up-right",
                "resize-up-left",
                "move-arrow",
                "rotate-arrow",
                "scale-arrow",
                "arrow-plus",
                "arrow-minus",
                "pan",
                "orbit",
                "zoom",
                "fps",
                "split-resize-up-down",
                "split-resize-left-right",
            ],
        },
    );

    // === Other Properties ===
    map.insert(
        "-unity-overflow-clip-box",
        UssProperty {
            name: "-unity-overflow-clip-box",
            description: "Sets the clipping box for overflow.",
            syntax: "padding-box | content-box",
            initial: "padding-box",
            inherited: false,
            values: vec!["padding-box", "content-box"],
        },
    );

    map
});

/// Unity UXML elements
pub static UXML_ELEMENTS: Lazy<Vec<UxmlElement>> = Lazy::new(|| {
    vec![
        // Core elements
        UxmlElement {
            name: "VisualElement",
            description: "The base class for all visual elements.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "BindableElement",
            description: "A visual element that can be bound to a property.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Box",
            description: "A container for grouping elements.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "TextElement",
            description: "The base class for text elements.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Label",
            description: "A text label.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Image",
            description: "Displays an image.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "IMGUIContainer",
            description: "A container for IMGUI content.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Foldout",
            description: "A collapsible container.",
            namespace: "UnityEngine.UIElements",
        },
        // Containers
        UxmlElement {
            name: "ScrollView",
            description: "A scrollable container.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "ListView",
            description: "A virtualized list view.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "TreeView",
            description: "A tree view for hierarchical data.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "MultiColumnListView",
            description: "A multi-column list view.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "MultiColumnTreeView",
            description: "A multi-column tree view.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "GroupBox",
            description: "A container with a title.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "TwoPaneSplitView",
            description: "A split view with two panes.",
            namespace: "UnityEngine.UIElements",
        },
        // Controls
        UxmlElement {
            name: "Button",
            description: "A clickable button.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "RepeatButton",
            description: "A button that repeats its action.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Toggle",
            description: "A checkbox toggle.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Scroller",
            description: "A scrollbar control.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Slider",
            description: "A slider for float values.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "SliderInt",
            description: "A slider for integer values.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "MinMaxSlider",
            description: "A slider for selecting a range.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "ProgressBar",
            description: "A progress bar.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "DropdownField",
            description: "A dropdown selection field.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "EnumField",
            description: "A dropdown for enum values.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "EnumFlagsField",
            description: "A field for enum flags.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "RadioButton",
            description: "A radio button.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "RadioButtonGroup",
            description: "A group of radio buttons.",
            namespace: "UnityEngine.UIElements",
        },
        // Text input
        UxmlElement {
            name: "TextField",
            description: "A text input field.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "IntegerField",
            description: "An input field for integers.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "LongField",
            description: "An input field for long integers.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "FloatField",
            description: "An input field for floats.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "DoubleField",
            description: "An input field for doubles.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Vector2Field",
            description: "An input field for Vector2.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Vector3Field",
            description: "An input field for Vector3.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Vector4Field",
            description: "An input field for Vector4.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Vector2IntField",
            description: "An input field for Vector2Int.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Vector3IntField",
            description: "An input field for Vector3Int.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "RectField",
            description: "An input field for Rect.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "RectIntField",
            description: "An input field for RectInt.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "BoundsField",
            description: "An input field for Bounds.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "BoundsIntField",
            description: "An input field for BoundsInt.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Hash128Field",
            description: "An input field for Hash128.",
            namespace: "UnityEngine.UIElements",
        },
        // Complex fields (Editor only)
        UxmlElement {
            name: "ColorField",
            description: "A color picker field.",
            namespace: "UnityEditor.UIElements",
        },
        UxmlElement {
            name: "CurveField",
            description: "An animation curve field.",
            namespace: "UnityEditor.UIElements",
        },
        UxmlElement {
            name: "GradientField",
            description: "A gradient field.",
            namespace: "UnityEditor.UIElements",
        },
        UxmlElement {
            name: "ObjectField",
            description: "A field for Unity objects.",
            namespace: "UnityEditor.UIElements",
        },
        UxmlElement {
            name: "PropertyField",
            description: "A field for serialized properties.",
            namespace: "UnityEditor.UIElements",
        },
        UxmlElement {
            name: "LayerField",
            description: "A layer selection field.",
            namespace: "UnityEditor.UIElements",
        },
        UxmlElement {
            name: "LayerMaskField",
            description: "A layer mask field.",
            namespace: "UnityEditor.UIElements",
        },
        UxmlElement {
            name: "MaskField",
            description: "A mask field.",
            namespace: "UnityEditor.UIElements",
        },
        UxmlElement {
            name: "TagField",
            description: "A tag selection field.",
            namespace: "UnityEditor.UIElements",
        },
        // Templates
        UxmlElement {
            name: "Template",
            description: "A UXML template reference.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "TemplateContainer",
            description: "A container for template instances.",
            namespace: "UnityEngine.UIElements",
        },
        UxmlElement {
            name: "Instance",
            description: "An instance of a template.",
            namespace: "UnityEngine.UIElements",
        },
    ]
});

/// USS pseudo-classes
pub static USS_PSEUDO_CLASSES: Lazy<Vec<UssPseudoClass>> = Lazy::new(|| {
    vec![
        UssPseudoClass {
            name: "hover",
            description: "Applied when the mouse is over the element.",
        },
        UssPseudoClass {
            name: "active",
            description: "Applied when the element is being activated (clicked).",
        },
        UssPseudoClass {
            name: "focus",
            description: "Applied when the element has focus.",
        },
        UssPseudoClass {
            name: "disabled",
            description: "Applied when the element is disabled.",
        },
        UssPseudoClass {
            name: "enabled",
            description: "Applied when the element is enabled.",
        },
        UssPseudoClass {
            name: "checked",
            description: "Applied when a toggle is checked.",
        },
        UssPseudoClass {
            name: "selected",
            description: "Applied when the element is selected.",
        },
        UssPseudoClass {
            name: "root",
            description: "Applied to the root element.",
        },
        UssPseudoClass {
            name: "first-child",
            description: "Applied to the first child of its parent.",
        },
        UssPseudoClass {
            name: "last-child",
            description: "Applied to the last child of its parent.",
        },
    ]
});

/// Common USS units
pub static USS_UNITS: &[(&str, &str)] = &[
    ("px", "Pixels"),
    ("%", "Percentage"),
    ("em", "Relative to font size"),
    ("rem", "Relative to root font size"),
    ("vw", "Viewport width"),
    ("vh", "Viewport height"),
    ("deg", "Degrees"),
    ("rad", "Radians"),
    ("turn", "Turns"),
    ("s", "Seconds"),
    ("ms", "Milliseconds"),
];

/// Named colors supported in USS
pub static USS_COLORS: &[(&str, &str)] = &[
    ("transparent", "#00000000"),
    ("black", "#000000"),
    ("white", "#FFFFFF"),
    ("red", "#FF0000"),
    ("green", "#008000"),
    ("blue", "#0000FF"),
    ("yellow", "#FFFF00"),
    ("cyan", "#00FFFF"),
    ("magenta", "#FF00FF"),
    ("gray", "#808080"),
    ("grey", "#808080"),
    ("silver", "#C0C0C0"),
    ("maroon", "#800000"),
    ("olive", "#808000"),
    ("lime", "#00FF00"),
    ("aqua", "#00FFFF"),
    ("teal", "#008080"),
    ("navy", "#000080"),
    ("fuchsia", "#FF00FF"),
    ("purple", "#800080"),
    ("orange", "#FFA500"),
];

#[allow(dead_code)]
/// Get property completions
pub fn get_property_names() -> Vec<&'static str> {
    USS_PROPERTIES.keys().copied().collect()
}

#[allow(dead_code)]
/// Get element type names
pub fn get_element_names() -> Vec<&'static str> {
    UXML_ELEMENTS.iter().map(|e| e.name).collect()
}

#[allow(dead_code)]
/// Get pseudo-class names
pub fn get_pseudo_class_names() -> Vec<&'static str> {
    USS_PSEUDO_CLASSES.iter().map(|p| p.name).collect()
}
