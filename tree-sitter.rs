---
package.edition = "2024"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
toml = "0.9"
bitflags = "2.10"
syntect = "5.3"
ureq = "3.1"
anstream = "0.6"
xshell = "0.2.7"
eyre = "0.6"
color-eyre = "0.6"
rayon = "1.11"
---

use eyre::OptionExt;
use rayon::prelude::*;
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::Write;
use std::process::Command;
use std::{env, fs};
use toml::{map::Map, Value};
use xshell::Shell;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;

    fs::remove_dir_all("checkouts")?;
    fs::create_dir_all("checkouts")?;

    // 5. git clone https://github.com/helix-editor/helix.git
    Command::new("git")
        .args(["clone", "https://github.com/helix-editor/helix.git", "checkouts/helix"])
        .status()?;

    // 7. git checkout {HELIX_COMMIT}
    Command::new("git")
        .args(["-C", "checkouts/helix", "checkout", HELIX_COMMIT])
        .status()?;

    // Read the file content
    let languages_toml = std::fs::read_to_string("checkouts/helix/languages.toml")?;
    let languages_toml: LanguagesToml = toml::from_str(&languages_toml)?;

    let find_lang = |lang: &str| {
        languages_toml.grammar.iter().find_map(|g| {
            (g.name == *lang).then_some((g.source.git.clone(), g.source.rev.clone()))
        })
    };

    for lang in LANGUAGES {
        let (git, rev) = find_lang(lang).ok_or_else(|| eyre::eyre!("no lang: {lang}"))?;
        let grammar_dir = format!("checkouts/tree-sitter-{lang}");

        Command::new("git")
            .arg("clone")
            .arg(&git)
            .arg(&grammar_dir)
            .status()?;

        Command::new("git")
            .current_dir(&grammar_dir)
            .arg("checkout")
            .arg(&rev)
            .status()?;

        println!("A");
        Command::new("tree-sitter")
            .current_dir(&grammar_dir)
            .arg("build")
            .arg("--docker")
            .arg("--wasm")
            .arg("--output")
            .arg("grammar.wasm")
            .status()?;
        println!("B");
    }

    return Ok(());
    // let theme_name = std::env::args()
    //     .skip(1)
    //     .next()
    //     .expect("provide name of helix theme as argument");
    let theme_name = THEME;

    let theme_toml = load_theme(&theme_name);

    let Value::Table(table) = theme_toml else { panic!() };
    let theme = Theme::from_keys(table).0;

    let mut s = String::new();

    // writeln!(s, "static {}: std::sync::LazyLock<", theme_name);
    writeln!(s, "static THEME: std::sync::LazyLock<");
    writeln!(s, "    std::collections::HashMap<String, syntect::highlighting::Style>,");
    writeln!(s, "> = std::sync::LazyLock::new(|| std::collections::HashMap::from([");

    // The mandatory default color for fields that are missing (`None`) in the source data,
    // as the `Style` struct requires a color, and the user specified alpha is always 255.
    const DEFAULT_OPAQUE_BLACK: &str =
        "syntect::highlighting::Color { r: 0, g: 0, b: 0, a: 255 }";

    // Lambda to resolve an optional Color enum into the Rust struct string
    let resolve_color_struct = |opt_color: &Option<Color>| -> String {
        if let Some(Color::Rgb(r, g, b)) = opt_color {
            format!("syntect::highlighting::Color {{ r: {r}, g: {g}, b: {b}, a: 255 }}")
        } else {
            DEFAULT_OPAQUE_BLACK.to_string()
        }
    };

    // 2. Loop over all scopes and generate HashMap entries
    for (scope, highlight) in std::iter::zip(theme.scopes.iter(), theme.highlights.iter())
    {
        // --- Resolve Foreground and Background ---
        let fg_color_str = resolve_color_struct(&highlight.fg);
        let bg_color_str = resolve_color_struct(&highlight.bg);

        // --- Write the HashMap Entry ---
        writeln!(s, r#"    ("{scope}".to_string(), syntect::highlighting::Style {{"#);
        writeln!(s, "        foreground: {},", fg_color_str);
        writeln!(s, "        background: {},", bg_color_str);
        writeln!(s, "        font_style: syntect::highlighting::FontStyle::empty()",);

        let mut modifier = Modifier::empty();
        modifier.insert(highlight.add_modifier);
        modifier.remove(highlight.sub_modifier);

        if modifier.contains(Modifier::BOLD) {
            writeln!(s, "| syntect::highlighting::FontStyle::BOLD");
        }

        if highlight.underline_style.is_some() {
            writeln!(s, "| syntect::highlighting::FontStyle::UNDERLINE");
        }

        if modifier.contains(Modifier::ITALIC) {
            writeln!(s, "| syntect::highlighting::FontStyle::ITALIC");
        }

        writeln!(s, "    ,}}),");
    }

    // 3. Write the footer to close the HashMap and LazyLock
    writeln!(s, "]));");

    std::fs::write("tree-sitter-output.rs", s)?;

    Ok(())
}

