use std::{io::Write, process::Stdio};

pub fn query(s: Option<&ecow::EcoString>) -> Option<&'static str> {
    s.and_then(|token| {
        Some(match token.as_str() {
            "rs" | "rust" => "rust",
            _ => return None,
        })
    })
}

static ENGINE: std::sync::OnceLock<wasmtime::Engine> = std::sync::OnceLock::new();

pub struct LanguageData {
    pub highlights_query: &'static str,
    pub injections_query: &'static str,
    pub locals_query: &'static str,
    pub wasm: &'static [u8],
}

pub fn highlight(
    routines: &crate::routines::Routines,
    target: crate::foundations::Target,
    lines: ecow::EcoVec<(ecow::EcoString, typst_syntax::Span)>,
    seq: &mut Vec<crate::foundations::Packed<super::RawLine>>,
    foreground: syntect::highlighting::Color,
    count: i64,
    lang_name: &'static str,
) {
    // let mut parser = tree_sitter::Parser::new();
    // parser
    //     .set_wasm_store(
    //         tree_sitter::WasmStore::new(ENGINE.get_or_init(Default::default)).unwrap(),
    //     )
    //     .unwrap();
    // let mut store = parser.take_wasm_store().unwrap();

    // let mut languages: std::collections::HashMap<
    //     &'static str,
    //     tree_sitter_highlight::HighlightConfiguration,
    // > = std::collections::HashMap::new();

    // for (lang, data) in super::tree_sitter_generated::langs() {
    //     let language_id = store.load_language(lang, data.wasm).unwrap();
    //     let mut configuration = tree_sitter_highlight::HighlightConfiguration::new(
    //         language_id,
    //         lang,
    //         data.highlights_query,
    //         data.injections_query,
    //         data.locals_query,
    //     )
    //     .unwrap();
    //     configuration.configure(SCOPES);
    //     languages.insert(lang, configuration);
    // }

    // parser.set_wasm_store(store).unwrap();

    // let mut highlighter = tree_sitter_highlight::Highlighter::new();
    // highlighter.parser = parser;

    // let output = hx_highlight.wait_with_output().unwrap();

    #[derive(serde::Deserialize, Default, Clone, Copy, Debug)]
    struct HxStyle {
        fg: Option<(u8, u8, u8)>,
        bg: Option<(u8, u8, u8)>,
        underline: bool,
        italic: bool,
        bold: bool,
    }

    impl From<HxStyle> for syntect::highlighting::Style {
        fn from(style: HxStyle) -> Self {
            let mut font_style = syntect::highlighting::FontStyle::empty();
            if style.bold {
                font_style.insert(syntect::highlighting::FontStyle::BOLD);
            }
            if style.italic {
                font_style.insert(syntect::highlighting::FontStyle::ITALIC);
            }
            if style.underline {
                font_style.insert(syntect::highlighting::FontStyle::UNDERLINE);
            }

            let fg = style
                .fg
                .map(|(r, g, b)| syntect::highlighting::Color { r, g, b, a: 255 })
                .unwrap_or(syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 0 });
            let bg = style
                .bg
                .map(|(r, g, b)| syntect::highlighting::Color { r, g, b, a: 255 })
                .unwrap_or(syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 0 });
            Self { foreground: fg, background: bg, font_style }
        }
    }

    // Whole text of the code block as a single String
    let text = lines.iter().map(|(s, _)| s.clone()).collect::<Vec<_>>().join("\n");

    // hx-highlight
    //
    // Figuring out how to get the right runtime directory is hard. so i replace the actual
    // hx executable with hx-highlight under the same name
    let mut hx_highlight = std::process::Command::new("hx")
        .arg("--lang")
        .arg(lang_name)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdin = hx_highlight.stdin.as_mut().expect("failed to open stdin");
        stdin.write_all(text.as_bytes()).unwrap();
    }

    let output = hx_highlight.wait_with_output().unwrap();

    let pieces =
        serde_json::from_slice::<Vec<(String, HxStyle)>>(&output.stdout).unwrap();

    let mut chars = Vec::new();
    for (piece, style) in pieces {
        for char in piece.chars() {
            chars.push((style, char));
        }
    }

    let mut highlighted_lines = Vec::new();
    let mut current_line = Vec::new();
    for (style, char) in chars {
        if char == '\n' {
            highlighted_lines.push(current_line.clone());
            current_line.clear();
        } else {
            current_line.push((style, char));
        }
    }
    if !current_line.is_empty() {
        highlighted_lines.push(current_line);
    }
    for ((i, line), (line_string, line_span)) in
        highlighted_lines.into_iter().enumerate().zip(lines)
    {
        let mut line_content = Vec::new();
        let mut span_offset = 0;
        for (style, piece) in line {
            let piece = piece.to_string();
            line_content.push(crate::text::raw::styled(
                routines,
                target,
                &piece,
                foreground,
                style.into(),
                line_span,
                span_offset,
            ));
            span_offset += piece.len();
        }
        seq.push(
            crate::foundations::Packed::new(super::RawLine::new(
                i as i64 + 1,
                count,
                line_string,
                crate::foundations::Content::sequence(line_content),
            ))
            .spanned(line_span),
        );
    }
}

pub const SCOPES: &[&str] = &[
    "attribute",
    "type",
    "type.builtin",
    "type.parameter",
    "type.enum",
    "type.enum.variant",
    "constructor",
    "constant",
    "constant.builtin",
    "constant.builtin.boolean",
    "constant.character",
    "constant.character.escape",
    "constant.numeric",
    "constant.numeric.integer",
    "constant.numeric.float",
    "string",
    "string.regexp",
    "string.special",
    "string.special.path",
    "string.special.url",
    "string.special.symbol",
    "comment",
    "comment.line",
    "comment.line.documentation",
    "comment.block",
    "comment.block.documentation",
    "comment.unused",
    "variable",
    "variable.builtin",
    "variable.parameter",
    "variable.other",
    "variable.other.member",
    "variable.other.member.private",
    "label",
    "punctuation",
    "punctuation.delimiter",
    "punctuation.bracket",
    "punctuation.special",
    "keyword",
    "keyword.control",
    "keyword.control.conditional",
    "keyword.control.repeat",
    "keyword.control.import",
    "keyword.control.return",
    "keyword.control.exception",
    "keyword.operator",
    "keyword.directive",
    "keyword.function",
    "keyword.storage",
    "keyword.storage.type",
    "keyword.storage.modifier",
    "operator",
    "function",
    "function.builtin",
    "function.method",
    "function.method.private",
    "function.macro",
    "function.special",
    "tag",
    "tag.builtin",
    "namespace",
    "special",
    "markup",
    "markup.heading",
    "markup.heading.marker",
    "markup.heading.h1",
    "markup.heading.h2",
    "markup.heading.h3",
    "markup.heading.h4",
    "markup.heading.h5",
    "markup.heading.h6",
    "markup.list",
    "markup.list.unnumbered",
    "markup.list.numbered",
    "markup.list.checked",
    "markup.list.unchecked",
    "markup.bold",
    "markup.italic",
    "markup.strikethrough",
    "markup.link",
    "markup.link.url",
    "markup.link.label",
    "markup.link.text",
    "markup.quote",
    "markup.raw",
    "markup.raw.inline",
    "markup.raw.block",
    "diff",
    "diff.plus",
    "diff.plus.gutter",
    "diff.minus",
    "diff.minus.gutter",
    "diff.delta",
    "diff.delta.moved",
    "diff.delta.conflict",
    "diff.delta.gutter",
];
