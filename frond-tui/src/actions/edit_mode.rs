use crate::actions::macros::define_actions;

define_actions! {
    EditModeActionSchema, EditModeAction {
        // Character operations
        DeleteChar {
            id: "delete_char",
            name: "delete char",
            description: "delete one character before cursor",
        },
        DeleteNextChar {
            id: "delete_next_char",
            name: "delete next char",
            description: "delete one character next to cursor",
        },
        InsertNewline {
            id: "insert_newline",
            name: "newline",
            description: "insert newline",
        },

        // Line operations
        DeleteLineByEnd {
            id: "delete_line_by_end",
            name: "delete to end",
            description: "delete from cursor until the end of line",
        },
        DeleteLineByHead {
            id: "delete_line_by_head",
            name: "delete to head",
            description: "delete from cursor until the head of line",
        },

        // Word operations
        DeleteWord {
            id: "delete_word",
            name: "delete word",
            description: "delete one word before cursor",
        },
        DeleteNextWord {
            id: "delete_next_word",
            name: "delete next word",
            description: "delete one word next to cursor",
        },

        // Undo/Redo
        Undo {
            id: "undo",
            name: "undo",
            description: "undo last operation",
        },
        Redo {
            id: "redo",
            name: "redo",
            description: "redo last undone operation",
        },

        // Clipboard operations
        Copy {
            id: "copy",
            name: "copy",
            description: "copy selected text",
        },
        Cut {
            id: "cut",
            name: "cut",
            description: "cut selected text",
        },
        Paste {
            id: "paste",
            name: "paste",
            description: "paste yanked text",
        },

        // Selection operations
        SelectAll {
            id: "select_all",
            name: "select all",
            description: "select entire text",
        },

        // Selection movement - character level
        SelectForward {
            id: "select_forward",
            name: "select forward",
            description: "extend selection forward by one character",
        },
        SelectBack {
            id: "select_back",
            name: "select back",
            description: "extend selection backward by one character",
        },
        SelectUp {
            id: "select_up",
            name: "select up",
            description: "extend selection up by one line",
        },
        SelectDown {
            id: "select_down",
            name: "select down",
            description: "extend selection down by one line",
        },

        // Selection movement - word level
        SelectWordForward {
            id: "select_word_forward",
            name: "select word forward",
            description: "extend selection forward by word",
        },
        SelectWordBack {
            id: "select_word_back",
            name: "select word back",
            description: "extend selection backward by word",
        },

        // Selection movement - line boundaries
        SelectToEnd {
            id: "select_to_end",
            name: "select to end",
            description: "extend selection to end of line",
        },
        SelectToHead {
            id: "select_to_head",
            name: "select to head",
            description: "extend selection to head of line",
        },

        // Cursor movement - character level
        MoveCursorForward {
            id: "move_cursor_forward",
            name: "cursor forward",
            description: "move cursor forward by one character",
        },
        MoveCursorBack {
            id: "move_cursor_back",
            name: "cursor back",
            description: "move cursor backward by one character",
        },
        MoveCursorUp {
            id: "move_cursor_up",
            name: "cursor up",
            description: "move cursor up by one line",
        },
        MoveCursorDown {
            id: "move_cursor_down",
            name: "cursor down",
            description: "move cursor down by one line",
        },

        // Cursor movement - word level
        MoveCursorWordForward {
            id: "move_cursor_word_forward",
            name: "cursor word forward",
            description: "move cursor forward by word",
        },
        MoveCursorWordEnd {
            id: "move_cursor_word_end",
            name: "cursor word end",
            description: "move cursor to next end of word",
        },
        MoveCursorWordBack {
            id: "move_cursor_word_back",
            name: "cursor word back",
            description: "move cursor backward by word",
        },

        // Cursor movement - paragraph level
        MoveCursorParagraphForward {
            id: "move_cursor_paragraph_forward",
            name: "cursor paragraph forward",
            description: "move cursor forward by paragraph",
        },
        MoveCursorParagraphBack {
            id: "move_cursor_paragraph_back",
            name: "cursor paragraph back",
            description: "move cursor backward by paragraph",
        },

        // Cursor movement - line boundaries
        MoveCursorEnd {
            id: "move_cursor_end",
            name: "cursor end",
            description: "move cursor to the end of line",
        },
        MoveCursorHead {
            id: "move_cursor_head",
            name: "cursor head",
            description: "move cursor to the head of line",
        },

        // Cursor movement - document boundaries
        MoveCursorTop {
            id: "move_cursor_top",
            name: "cursor top",
            description: "move cursor to top of document",
        },
        MoveCursorBottom {
            id: "move_cursor_bottom",
            name: "cursor bottom",
            description: "move cursor to bottom of document",
        },

        // Cursor movement - viewport
        MoveCursorInViewport {
            id: "move_cursor_in_viewport",
            name: "cursor in viewport",
            description: "move cursor to stay in the viewport",
        },

        // Mode operations
        ExitCurrentMode {
            id: "exit_mode",
            name: "exit mode",
            description: "exit the current mode",
        },
        SubmitMessage {
            id: "submit_message",
            name: "submit message",
            description: "submit the message and generate response",
        },
    }
}