#[derive(Deserialize)]
struct LanguagesToml {
    grammar: Vec<LanguageToml>,
}

#[derive(Deserialize)]
struct LanguageToml {
    name: String,
    source: LanguageSource,
}

#[derive(Deserialize)]
struct LanguageSource {
    /// Git repository URL
    git: String,
    /// Commit to clone the git repo at
    rev: String,
}

const HELIX_COMMIT: &str = "68c7e8757f0183adeb01ba1fbbabd1cee2885b70";
// Languages to add tree-sitter support for
const LANGUAGES: &[&str] = &["rust", "comment"];
const THEME: &str = "catppuccin_mocha";

fn merge_themes(parent_theme_toml: Value, theme_toml: Value) -> Value {
    let parent_palette = parent_theme_toml.get("palette");
    let palette = theme_toml.get("palette");

    // handle the table separately since it needs a `merge_depth` of 2
    // this would conflict with the rest of the theme merge strategy
    let palette_values = match (parent_palette, palette) {
        (Some(parent_palette), Some(palette)) => {
            merge_toml_values(parent_palette.clone(), palette.clone(), 2)
        }
        (Some(parent_palette), None) => parent_palette.clone(),
        (None, Some(palette)) => palette.clone(),
        (None, None) => Map::new().into(),
    };

    // add the palette correctly as nested table
    let mut palette = Map::new();
    palette.insert(String::from("palette"), palette_values);

    // merge the theme into the parent theme
    let theme = merge_toml_values(parent_theme_toml, theme_toml, 1);
    // merge the before specially handled palette into the theme
    merge_toml_values(theme, palette.into(), 1)
}

pub fn merge_toml_values(
    left: toml::Value,
    right: toml::Value,
    merge_depth: usize,
) -> toml::Value {
    use toml::Value;

    fn get_name(v: &Value) -> Option<&str> {
        v.get("name").and_then(Value::as_str)
    }

    match (left, right) {
        (Value::Array(mut left_items), Value::Array(right_items)) => {
            if merge_depth > 0 {
                left_items.reserve(right_items.len());
                for rvalue in right_items {
                    let lvalue = get_name(&rvalue)
                        .and_then(|rname| {
                            left_items.iter().position(|v| get_name(v) == Some(rname))
                        })
                        .map(|lpos| left_items.remove(lpos));
                    let mvalue = match lvalue {
                        Some(lvalue) => {
                            merge_toml_values(lvalue, rvalue, merge_depth - 1)
                        }
                        None => rvalue,
                    };
                    left_items.push(mvalue);
                }
                Value::Array(left_items)
            } else {
                Value::Array(right_items)
            }
        }
        (Value::Table(mut left_map), Value::Table(right_map)) => {
            if merge_depth > 0 {
                for (rname, rvalue) in right_map {
                    match left_map.remove(&rname) {
                        Some(lvalue) => {
                            let merged_value =
                                merge_toml_values(lvalue, rvalue, merge_depth - 1);
                            left_map.insert(rname, merged_value);
                        }
                        None => {
                            left_map.insert(rname, rvalue);
                        }
                    }
                }
                Value::Table(left_map)
            } else {
                Value::Table(right_map)
            }
        }
        // Catch everything else we didn't handle, and use the right value
        (_, value) => value,
    }
}

fn load_theme(theme_name: &str) -> Value {
    let theme_toml = load_theme_toml(theme_name);

    if let Some(parent_theme_name) = theme_toml.get("inherits") {
        let parent_theme_name = parent_theme_name.as_str().unwrap();
        let parent_theme_toml = load_theme(parent_theme_name);
        merge_themes(parent_theme_toml, theme_toml)
    } else {
        theme_toml
    }
}

