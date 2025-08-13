pub mod keybindings;
pub mod theme;

pub use keybindings::Keybindings;
pub use theme::Theme;

#[derive(Debug, Clone)]
pub struct Config {
    pub theme: Theme,
    pub keybindings: Keybindings,
    pub model: Model,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub provider: String,
    pub name: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::default(),
            keybindings: Keybindings::default(),
            model: Model {
                provider: "ollama".to_string(),
                name: "gemma3n:2b".to_string(),
            },
        }
    }
}

impl Config {
    pub fn get_key_for_action(
        &self,
        mode: crate::app::Mode,
        action_id: &str,
    ) -> Option<&crate::input::KeyChord> {
        self.keybindings.get_keys_for_action(mode, action_id)?.first()
    }

    pub fn get_keys_for_action(
        &self,
        mode: crate::app::Mode,
        action_id: &str,
    ) -> Option<&Vec<crate::input::KeyChord>> {
        self.keybindings.get_keys_for_action(mode, action_id)
    }

    pub fn set_keys_for_action(
        &mut self,
        mode: crate::app::Mode,
        action_id: String,
        keys: Vec<crate::input::KeyChord>,
    ) {
        self.keybindings.set_keys_for_action(mode, action_id, keys);
    }
}
