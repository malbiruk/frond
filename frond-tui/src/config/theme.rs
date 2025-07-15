use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub assistant: MessageTheme,
    pub user: MessageTheme,
    pub help_key_color: Color,
}

#[derive(Debug, Clone)]
pub struct MessageTheme {
    pub frame_color: Color,
    pub title_color: Color,
    pub text_color: Color,
    pub display_name: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            assistant: MessageTheme {
                frame_color: Color::Blue,
                title_color: Color::Blue,
                text_color: Color::White,
                display_name: "Assistant".to_string(),
            },
            user: MessageTheme {
                frame_color: Color::DarkGray,
                title_color: Color::White,
                text_color: Color::White,
                display_name: "User".to_string(),
            },
            help_key_color: Color::Blue,
        }
    }
}
