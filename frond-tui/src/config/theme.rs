use ratatui::{style::Color, widgets::BorderType};
use std::collections::HashMap;

pub fn parse_color(s: &str) -> Option<Color> {
    match s.to_lowercase().as_str() {
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "gray" => Some(Color::Gray),
        "dark_gray" => Some(Color::DarkGray),
        "light_red" => Some(Color::LightRed),
        "light_green" => Some(Color::LightGreen),
        "light_yellow" => Some(Color::LightYellow),
        "light_blue" => Some(Color::LightBlue),
        "light_magenta" => Some(Color::LightMagenta),
        "light_cyan" => Some(Color::LightCyan),
        "white" => Some(Color::White),
        _ => None,
    }
}

pub fn parse_border_type(s: &str) -> Option<BorderType> {
    match s.to_lowercase().as_str() {
        "plain" => Some(BorderType::Plain),
        "double" => Some(BorderType::Double),
        "rounded" => Some(BorderType::Rounded),
        "thick" => Some(BorderType::Thick),
        "quadrant_inside" => Some(BorderType::QuadrantInside),
        "quadrant_outside" => Some(BorderType::QuadrantOutside),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub assistant: MessageTheme,
    pub user: MessageTheme,
    pub focused: FocusedTheme,
    pub help: Help,
}

#[derive(Debug, Clone)]
pub struct MessageTheme {
    pub frame_color: Color,
    pub title_color: Color,
    pub text_color: Color,
    pub border_type: BorderType,
    pub display_name: String,
}

#[derive(Debug, Clone)]
pub struct FocusedTheme {
    pub frame_color: Color,
    pub border_type: BorderType,
    pub text_color: Color,
}

#[derive(Debug, Clone)]
pub struct Help {
    pub key_color: Color,
    pub text_color: Color,
}

pub fn default_theme_string_map() -> HashMap<String, HashMap<String, String>> {
    [
        (
            "assistant",
            &[
                ("frame_color", "blue"),
                ("title_color", "blue"),
                ("text_color", "white"),
                ("border_type", "plain"),
                ("display_name", "Assistant"),
            ][..],
        ),
        (
            "user",
            &[
                ("frame_color", "dark_gray"),
                ("title_color", "white"),
                ("text_color", "white"),
                ("border_type", "plain"),
                ("display_name", "User"),
            ],
        ),
        (
            "focused",
            &[
                ("frame_color", "white"),
                ("border_type", "double"),
                ("text_color", "white"),
            ],
        ),
        ("help", &[("key_color", "blue"), ("text_color", "white")]),
    ]
    .into_iter()
    .map(|(section, fields)| {
        (
            section.to_string(),
            fields
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect(),
        )
    })
    .collect()
}

impl Theme {
    pub fn from_string_map(raw: HashMap<String, HashMap<String, String>>) -> Result<Self, String> {
        let defaults = default_theme_string_map();

        let get_section = |section: &str| raw.get(section).or_else(|| defaults.get(section));

        let assistant = MessageTheme::from_string_map(
            get_section("assistant").unwrap_or(&HashMap::new()),
            defaults.get("assistant").unwrap(),
        )?;
        let user = MessageTheme::from_string_map(
            get_section("user").unwrap_or(&HashMap::new()),
            defaults.get("user").unwrap(),
        )?;
        let focused = FocusedTheme::from_string_map(
            get_section("focused").unwrap_or(&HashMap::new()),
            defaults.get("focused").unwrap(),
        )?;

        let help = Help::from_string_map(
            get_section("help").unwrap_or(&HashMap::new()),
            defaults.get("help").unwrap(),
        )?;

        Ok(Self {
            assistant,
            user,
            focused,
            help,
        })
    }
}

impl MessageTheme {
    pub fn from_string_map(
        raw: &HashMap<String, String>,
        defaults: &HashMap<String, String>,
    ) -> Result<Self, String> {
        let get = |k: &str| raw.get(k).or_else(|| defaults.get(k)).cloned();

        Ok(Self {
            frame_color: get("frame_color")
                .and_then(|s| parse_color(&s))
                .unwrap_or(Color::Blue),
            title_color: get("title_color")
                .and_then(|s| parse_color(&s))
                .unwrap_or(Color::Blue),
            text_color: get("text_color")
                .and_then(|s| parse_color(&s))
                .unwrap_or(Color::White),
            border_type: get("border_type")
                .and_then(|s| parse_border_type(&s))
                .unwrap_or(BorderType::Plain),
            display_name: get("display_name").unwrap_or_else(|| "Assistant".to_string()),
        })
    }
}

impl FocusedTheme {
    pub fn from_string_map(
        raw: &HashMap<String, String>,
        defaults: &HashMap<String, String>,
    ) -> Result<Self, String> {
        let get = |k: &str| raw.get(k).or_else(|| defaults.get(k)).cloned();

        Ok(Self {
            frame_color: get("frame_color")
                .and_then(|s| parse_color(&s))
                .unwrap_or(Color::White),
            border_type: get("border_type")
                .and_then(|s| parse_border_type(&s))
                .unwrap_or(BorderType::Double),
            text_color: get("text_color")
                .and_then(|s| parse_color(&s))
                .unwrap_or(Color::White),
        })
    }
}

impl Help {
    pub fn from_string_map(
        raw: &HashMap<String, String>,
        defaults: &HashMap<String, String>,
    ) -> Result<Self, String> {
        let get = |k: &str| raw.get(k).or_else(|| defaults.get(k)).cloned();

        Ok(Self {
            key_color: get("key_color")
                .and_then(|s| parse_color(&s))
                .unwrap_or(Color::Blue),
            text_color: get("text_color")
                .and_then(|s| parse_color(&s))
                .unwrap_or(Color::White),
        })
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::from_string_map(default_theme_string_map())
            .expect("Default theme string map is valid")
    }
}
