use strum::EnumIter;

// Schema - parameter-less templates for registry
#[derive(Debug, Clone, EnumIter)]
pub enum EditModeActionSchema {
    // Mode transitions
    ExitCurrentMode,

    // Text editing actions (can be expanded later)
    InsertChar,
    DeleteChar,
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorUp,
    MoveCursorDown,
    SelectAll,
    Cut,
    Copy,
    Paste,
}

impl EditModeActionSchema {
    pub fn id(&self) -> &'static str {
        match self {
            Self::ExitCurrentMode => "exit_mode",
            Self::InsertChar => "insert_char",
            Self::DeleteChar => "delete_char",
            Self::MoveCursorLeft => "move_cursor_left",
            Self::MoveCursorRight => "move_cursor_right",
            Self::MoveCursorUp => "move_cursor_up",
            Self::MoveCursorDown => "move_cursor_down",
            Self::SelectAll => "select_all",
            Self::Cut => "cut",
            Self::Copy => "copy",
            Self::Paste => "paste",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::ExitCurrentMode => "exit mode",
            Self::InsertChar => "insert character",
            Self::DeleteChar => "delete character",
            Self::MoveCursorLeft => "move cursor left",
            Self::MoveCursorRight => "move cursor right",
            Self::MoveCursorUp => "move cursor up",
            Self::MoveCursorDown => "move cursor down",
            Self::SelectAll => "select all",
            Self::Cut => "cut",
            Self::Copy => "copy",
            Self::Paste => "paste",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ExitCurrentMode => "exit the current mode",
            Self::InsertChar => "insert a character at cursor position",
            Self::DeleteChar => "delete character at cursor position",
            Self::MoveCursorLeft => "move cursor one position left",
            Self::MoveCursorRight => "move cursor one position right",
            Self::MoveCursorUp => "move cursor one line up",
            Self::MoveCursorDown => "move cursor one line down",
            Self::SelectAll => "select all text",
            Self::Cut => "cut selected text to clipboard",
            Self::Copy => "copy selected text to clipboard",
            Self::Paste => "paste text from clipboard",
        }
    }

    pub fn requires_focus(&self) -> bool {
        // Most edit actions don't require message focus, just editor focus
        false
    }
}

// Action - with real parameters for dispatch
#[derive(Debug, Clone)]
pub enum EditModeAction {
    // Mode transitions
    ExitCurrentMode,

    // Text editing actions with parameters
    InsertChar(char),
    DeleteChar,
    MoveCursor(CursorDirection),
    SelectAll,
    Cut,
    Copy,
    Paste(String),
}

#[derive(Debug, Clone)]
pub enum CursorDirection {
    Left,
    Right,
    Up,
    Down,
}
