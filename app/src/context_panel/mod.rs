//! Context Panel — terminal-context-aware widgets.
//!
//! v0.3 introduction. Replaces (logically, not structurally) the AI Panel with
//! a stack of collapsible cards that show working directory, git status,
//! Claude Code session activity, and the foreground process for the active tab.
//!
//! The AI Panel struct is still present but defaults to closed; the Context
//! Panel is a sibling right-side panel toggled by `Cmd+Shift+K`.

pub mod git_state;
pub mod panel;
pub mod widgets;

pub const CONTEXT_PANEL_FEATURE_NAME: &str = "Context";
pub const CONTEXT_PANEL_TOGGLE_BINDING_NAME: &str = "workspace:toggle_context_panel";
