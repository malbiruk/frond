use crate::app::AppState;
use ratatui::text::Line;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::{Frame, layout::Rect};

use super::shared::{calculate_dash_count, create_line_with_dashes};

pub fn render(frame: &mut Frame, area: Rect, app_state: &AppState) {
    let breadcrumb_text = build_breadcrumb_text(app_state);
    let token_info = build_token_info(app_state);
    let title_line = create_title_line(breadcrumb_text, token_info, area.width);
    let breadcrumb = create_breadcrumb_widget(title_line);
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

fn build_token_info(app_state: &AppState) -> String {
    format!(
        "{}K/{}K",
        app_state.config.model_info.tokens_used / 1000,
        app_state.config.model_info.tokens_available / 1000
    )
}

fn create_title_line(left_text: String, right_text: String, width: u16) -> Line<'static> {
    let dash_count = calculate_dash_count(left_text.len(), right_text.len(), width);
    create_line_with_dashes(left_text, right_text, dash_count)
}

fn create_breadcrumb_widget(title_line: Line<'static>) -> Paragraph<'static> {
    Paragraph::new("").block(Block::new().borders(Borders::TOP).title(title_line))
}
