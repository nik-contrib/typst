#[rustfmt::skip] mod a { use crate::text::tree_sitter::LanguageData; pub fn langs() -> std::collections::HashMap<&'static str, LanguageData> {
std::collections::HashMap::from([

    (
        "rust",
        LanguageData {
            highlights_query: include_str!("../../../../checkouts/helix/runtime/queries/rust/highlights.scm"),
            injections_query: include_str!("../../../../checkouts/helix/runtime/queries/rust/injections.scm"),
            locals_query: include_str!("../../../../checkouts/helix/runtime/queries/rust/locals.scm"),
            wasm: include_bytes!("../../../../checkouts/tree-sitter-rust/grammar.wasm"),
        },
    ),

    (
        "comment",
        LanguageData {
            highlights_query: include_str!("../../../../checkouts/helix/runtime/queries/comment/highlights.scm"),
            injections_query: include_str!("../../../../checkouts/helix/runtime/queries/comment/injections.scm"),
            locals_query: include_str!("../../../../checkouts/helix/runtime/queries/comment/locals.scm"),
            wasm: include_bytes!("../../../../checkouts/tree-sitter-comment/grammar.wasm"),
        },
    ),
])}
pub static THEME: std::sync::LazyLock<
    std::collections::HashMap<String, syntect::highlighting::Style>,
