//! Tests for layout calculation functions
//!
//! These tests verify the accuracy of layout calculations for different terminal sizes
//! and modes, ensuring proper area allocation and boundary handling.

use frond::app::Mode;
use frond::ui::layout::AppLayout;
use ratatui::layout::Rect;

// === Layout Creation Tests ===

#[test]
fn app_layout_new_normal_mode() {
    let area = Rect::new(0, 0, 80, 24);
    let layout = AppLayout::new(area, Mode::Normal);

    match layout {
        AppLayout::Normal {
            breadcrumb,
            content,
            status,
        } => {
            // All areas should be within the original area bounds
            assert!(breadcrumb.x >= area.x);
            assert!(breadcrumb.y >= area.y);
            assert!(breadcrumb.right() <= area.right());
            assert!(breadcrumb.bottom() <= area.bottom());

            assert!(content.x >= area.x);
            assert!(content.y >= area.y);
            assert!(content.right() <= area.right());
            assert!(content.bottom() <= area.bottom());

            assert!(status.x >= area.x);
            assert!(status.y >= area.y);
            assert!(status.right() <= area.right());
            assert!(status.bottom() <= area.bottom());

            // Areas should not overlap vertically
            assert!(breadcrumb.bottom() <= content.y);
            assert!(content.bottom() <= status.y);

            // Breadcrumb should be 2 units high (in normal mode)
            assert_eq!(breadcrumb.height, 2);

            // Status should be 2 units high
            assert_eq!(status.height, 2);

            // Content should be flexible (remaining space)
            assert!(content.height > 0);
        }
    }
}

#[test]
fn app_layout_new_edit_mode() {
    let area = Rect::new(0, 0, 80, 24);
    let layout = AppLayout::new(area, Mode::Edit);

    match layout {
        AppLayout::Normal {
            breadcrumb,
            content,
            status,
        } => {
            // Should still return Normal variant but with different dimensions
            // Breadcrumb should be 2 units high (in edit mode)
            assert_eq!(breadcrumb.height, 2);

            // Status should be 2 units high
            assert_eq!(status.height, 2);

            // Content should be flexible (remaining space)
            assert!(content.height > 0);

            // Areas should not overlap vertically
            assert!(breadcrumb.bottom() <= content.y);
            assert!(content.bottom() <= status.y);
        }
    }
}

#[test]
fn app_layout_minimal_terminal_size() {
    let area = Rect::new(0, 0, 10, 8);
    let layout = AppLayout::new(area, Mode::Normal);

    match layout {
        AppLayout::Normal {
            breadcrumb,
            content,
            status,
        } => {
            // Should handle minimal size gracefully
            assert!(breadcrumb.height > 0);
            assert!(status.height > 0);

            // Content might be very small or zero in minimal terminal
            // Layout should not panic (test passes if we reach this point)

            // Areas should be within bounds
            assert!(breadcrumb.right() <= area.right());
            assert!(content.right() <= area.right());
            assert!(status.right() <= area.right());
        }
    }
}

#[test]
fn app_layout_very_small_terminal() {
    let area = Rect::new(0, 0, 5, 5);
    let layout = AppLayout::new(area, Mode::Normal);

    match layout {
        AppLayout::Normal {
            breadcrumb,
            content,
            status,
        } => {
            // Should handle very small terminal without panicking
            assert!(breadcrumb.x < area.right());
            assert!(content.x < area.right());
            assert!(status.x < area.right());

            // Heights might be constrained by available space
            assert!(breadcrumb.bottom() <= area.bottom());
            assert!(content.bottom() <= area.bottom());
            assert!(status.bottom() <= area.bottom());
        }
    }
}

#[test]
fn app_layout_wide_terminal() {
    let area = Rect::new(0, 0, 200, 50);
    let layout = AppLayout::new(area, Mode::Normal);

    match layout {
        AppLayout::Normal {
            breadcrumb,
            content,
            status,
        } => {
            // All areas should utilize the full width (accounting for margins)
            let expected_width = area.width.saturating_sub(2); // 1 margin on each side

            assert_eq!(breadcrumb.width, expected_width);
            assert_eq!(content.width, expected_width);
            assert_eq!(status.width, expected_width);

            // Heights should be as specified
            assert_eq!(breadcrumb.height, 2);
            assert_eq!(status.height, 2);

            // Content should get most of the height
            assert!(content.height > 40); // Should be around 46 (50 - 4 - 2 margins)
        }
    }
}

