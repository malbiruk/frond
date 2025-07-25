//! Integration tests for UI calculation functions
//!
//! This test file imports and runs all the UI calculation tests that verify
//! mathematical functions used in the UI layer.

mod ui;

// Re-export test modules so they are discovered by cargo test
pub use ui::*;
