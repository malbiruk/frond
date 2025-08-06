use ansi_to_tui::IntoText;
use bat::{Input, assets::HighlightingAssets, config::Config, controller::Controller};
use ratatui::text::Text;

pub fn highlight_markdown(content: &str) -> Text<'static> {
    let config = Config {
        colored_output: true,
        ..Default::default()
    };
    let assets = HighlightingAssets::from_binary();
    let controller = Controller::new(&config, &assets);

    let mut buffer = String::new();
    let input = Input::from_bytes(content.as_bytes()).name("text.md");

    if controller
        .run(vec![input.into()], Some(&mut buffer))
        .is_ok()
    {
        buffer
            .into_text()
            .unwrap_or_else(|_| Text::raw(content.to_string()))
    } else {
        Text::raw(content.to_string())
    }
}