fn load_theme_toml(theme_name: &str) -> Value {
    let url = format!(
            "https://raw.githubusercontent.com/helix-editor/helix/a79292b630ae4a0e6e37814ad21411ab50926c73/runtime/themes/{theme_name}.toml",
        );

    let theme = ureq::get(url).call().unwrap().body_mut().read_to_string().unwrap();

    let value = toml::from_str(&theme).unwrap();

    value
}

#[derive(Clone, Debug, Default)]
pub struct Theme {
    pub name: String,
    // UI styles are stored in a HashMap
    pub styles: HashMap<String, Style>,
    // tree-sitter highlight styles are stored in a Vec to optimize lookups
    pub scopes: Vec<String>,
    pub highlights: Vec<Style>,
    pub rainbow_length: usize,
}

impl Theme {
    fn from_keys(toml_keys: Map<String, Value>) -> (Self, Vec<String>) {
        let (styles, scopes, highlights, rainbow_length, load_errors) =
            build_theme_values(toml_keys);

        let theme = Self {
            styles,
            scopes,
            highlights,
            rainbow_length,
            ..Default::default()
        };
        (theme, load_errors)
    }
}

impl<'de> Deserialize<'de> for Theme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let values = toml::map::Map::<String, toml::Value>::deserialize(deserializer)?;
        let (theme, warnings) = Theme::from_keys(values);
        if !warnings.is_empty() {
            panic!("{warnings:?}");
        }
        Ok(theme)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Style {
    pub fg: Option<Color>,
    pub bg: Option<Color>,
    pub underline_color: Option<Color>,
    pub underline_style: Option<UnderlineStyle>,
    pub add_modifier: Modifier,
    pub sub_modifier: Modifier,
}

impl From<Color> for syntect::highlighting::Color {
    fn from(value: Color) -> Self {
        let Color::Rgb(r, g, b) = value else { panic!("expected `Color::Rgb`") };
        syntect::highlighting::Color { r, g, b, a: 255 }
    }
}