#[test]
fn app_layout_tall_terminal() {
    let area = Rect::new(0, 0, 80, 100);
    let layout = AppLayout::new(area, Mode::Normal);

    match layout {
        AppLayout::Normal {
            breadcrumb,
            content,
            status,
        } => {
            // Fixed heights should remain the same
            assert_eq!(breadcrumb.height, 2);
            assert_eq!(status.height, 2);

            // Content should get most of the extra height
            assert!(content.height > 90); // Should be around 96 (100 - 4 - 2 margins)
        }
    }
}

#[test]
fn app_layout_zero_size_area() {
    let area = Rect::new(0, 0, 0, 0);
    let layout = AppLayout::new(area, Mode::Normal);

    match layout {
        AppLayout::Normal {
            breadcrumb,
            content,
            status,
        } => {
            // Should handle zero size without panicking
            // Areas might be zero-sized but should not be out of bounds
            assert!(breadcrumb.right() <= area.right().max(breadcrumb.x));
            assert!(content.right() <= area.right().max(content.x));
            assert!(status.right() <= area.right().max(status.x));
        }
    }
}

// === Layout Consistency Tests ===

#[test]
fn app_layout_areas_dont_overlap() {
    let test_sizes = vec![
        (80, 24),  // Standard terminal
        (120, 30), // Large terminal
        (40, 15),  // Small terminal
        (20, 10),  // Very small terminal
    ];

    for (width, height) in test_sizes {
        let area = Rect::new(0, 0, width, height);
        let layout = AppLayout::new(area, Mode::Normal);

        match layout {
            AppLayout::Normal {
                breadcrumb,
                content,
                status,
            } => {
                // Vertical non-overlap
                assert!(
                    breadcrumb.bottom() <= content.y || content.height == 0,
                    "Breadcrumb and content overlap for size {}x{}",
                    width,
                    height
                );
                assert!(
                    content.bottom() <= status.y || content.height == 0,
                    "Content and status overlap for size {}x{}",
                    width,
                    height
                );

                // All should have same x position and width (from margins)
                assert_eq!(
                    breadcrumb.x, content.x,
                    "X positions should match for size {}x{}",
                    width, height
                );
                assert_eq!(
                    content.x, status.x,
                    "X positions should match for size {}x{}",
                    width, height
                );
                assert_eq!(
                    breadcrumb.width, content.width,
                    "Widths should match for size {}x{}",
                    width, height
                );
                assert_eq!(
                    content.width, status.width,
                    "Widths should match for size {}x{}",
                    width, height
                );
            }
        }
    }
}

#[test]
fn app_layout_mode_differences() {
    let area = Rect::new(0, 0, 80, 24);
    let normal_layout = AppLayout::new(area, Mode::Normal);
    let edit_layout = AppLayout::new(area, Mode::Edit);

    match (normal_layout, edit_layout) {
        (
            AppLayout::Normal {
                breadcrumb: n_breadcrumb,
                content: n_content,
                status: n_status,
            },
            AppLayout::Normal {
                breadcrumb: e_breadcrumb,
                content: e_content,
                status: e_status,
            },
        ) => {
            // Both modes now use the same breadcrumb height (2)
            assert_eq!(
                n_breadcrumb.height, e_breadcrumb.height,
                "Both modes should have the same breadcrumb height"
            );

            // Both should have height 2
            assert_eq!(n_breadcrumb.height, 2);
            assert_eq!(e_breadcrumb.height, 2);

            // Status should be the same height
            assert_eq!(
                n_status.height, e_status.height,
                "Status height should be same between modes"
            );

            // Content should be the same (both modes use same layout)
            assert_eq!(
                n_content.height, e_content.height,
                "Content height should be same when layouts are identical"
            );

            // Both should have the same content height
            assert!(n_content.height > 0);
            assert!(e_content.height > 0);
        }
    }
}

