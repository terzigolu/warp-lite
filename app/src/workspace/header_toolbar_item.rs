use serde::{Deserialize, Serialize};

use crate::features::FeatureFlag;
use crate::ui_components::icons::Icon;
use crate::workspace::tab_settings::TabSettings;

use warpui::{AppContext, SingletonEntity};

/// A configurable item in the vertical tabs header toolbar.
///
/// Each variant represents a panel toggle button that can be placed on either
/// the left or right side of the toolbar. The side determines which side of the
/// main content area the panel opens on.
#[derive(
    Clone,
    Debug,
    Eq,
    PartialEq,
    Hash,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(rename_all = "snake_case")]
pub enum HeaderToolbarItemKind {
    TabsPanel,
    ToolsPanel,
    AgentManagement,
    CodeReview,
    NotificationsMailbox,
}

impl HeaderToolbarItemKind {
    pub fn display_label(&self) -> &'static str {
        match self {
            Self::TabsPanel => "Tabs Panel",
            Self::ToolsPanel => "Tools Panel",
            Self::AgentManagement => "Agent Management",
            Self::CodeReview => "Code Review",
            Self::NotificationsMailbox => "Notifications",
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            Self::TabsPanel => Icon::Menu,
            Self::ToolsPanel => Icon::Tool2,
            Self::AgentManagement => Icon::Grid,
            Self::CodeReview => Icon::Diff,
            Self::NotificationsMailbox => Icon::Inbox,
        }
    }

    /// Whether this item is supported on the current platform/configuration
    /// (feature flags, compile-time features, AI enabled, auth state).
    /// Does not check user show/hide preferences — use `is_available` for that.
    pub fn is_supported(&self, app: &AppContext) -> bool {
        match self {
            Self::TabsPanel => {
                FeatureFlag::VerticalTabs.is_enabled()
                    && *TabSettings::as_ref(app).use_vertical_tabs
            }
            Self::ToolsPanel => false,
            // warp-lite: AI/agent/code-review/notification toolbar items are
            // unsupported in pure-terminal mode (UI cleanup). Their backing
            // logic still compiles, but no header buttons surface them.
            Self::AgentManagement | Self::CodeReview | Self::NotificationsMailbox => false,
        }
    }

    /// Whether this item should be shown in the toolbar.
    /// Checks both `is_supported` and user show/hide preferences.
    pub fn is_available(&self, app: &AppContext) -> bool {
        // warp-lite: AI/CodeReview/Notifications are unsupported (see is_supported);
        // for the remaining variants, no per-user preference gates them off.
        self.is_supported(app)
    }

    /// Whether this item opens a side panel (as opposed to replacing the content
    /// area or opening a popover).
    pub fn is_panel(&self) -> bool {
        matches!(self, Self::TabsPanel | Self::CodeReview)
    }

    pub fn default_left() -> Vec<Self> {
        vec![Self::TabsPanel, Self::AgentManagement]
    }

    pub fn default_right() -> Vec<Self> {
        // warp-lite v0.3.6: ToolsPanel button hidden from the toolbar. The
        // Context Panel widgets do not yet read from Warp's existing
        // repo/cwd infrastructure (`repo_metadata`/`WorkingDirectoriesModel`)
        // reliably, so an empty panel was worse than no panel at all.
        // Keeping the variant in the enum so the rest of the codebase still
        // compiles; just removing it from the default set.
        vec![Self::CodeReview, Self::NotificationsMailbox]
    }

    /// All toolbar item variants (availability filtering is done at the call site).
    pub fn all_items() -> Vec<Self> {
        vec![
            Self::TabsPanel,
            Self::ToolsPanel,
            Self::AgentManagement,
            Self::CodeReview,
            Self::NotificationsMailbox,
        ]
    }
}
