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
    pub highlight_color: Color,
    pub secondary_color: Color,
}

pub fn default_theme_string_map() -> HashMap<String, HashMap<String, String>> {
    [(
        "theme",
        &[
            ("highlight_color", "blue"),
            ("secondary_color", "dark_gray"),
        ][..],
    )]
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
        let theme_section = raw.get("theme").or_else(|| defaults.get("theme")).unwrap();

        let get = |k: &str| theme_section.get(k).cloned();

        Ok(Self {
            highlight_color: get("highlight_color")
                .and_then(|s| parse_color(&s))
                .unwrap_or(Color::Blue),
            secondary_color: get("secondary_color")
                .and_then(|s| parse_color(&s))
                .unwrap_or(Color::DarkGray),
        })
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::from_string_map(default_theme_string_map())
            .expect("Default theme string map is valid")
    }
}