#[test]
fn app_layout_margins_applied_correctly() {
    let area = Rect::new(5, 10, 80, 24);
    let layout = AppLayout::new(area, Mode::Normal);

    match layout {
        AppLayout::Normal {
            breadcrumb,
            content,
            status,
        } => {
            // All areas should be inset by 1 from the original area
            assert_eq!(
                breadcrumb.x,
                area.x + 1,
                "Breadcrumb should respect left margin"
            );
            assert_eq!(content.x, area.x + 1, "Content should respect left margin");
            assert_eq!(status.x, area.x + 1, "Status should respect left margin");

            assert_eq!(
                breadcrumb.y,
                area.y + 1,
                "Breadcrumb should respect top margin"
            );

            // Width should be reduced by 2 (1 margin on each side)
            let expected_width = area.width.saturating_sub(2);
            assert_eq!(
                breadcrumb.width, expected_width,
                "Breadcrumb width should account for margins"
            );
            assert_eq!(
                content.width, expected_width,
                "Content width should account for margins"
            );
            assert_eq!(
                status.width, expected_width,
                "Status width should account for margins"
            );

            // Total used area should not exceed original area
            assert!(
                status.bottom() <= area.bottom(),
                "Layout should not exceed original area"
            );
            assert!(
                breadcrumb.right() <= area.right(),
                "Layout should not exceed original area width"
            );
        }
    }
}

// === Edge Case Tests ===

#[test]
fn app_layout_handles_insufficient_height() {
    // Terminal too small for all required areas
    let area = Rect::new(0, 0, 80, 3); // Only 3 lines total
    let layout = AppLayout::new(area, Mode::Normal);

    match layout {
        AppLayout::Normal {
            breadcrumb,
            content,
            status,
        } => {
            // Should not panic, but some areas might be zero-height
            let total_height = breadcrumb.height + content.height + status.height;

            // Total should not exceed available space (minus margins)
            let available_height = area.height.saturating_sub(2); // 1 margin top/bottom
            assert!(
                total_height <= available_height || available_height == 0,
                "Total layout height {} should not exceed available height {}",
                total_height,
                available_height
            );
        }
    }
}

#[test]
fn app_layout_handles_insufficient_width() {
    // Terminal too narrow
    let area = Rect::new(0, 0, 3, 24); // Only 3 columns total
    let layout = AppLayout::new(area, Mode::Normal);

    match layout {
        AppLayout::Normal {
            breadcrumb,
            content,
            status,
        } => {
            // Should not panic, widths might be zero
            let expected_width = area.width.saturating_sub(2); // Account for margins

            if area.width >= 2 {
                assert_eq!(breadcrumb.width, expected_width);
                assert_eq!(content.width, expected_width);
                assert_eq!(status.width, expected_width);
            } else {
                // With width < 2, margins might result in zero width
                assert!(breadcrumb.width == 0);
                assert!(content.width == 0);
                assert!(status.width == 0);
            }
        }
    }
}

#[test]
fn app_layout_consistent_across_positions() {
    // Layout should be consistent regardless of where the area is positioned
    let positions = vec![(0, 0), (10, 5), (100, 50)];
    let size = (80, 24);

    let mut layouts = vec![];
    for (x, y) in positions {
        let area = Rect::new(x, y, size.0, size.1);
        let layout = AppLayout::new(area, Mode::Normal);
        layouts.push(layout);
    }

    // All layouts should have the same relative dimensions
    for i in 1..layouts.len() {
        match (&layouts[0], &layouts[i]) {
            (
                AppLayout::Normal {
                    breadcrumb: b1,
                    content: c1,
                    status: s1,
                },
                AppLayout::Normal {
                    breadcrumb: b2,
                    content: c2,
                    status: s2,
                },
            ) => {
                assert_eq!(b1.width, b2.width, "Breadcrumb widths should be consistent");
                assert_eq!(c1.width, c2.width, "Content widths should be consistent");
                assert_eq!(s1.width, s2.width, "Status widths should be consistent");

                assert_eq!(
                    b1.height, b2.height,
                    "Breadcrumb heights should be consistent"
                );
                assert_eq!(c1.height, c2.height, "Content heights should be consistent");
                assert_eq!(s1.height, s2.height, "Status heights should be consistent");
            }
        }
    }
}