impl From<Style> for syntect::highlighting::Style {
    fn from(value: Style) -> Self {
        syntect::highlighting::Style {
            foreground: value.fg.unwrap_or(Color::Rgb(255, 0, 0)).into(),
            background: value.bg.unwrap_or(Color::Rgb(255, 0, 0)).into(),
            font_style: {
                let mut style = syntect::highlighting::FontStyle::default();

                if value.underline_style.is_some() {
                    style.insert(syntect::highlighting::FontStyle::UNDERLINE);
                }

                let mut modifier = Modifier::empty();
                modifier.insert(value.add_modifier);
                modifier.remove(value.sub_modifier);

                if modifier.contains(Modifier::BOLD) {
                    style.insert(syntect::highlighting::FontStyle::BOLD)
                }

                if modifier.contains(Modifier::ITALIC) {
                    style.insert(syntect::highlighting::FontStyle::ITALIC)
                }

                style
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Reset,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    Gray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    LightGray,
    White,
    Rgb(u8, u8, u8),
    Indexed(u8),
}

impl Default for Color {
    fn default() -> Self {
        Self::Rgb(0, 0, 0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnderlineStyle {
    Reset,
    Line,
    Curl,
    Dotted,
    Dashed,
    DoubleLine,
}

bitflags::bitflags! {
    /// Modifier changes the way a piece of text is displayed.
    ///
    /// They are bitflags so they can easily be composed.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use helix_view::graphics::Modifier;
    ///
    /// let m = Modifier::BOLD | Modifier::ITALIC;
    /// ```
    #[derive(PartialEq, Eq, Debug, Clone, Copy)]
    pub struct Modifier: u16 {
        const BOLD              = 0b0000_0000_0001;
        const DIM               = 0b0000_0000_0010;
        const ITALIC            = 0b0000_0000_0100;
        const SLOW_BLINK        = 0b0000_0001_0000;
        const RAPID_BLINK       = 0b0000_0010_0000;
        const REVERSED          = 0b0000_0100_0000;
        const HIDDEN            = 0b0000_1000_0000;
        const CROSSED_OUT       = 0b0001_0000_0000;
    }
}

#[allow(clippy::type_complexity)]
fn build_theme_values(
    mut values: Map<String, Value>,
) -> (HashMap<String, Style>, Vec<String>, Vec<Style>, usize, Vec<String>) {
    let mut styles = HashMap::new();
    let mut scopes = Vec::new();
    let mut highlights = Vec::new();
    let mut rainbow_length = 0;

    let mut warnings = Vec::new();

    // TODO: alert user of parsing failures in editor
    let palette = values
        .remove("palette")
        .map(|value| {
            ThemePalette::try_from(value).unwrap_or_else(|err| {
                warnings.push(err);
                ThemePalette::default()
            })
        })
        .unwrap_or_default();
    // remove inherits from value to prevent errors
    let _ = values.remove("inherits");
    styles.reserve(values.len());
    scopes.reserve(values.len());
    highlights.reserve(values.len());

    for (i, style) in values
        .remove("rainbow")
        .and_then(|value| match palette.parse_style_array(value) {
            Ok(styles) => Some(styles),
            Err(err) => {
                warnings.push(err);
                None
            }
        })
        .unwrap_or_else(default_rainbow)
        .into_iter()
        .enumerate()
    {
        let name = format!("rainbow.{i}");
        styles.insert(name.clone(), style);
        scopes.push(name);
        highlights.push(style);
        rainbow_length += 1;
    }

    for (name, style_value) in values {
        let mut style = Style::default();
        if let Err(err) = palette.parse_style(&mut style, style_value) {
            warnings.push(format!("Failed to parse style for key {name:?}. {err}"));
        }

        // these are used both as UI and as highlights
        styles.insert(name.clone(), style);
        scopes.push(name);
        highlights.push(style);
    }

    (styles, scopes, highlights, rainbow_length, warnings)
}

struct ThemePalette {
    palette: HashMap<String, Color>,
}

impl Default for ThemePalette {
    fn default() -> Self {
        Self {
            palette: hashmap! {
                "default".to_string() => Color::Reset,
                "black".to_string() => Color::Black,
                "red".to_string() => Color::Red,
                "green".to_string() => Color::Green,
                "yellow".to_string() => Color::Yellow,
                "blue".to_string() => Color::Blue,
                "magenta".to_string() => Color::Magenta,
                "cyan".to_string() => Color::Cyan,
                "gray".to_string() => Color::Gray,
                "light-red".to_string() => Color::LightRed,
                "light-green".to_string() => Color::LightGreen,
                "light-yellow".to_string() => Color::LightYellow,
                "light-blue".to_string() => Color::LightBlue,
                "light-magenta".to_string() => Color::LightMagenta,
                "light-cyan".to_string() => Color::LightCyan,
                "light-gray".to_string() => Color::LightGray,
                "white".to_string() => Color::White,
            },
        }
    }
}

impl ThemePalette {
    pub fn new(palette: HashMap<String, Color>) -> Self {
        let ThemePalette { palette: mut default } = ThemePalette::default();

        default.extend(palette);
        Self { palette: default }
    }

    pub fn string_to_rgb(s: &str) -> Result<Color, String> {
        if s.starts_with('#') {
            Self::hex_string_to_rgb(s)
        } else {
            Self::ansi_string_to_rgb(s)
        }
    }

    fn ansi_string_to_rgb(s: &str) -> Result<Color, String> {
        if let Ok(index) = s.parse::<u8>() {
            return Ok(Color::Indexed(index));
        }
        Err(format!("Malformed ANSI: {}", s))
    }

    fn hex_string_to_rgb(s: &str) -> Result<Color, String> {
        if s.len() >= 7 {
            if let (Ok(red), Ok(green), Ok(blue)) = (
                u8::from_str_radix(&s[1..3], 16),
                u8::from_str_radix(&s[3..5], 16),
                u8::from_str_radix(&s[5..7], 16),
            ) {
                return Ok(Color::Rgb(red, green, blue));
            }
        }

        Err(format!("Malformed hexcode: {}", s))
    }

    fn parse_value_as_str(value: &Value) -> Result<&str, String> {
        value.as_str().ok_or(format!("Unrecognized value: {}", value))
    }

    pub fn parse_color(&self, value: Value) -> Result<Color, String> {
        let value = Self::parse_value_as_str(&value)?;

        self.palette
            .get(value)
            .copied()
            .ok_or("")
            .or_else(|_| Self::string_to_rgb(value))
    }

    pub fn parse_modifier(value: &Value) -> Result<Modifier, String> {
        value
            .as_str()
            .and_then(|s| s.parse().ok())
            .ok_or(format!("Invalid modifier: {}", value))
    }

    pub fn parse_underline_style(value: &Value) -> Result<UnderlineStyle, String> {
        value
            .as_str()
            .and_then(|s| s.parse().ok())
            .ok_or(format!("Invalid underline style: {}", value))
    }

    pub fn parse_style(&self, style: &mut Style, value: Value) -> Result<(), String> {
        if let Value::Table(entries) = value {
            for (name, mut value) in entries {
                match name.as_str() {
                    "fg" => *style = style.fg(self.parse_color(value)?),
                    "bg" => *style = style.bg(self.parse_color(value)?),
                    "underline" => {
                        let table =
                            value.as_table_mut().ok_or("Underline must be table")?;
                        if let Some(value) = table.remove("color") {
                            *style = style.underline_color(self.parse_color(value)?);
                        }
                        if let Some(value) = table.remove("style") {
                            *style = style
                                .underline_style(Self::parse_underline_style(&value)?);
                        }

                        if let Some(attr) = table.keys().next() {
                            return Err(format!("Invalid underline attribute: {attr}"));
                        }
                    }
                    "modifiers" => {
                        let modifiers =
                            value.as_array().ok_or("Modifiers should be an array")?;

                        for modifier in modifiers {
                            if modifier.as_str() == Some("underlined") {
                                *style = style.underline_style(UnderlineStyle::Line);
                            } else {
                                *style =
                                    style.add_modifier(Self::parse_modifier(modifier)?);
                            }
                        }
                    }
                    _ => return Err(format!("Invalid style attribute: {}", name)),
                }
            }
        } else {
            *style = style.fg(self.parse_color(value)?);
        }
        Ok(())
    }

    fn parse_style_array(&self, value: Value) -> Result<Vec<Style>, String> {
        let mut styles = Vec::new();

        for v in value
            .as_array()
            .ok_or_else(|| format!("Could not parse value as an array: '{value}'"))?
        {
            let mut style = Style::default();
            self.parse_style(&mut style, v.clone())?;
            styles.push(style);
        }

        Ok(styles)
    }
}

impl TryFrom<Value> for ThemePalette {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let map = match value {
            Value::Table(entries) => entries,
            _ => return Ok(Self::default()),
        };

        let mut palette = HashMap::with_capacity(map.len());
        for (name, value) in map {
            let value = Self::parse_value_as_str(&value)?;
            let color = Self::string_to_rgb(value)?;
            palette.insert(name, color);
        }

        Ok(Self::new(palette))
    }
}

macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = hashmap!(@count $($key),*);
            let mut _map = ::std::collections::HashMap::with_capacity(_cap);
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}
use hashmap;

fn default_rainbow() -> Vec<Style> {
    vec![
        Style::default().fg(Color::Red),
        Style::default().fg(Color::Yellow),
        Style::default().fg(Color::Green),
        Style::default().fg(Color::Blue),
        Style::default().fg(Color::Cyan),
        Style::default().fg(Color::Magenta),
    ]
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl Style {
    pub const fn new() -> Self {
        Style {
            fg: None,
            bg: None,
            underline_color: None,
            underline_style: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }

    /// Returns a `Style` resetting all properties.
    pub const fn reset() -> Self {
        Self {
            fg: Some(Color::Reset),
            bg: Some(Color::Reset),
            underline_color: None,
            underline_style: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::all(),
        }
    }

    /// Changes the foreground color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use helix_view::graphics::{Color, Style};
    /// let style = Style::default().fg(Color::Blue);
    /// let diff = Style::default().fg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().fg(Color::Red));
    /// ```
    pub const fn fg(mut self, color: Color) -> Style {
        self.fg = Some(color);
        self
    }

    /// Changes the background color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use helix_view::graphics::{Color, Style};
    /// let style = Style::default().bg(Color::Blue);
    /// let diff = Style::default().bg(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().bg(Color::Red));
    /// ```
    pub const fn bg(mut self, color: Color) -> Style {
        self.bg = Some(color);
        self
    }

    /// Changes the underline color.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use helix_view::graphics::{Color, Style};
    /// let style = Style::default().underline_color(Color::Blue);
    /// let diff = Style::default().underline_color(Color::Red);
    /// assert_eq!(style.patch(diff), Style::default().underline_color(Color::Red));
    /// ```
    pub const fn underline_color(mut self, color: Color) -> Style {
        self.underline_color = Some(color);
        self
    }

    /// Changes the underline style.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use helix_view::graphics::{UnderlineStyle, Style};
    /// let style = Style::default().underline_style(UnderlineStyle::Line);
    /// let diff = Style::default().underline_style(UnderlineStyle::Curl);
    /// assert_eq!(style.patch(diff), Style::default().underline_style(UnderlineStyle::Curl));
    /// ```
    pub const fn underline_style(mut self, style: UnderlineStyle) -> Style {
        self.underline_style = Some(style);
        self
    }

    /// Changes the text emphasis.
    ///
    /// When applied, it adds the given modifier to the `Style` modifiers.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use helix_view::graphics::{Color, Modifier, Style};
    /// let style = Style::default().add_modifier(Modifier::BOLD);
    /// let diff = Style::default().add_modifier(Modifier::ITALIC);
    /// let patched = style.patch(diff);
    /// assert_eq!(patched.add_modifier, Modifier::BOLD | Modifier::ITALIC);
    /// assert_eq!(patched.sub_modifier, Modifier::empty());
    /// ```
    pub fn add_modifier(mut self, modifier: Modifier) -> Style {
        self.sub_modifier.remove(modifier);
        self.add_modifier.insert(modifier);
        self
    }

    /// Changes the text emphasis.
    ///
    /// When applied, it removes the given modifier from the `Style` modifiers.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// # use helix_view::graphics::{Color, Modifier, Style};
    /// let style = Style::default().add_modifier(Modifier::BOLD | Modifier::ITALIC);
    /// let diff = Style::default().remove_modifier(Modifier::ITALIC);
    /// let patched = style.patch(diff);
    /// assert_eq!(patched.add_modifier, Modifier::BOLD);
    /// assert_eq!(patched.sub_modifier, Modifier::ITALIC);
    /// ```
    pub fn remove_modifier(mut self, modifier: Modifier) -> Style {
        self.add_modifier.remove(modifier);
        self.sub_modifier.insert(modifier);
        self
    }

    /// Results in a combined style that is equivalent to applying the two individual styles to
    /// a style one after the other.
    ///
    /// ## Examples
    /// ```
    /// # use helix_view::graphics::{Color, Modifier, Style};
    /// let style_1 = Style::default().fg(Color::Yellow);
    /// let style_2 = Style::default().bg(Color::Red);
    /// let combined = style_1.patch(style_2);
    /// assert_eq!(
    ///     Style::default().patch(style_1).patch(style_2),
    ///     Style::default().patch(combined));
    /// ```
    pub fn patch(mut self, other: Style) -> Style {
        self.fg = other.fg.or(self.fg);
        self.bg = other.bg.or(self.bg);
        self.underline_color = other.underline_color.or(self.underline_color);
        self.underline_style = other.underline_style.or(self.underline_style);

        self.add_modifier.remove(other.sub_modifier);
        self.add_modifier.insert(other.add_modifier);
        self.sub_modifier.remove(other.add_modifier);
        self.sub_modifier.insert(other.sub_modifier);

        self
    }
}

impl std::str::FromStr for UnderlineStyle {
    type Err = &'static str;

    fn from_str(modifier: &str) -> Result<Self, Self::Err> {
        match modifier {
            "line" => Ok(Self::Line),
            "curl" => Ok(Self::Curl),
            "dotted" => Ok(Self::Dotted),
            "dashed" => Ok(Self::Dashed),
            "double_line" => Ok(Self::DoubleLine),
            _ => Err("Invalid underline style"),
        }
    }
}

impl std::str::FromStr for Modifier {
    type Err = &'static str;

    fn from_str(modifier: &str) -> Result<Self, Self::Err> {
        match modifier {
            "bold" => Ok(Self::BOLD),
            "dim" => Ok(Self::DIM),
            "italic" => Ok(Self::ITALIC),
            "slow_blink" => Ok(Self::SLOW_BLINK),
            "rapid_blink" => Ok(Self::RAPID_BLINK),
            "reversed" => Ok(Self::REVERSED),
            "hidden" => Ok(Self::HIDDEN),
            "crossed_out" => Ok(Self::CROSSED_OUT),
            _ => Err("Invalid modifier"),
        }
    }
}
