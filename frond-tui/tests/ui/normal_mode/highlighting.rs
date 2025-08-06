//! Tests for syntax highlighting functionality
//!
//! These tests verify the markdown syntax highlighting behavior,
//! focusing on input/output behavior rather than implementation details.

use frond::ui::normal_mode::highlighting::highlight_markdown;

#[test]
fn highlight_markdown_plain_text() {
    let result = highlight_markdown("Simple plain text");
    
    // Should return valid Text object (not empty)
    assert!(!result.lines.is_empty());
    
    // Content should be preserved
    let content = result.lines.into_iter()
        .flat_map(|line| line.spans)
        .map(|span| span.content)
        .collect::<String>();
    assert!(content.contains("Simple plain text"));
}

#[test]
fn highlight_markdown_with_headers() {
    let markdown = "# Header 1\n## Header 2\nRegular text";
    let result = highlight_markdown(markdown);
    
    // Should return valid Text with multiple lines
    assert!(result.lines.len() >= 3);
    
    // Content should be preserved
    let full_content = result.lines.into_iter()
        .flat_map(|line| line.spans)
        .map(|span| span.content)
        .collect::<String>();
    assert!(full_content.contains("Header 1"));
    assert!(full_content.contains("Header 2"));
    assert!(full_content.contains("Regular text"));
}

#[test]
fn highlight_markdown_with_code_blocks() {
    let markdown = "```rust\nfn main() {\n    println!(\"Hello\");\n}\n```";
    let result = highlight_markdown(markdown);
    
    // Should return valid Text
    assert!(!result.lines.is_empty());
    
    // Code content should be preserved
    let content = result.lines.into_iter()
        .flat_map(|line| line.spans)
        .map(|span| span.content)
        .collect::<String>();
    assert!(content.contains("fn main"));
    assert!(content.contains("println!"));
}

#[test]
fn highlight_markdown_with_inline_code() {
    let markdown = "This is `inline code` in text";
    let result = highlight_markdown(markdown);
    
    // Should return valid Text
    assert!(!result.lines.is_empty());
    
    // Content should be preserved
    let content = result.lines.into_iter()
        .flat_map(|line| line.spans)
        .map(|span| span.content)
        .collect::<String>();
    assert!(content.contains("inline code"));
    assert!(content.contains("This is"));
    assert!(content.contains("in text"));
}

#[test]
fn highlight_markdown_empty_string() {
    let result = highlight_markdown("");
    
    // Should handle empty input gracefully
    // May return empty or single empty line
    assert!(result.lines.len() <= 1);
}

#[test]
fn highlight_markdown_preserves_newlines() {
    let markdown = "Line 1\n\nLine 3";
    let result = highlight_markdown(markdown);
    
    // Should preserve line structure
    assert!(result.lines.len() >= 2);
    
    // Content should be preserved
    let content = result.lines.into_iter()
        .flat_map(|line| line.spans)
        .map(|span| span.content)
        .collect::<String>();
    assert!(content.contains("Line 1"));
    assert!(content.contains("Line 3"));
}

#[test]
fn highlight_markdown_with_special_characters() {
    let markdown = "Text with **bold** and *italic* and [links](url)";
    let result = highlight_markdown(markdown);
    
    // Should handle markdown syntax
    assert!(!result.lines.is_empty());
    
    // Content should be preserved (may be formatted)
    let content = result.lines.into_iter()
        .flat_map(|line| line.spans)
        .map(|span| span.content)
        .collect::<String>();
    assert!(content.contains("bold"));
    assert!(content.contains("italic"));
    assert!(content.contains("links"));
}

#[test]
fn highlight_markdown_deterministic_output() {
    let markdown = "# Test\nSome content";
    
    // Should produce same output for same input
    let result1 = highlight_markdown(markdown);
    let result2 = highlight_markdown(markdown);
    
    assert_eq!(result1.lines.len(), result2.lines.len());
    
    // Compare content
    let content1 = result1.lines.into_iter()
        .flat_map(|line| line.spans)
        .map(|span| span.content)
        .collect::<String>();
    let content2 = result2.lines.into_iter()
        .flat_map(|line| line.spans)
        .map(|span| span.content)
        .collect::<String>();
    
    assert_eq!(content1, content2);
}