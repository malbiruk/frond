use crate::app::AppState;
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::{Frame, layout::Rect};

pub fn render(frame: &mut Frame, area: Rect, app_state: &AppState) {
    let breadcrumb_text = build_breadcrumb_text(app_state);
    let token_info = build_token_info(app_state);
    let breadcrumb = create_breadcrumb_widget(breadcrumb_text, token_info);
    frame.render_widget(breadcrumb, area);
}

fn build_breadcrumb_text(app_state: &AppState) -> String {
    let dialogue_name = app_state.dialogue.name().to_string();

    match (
        has_multiple_trees(app_state),
        has_multiple_branches(app_state),
    ) {
        (true, _) => breadcrumb_with_tree_and_optional_branch(app_state, &dialogue_name),
        (false, true) => breadcrumb_with_branch(app_state, &dialogue_name),
        (false, false) => dialogue_name,
    }
}

fn has_multiple_trees(app_state: &AppState) -> bool {
    app_state.dialogue.trees().len() > 1
}

fn has_multiple_branches(app_state: &AppState) -> bool {
    app_state
        .current_tree()
        .map(|tree| tree.branches().len() > 1)
        .unwrap_or(false)
}

fn breadcrumb_with_tree_and_optional_branch(app_state: &AppState, dialogue_name: &str) -> String {
    let mut parts = vec![dialogue_name.to_string()];
    if let Some(tree) = app_state.current_tree() {
        parts.push(tree.name().to_string());
        if tree.branches().len() > 1 {
            if let Some(branch) = app_state.current_branch() {
                parts.push(branch.name().to_string());
            }
        }
    }
    parts.join(" > ")
}

fn breadcrumb_with_branch(app_state: &AppState, dialogue_name: &str) -> String {
    if let Some(branch) = app_state.current_branch() {
        format!("{} >> {}", dialogue_name, branch.name())
    } else {
        dialogue_name.to_string()
    }
}

pub fn build_token_info(app_state: &AppState) -> String {
    let used = format_tokens(app_state.model_info.tokens_used);
    let available = format_tokens(app_state.model_info.tokens_available);
    format!("{}/{}", used, available)
}

fn format_tokens(tokens: u32) -> String {
    if tokens < 1000 {
        tokens.to_string()
    } else {
        let k = tokens as f64 / 1000.0;
        if k < 10.0 {
            format!("{:.1}K", k)
        } else {
            format!("{}K", (k as usize))
        }
    }
}

fn create_breadcrumb_widget(left_text: String, right_text: String) -> Paragraph<'static> {
    let block = Block::new()
        .borders(Borders::TOP)
        .title(Line::from(format!("{} ", left_text)).left_aligned())
        .title(Line::from(format!(" {}", right_text)).right_aligned());

    Paragraph::new("").block(block)
}
