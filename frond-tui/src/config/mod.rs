pub mod keybindings;
pub mod theme;

pub use keybindings::Keybindings;
pub use theme::Theme;

#[derive(Debug, Clone)]
pub struct Config {
    pub theme: Theme,
    pub keybindings: Keybindings,
    pub model_info: ModelInfo,
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub provider: String,
    pub model: String,
    pub tokens_used: u32,
    pub tokens_available: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            keybindings: Keybindings::default(),
            model_info: ModelInfo {
                provider: "ollama".to_string(),
                model: "gemma3n:2b".to_string(),
                tokens_used: 65_000,
                tokens_available: 128_000,
            },
        }
    }
}