> = std::sync::LazyLock::new(|| std::collections::HashMap::from([
    ("rainbow.0".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 243, g: 139, b: 168, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("rainbow.1".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 250, g: 179, b: 135, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("rainbow.2".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 249, g: 226, b: 175, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("rainbow.3".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 166, g: 227, b: 161, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("rainbow.4".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 116, g: 199, b: 236, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("rainbow.5".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 180, g: 190, b: 254, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("attribute".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 249, g: 226, b: 175, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("comment".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 147, g: 153, b: 178, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::ITALIC
    ,}),
    ("constant".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 250, g: 179, b: 135, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("constant.character".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 148, g: 226, b: 213, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("constant.character.escape".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 245, g: 194, b: 231, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("constructor".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 116, g: 199, b: 236, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("diagnostic.error".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::UNDERLINE
    ,}),
    ("diagnostic.hint".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::UNDERLINE
    ,}),
    ("diagnostic.info".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::UNDERLINE
    ,}),
    ("diagnostic.unnecessary".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("diagnostic.warning".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::UNDERLINE
    ,}),
    ("diff.delta".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 137, g: 180, b: 250, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("diff.minus".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 243, g: 139, b: 168, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("diff.plus".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 166, g: 227, b: 161, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("error".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 243, g: 139, b: 168, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("function".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 137, g: 180, b: 250, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("function.macro".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 203, g: 166, b: 247, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("hint".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 148, g: 226, b: 213, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("info".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 137, g: 220, b: 235, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("keyword".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 203, g: 166, b: 247, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("keyword.control.conditional".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 203, g: 166, b: 247, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::ITALIC
    ,}),
    ("label".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 116, g: 199, b: 236, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.bold".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 243, g: 139, b: 168, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::BOLD
    ,}),
    ("markup.heading.1".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 243, g: 139, b: 168, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.heading.2".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 250, g: 179, b: 135, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.heading.3".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 249, g: 226, b: 175, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.heading.4".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 166, g: 227, b: 161, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.heading.5".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 116, g: 199, b: 236, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.heading.6".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 180, g: 190, b: 254, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.italic".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 243, g: 139, b: 168, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::ITALIC
    ,}),
    ("markup.link.label".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 116, g: 199, b: 236, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.link.text".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 180, g: 190, b: 254, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.link.url".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 137, g: 180, b: 250, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::UNDERLINE
| syntect::highlighting::FontStyle::ITALIC
    ,}),
    ("markup.list".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 148, g: 226, b: 213, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.list.checked".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 166, g: 227, b: 161, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.list.unchecked".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 147, g: 153, b: 178, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.quote".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 245, g: 194, b: 231, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("markup.raw".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 166, g: 227, b: 161, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("namespace".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 249, g: 226, b: 175, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::ITALIC
    ,}),
    ("operator".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 137, g: 220, b: 235, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("punctuation".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 147, g: 153, b: 178, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("punctuation.special".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 137, g: 220, b: 235, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("special".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 137, g: 180, b: 250, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("string".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 166, g: 227, b: 161, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("string.regexp".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 245, g: 194, b: 231, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("string.special".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 137, g: 180, b: 250, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("string.special.symbol".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 243, g: 139, b: 168, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("tag".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 137, g: 180, b: 250, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("type".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 249, g: 226, b: 175, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("type.enum.variant".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 148, g: 226, b: 213, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.background".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 205, g: 214, b: 244, a: 255 },
        background: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.bufferline".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 166, g: 173, b: 200, a: 255 },
        background: syntect::highlighting::Color { r: 24, g: 24, b: 37, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.bufferline.active".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 203, g: 166, b: 247, a: 255 },
        background: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::UNDERLINE
    ,}),
    ("ui.bufferline.background".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        background: syntect::highlighting::Color { r: 17, g: 17, b: 27, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.cursor".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        background: syntect::highlighting::Color { r: 181, g: 166, b: 168, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.cursor.insert".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        background: syntect::highlighting::Color { r: 126, g: 168, b: 127, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.cursor.match".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 250, g: 179, b: 135, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::BOLD
    ,}),
    ("ui.cursor.normal".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        background: syntect::highlighting::Color { r: 181, g: 166, b: 168, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.cursor.primary".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        background: syntect::highlighting::Color { r: 245, g: 224, b: 220, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.cursor.primary.insert".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        background: syntect::highlighting::Color { r: 166, g: 227, b: 161, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.cursor.primary.normal".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        background: syntect::highlighting::Color { r: 245, g: 224, b: 220, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.cursor.primary.select".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        background: syntect::highlighting::Color { r: 180, g: 190, b: 254, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.cursor.select".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        background: syntect::highlighting::Color { r: 135, g: 142, b: 192, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.cursorline.primary".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        background: syntect::highlighting::Color { r: 42, g: 43, b: 60, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.help".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 147, g: 153, b: 178, a: 255 },
        background: syntect::highlighting::Color { r: 49, g: 50, b: 68, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.highlight".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        background: syntect::highlighting::Color { r: 69, g: 71, b: 90, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::BOLD
    ,}),
    ("ui.linenr".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 69, g: 71, b: 90, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.linenr.selected".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 180, g: 190, b: 254, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.menu".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 147, g: 153, b: 178, a: 255 },
        background: syntect::highlighting::Color { r: 49, g: 50, b: 68, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.menu.selected".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 205, g: 214, b: 244, a: 255 },
        background: syntect::highlighting::Color { r: 69, g: 71, b: 90, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::BOLD
    ,}),
    ("ui.popup".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 205, g: 214, b: 244, a: 255 },
        background: syntect::highlighting::Color { r: 49, g: 50, b: 68, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.selection".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        background: syntect::highlighting::Color { r: 69, g: 71, b: 90, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.statusline".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 186, g: 194, b: 222, a: 255 },
        background: syntect::highlighting::Color { r: 24, g: 24, b: 37, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.statusline.inactive".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 88, g: 91, b: 112, a: 255 },
        background: syntect::highlighting::Color { r: 24, g: 24, b: 37, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.statusline.insert".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        background: syntect::highlighting::Color { r: 166, g: 227, b: 161, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::BOLD
    ,}),
    ("ui.statusline.normal".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        background: syntect::highlighting::Color { r: 245, g: 224, b: 220, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::BOLD
    ,}),
    ("ui.statusline.select".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 30, g: 30, b: 46, a: 255 },
        background: syntect::highlighting::Color { r: 180, g: 190, b: 254, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::BOLD
    ,}),
    ("ui.text".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 205, g: 214, b: 244, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.text.directory".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 137, g: 180, b: 250, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.text.focus".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 205, g: 214, b: 244, a: 255 },
        background: syntect::highlighting::Color { r: 49, g: 50, b: 68, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::BOLD
    ,}),
    ("ui.text.inactive".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 127, g: 132, b: 156, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.virtual".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 108, g: 112, b: 134, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.virtual.indent-guide".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 49, g: 50, b: 68, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.virtual.inlay-hint".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 69, g: 71, b: 90, a: 255 },
        background: syntect::highlighting::Color { r: 24, g: 24, b: 37, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.virtual.jump-label".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 245, g: 224, b: 220, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::BOLD
    ,}),
    ("ui.virtual.ruler".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        background: syntect::highlighting::Color { r: 49, g: 50, b: 68, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("ui.window".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 17, g: 17, b: 27, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("variable".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 205, g: 214, b: 244, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("variable.builtin".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 243, g: 139, b: 168, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("variable.other.member".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 137, g: 180, b: 250, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
    ("variable.parameter".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 235, g: 160, b: 172, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
| syntect::highlighting::FontStyle::ITALIC
    ,}),
    ("warning".to_string(), syntect::highlighting::Style {
        foreground: syntect::highlighting::Color { r: 249, g: 226, b: 175, a: 255 },
        background: syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 },
        font_style: syntect::highlighting::FontStyle::empty()
    ,}),
])); }
pub use a::*;
