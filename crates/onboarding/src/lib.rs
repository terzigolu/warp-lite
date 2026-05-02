// Onboarding library — INERT STUB (warp-lite v0.4)
//
// In upstream Warp this crate ships the full first-time-user experience
// (intro/intention slides, callout tutorial, agent model picker, third-party
// integration sign-up, etc.). For warp-lite we have removed AI-driven
// onboarding entirely; the crate is preserved as a typed surface so the rest
// of the application keeps compiling without sprinkling `#[cfg]` guards
// throughout `app/`.
//
// All view/model implementations render `Empty::new().finish()` and emit
// nothing at runtime. Public types stay constructible to keep call sites
// working unmodified.

#![allow(dead_code)]
#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use warpui::{
    elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext,
};

pub mod llm_id;

use crate::llm_id::LLMId;

// ---------------------------------------------------------------------------
// Top-level enums + constants used across `app/`.
// ---------------------------------------------------------------------------

/// The user's intention selected during onboarding slides.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnboardingIntention {
    Terminal,
    AgentDrivenDevelopment,
}

impl std::fmt::Display for OnboardingIntention {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OnboardingIntention::AgentDrivenDevelopment => write!(f, "agent_driven"),
            OnboardingIntention::Terminal => write!(f, "terminal"),
        }
    }
}

/// User-facing names of the AI features. In warp-lite these lists exist only
/// so the login slide can render a placeholder bullet list; they are never
/// used to advertise any product surface.
pub const AI_FEATURES: &[&str] = &[];
pub const WARP_DRIVE_FEATURES: &[&str] = &[];

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SessionDefault {
    #[default]
    Agent,
    Terminal,
}

impl std::fmt::Display for SessionDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionDefault::Agent => write!(f, "agent"),
            SessionDefault::Terminal => write!(f, "terminal"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OnboardingAuthState {
    LoggedOut,
    FreeUser,
    PayingUser,
}

#[derive(Clone, Debug)]
pub struct UICustomizationSettings {
    pub use_vertical_tabs: bool,
    pub show_conversation_history: bool,
    pub show_project_explorer: bool,
    pub show_global_search: bool,
    pub show_warp_drive: bool,
    pub show_code_review_button: bool,
}

impl UICustomizationSettings {
    pub fn agent_defaults() -> Self {
        Self {
            use_vertical_tabs: true,
            show_conversation_history: true,
            show_project_explorer: true,
            show_global_search: true,
            show_warp_drive: true,
            show_code_review_button: true,
        }
    }

    pub fn terminal_defaults() -> Self {
        Self {
            use_vertical_tabs: false,
            show_conversation_history: false,
            show_project_explorer: false,
            show_global_search: false,
            show_warp_drive: false,
            show_code_review_button: false,
        }
    }

    pub fn tools_panel_enabled(&self, _intention: &OnboardingIntention) -> bool {
        false
    }
}

#[derive(Clone, Debug)]
pub enum SelectedSettings {
    Terminal {
        ui_customization: Option<UICustomizationSettings>,
        cli_agent_toolbar_enabled: bool,
        show_agent_notifications: bool,
    },
    AgentDrivenDevelopment {
        agent_settings: slides::AgentDevelopmentSettings,
        project_settings: slides::ProjectOnboardingSettings,
        ui_customization: Option<UICustomizationSettings>,
    },
}

impl SelectedSettings {
    pub fn is_ai_enabled(&self) -> bool {
        false
    }

    pub fn is_warp_drive_enabled(&self) -> bool {
        false
    }
}

// ---------------------------------------------------------------------------
// Slides surface — only the items consumed outside this crate are kept.
// ---------------------------------------------------------------------------

pub mod slides {
    use super::*;

    pub use layout::*;
    pub use slide_content::*;

    #[derive(Clone, Debug)]
    pub struct OnboardingModelInfo {
        pub id: LLMId,
        pub title: String,
        pub icon: warp_core::ui::icons::Icon,
        pub requires_upgrade: bool,
        pub is_default: bool,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
    pub enum AgentAutonomy {
        Full,
        #[default]
        Partial,
        None,
    }

    impl std::fmt::Display for AgentAutonomy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                AgentAutonomy::Full => write!(f, "full"),
                AgentAutonomy::Partial => write!(f, "partial"),
                AgentAutonomy::None => write!(f, "none"),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct AgentDevelopmentSettings {
        pub selected_model_id: LLMId,
        pub autonomy: Option<AgentAutonomy>,
        pub cli_agent_toolbar_enabled: bool,
        pub session_default: SessionDefault,
        pub disable_oz: bool,
        pub show_agent_notifications: bool,
    }

    impl AgentDevelopmentSettings {
        pub fn new(default_model_id: LLMId) -> Self {
            Self {
                selected_model_id: default_model_id,
                autonomy: Some(AgentAutonomy::default()),
                cli_agent_toolbar_enabled: true,
                session_default: SessionDefault::Agent,
                disable_oz: false,
                show_agent_notifications: true,
            }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub enum ProjectOnboardingSettings {
        #[default]
        NoProject,
        Project {
            selected_local_folder: String,
            initialize_projects_automatically: bool,
        },
    }

    impl ProjectOnboardingSettings {
        pub fn from_path(path: Option<String>) -> Self {
            match path {
                None => ProjectOnboardingSettings::NoProject,
                Some(path) => ProjectOnboardingSettings::Project {
                    selected_local_folder: path,
                    initialize_projects_automatically: true,
                },
            }
        }
    }

    pub mod layout {
        use warpui::{elements::Empty, Element};

        pub const ONBOARDING_BG_PATH: &str = "";
        pub const TWO_COLUMN_MIN_WIDTH: f32 = 1120.0;

        #[derive(Clone, Copy)]
        pub enum HPadding {
            Fixed(f32),
            ProportionalBoth(f32),
            ProportionalLeft(f32),
            ProportionalRight { ratio: f32, left_offset: f32 },
        }

        #[derive(Clone, Copy)]
        pub enum TopMode {
            Ratio(f32),
        }

        #[derive(Clone, Copy)]
        pub enum ForegroundFit {
            Width,
            Height,
        }

        #[derive(Clone, Copy)]
        pub struct ForegroundLayout {
            pub h_padding: HPadding,
            pub top: TopMode,
            pub fit: ForegroundFit,
        }

        pub const FOREGROUND_LAYOUT_DEFAULT: ForegroundLayout = ForegroundLayout {
            h_padding: HPadding::Fixed(0.0),
            top: TopMode::Ratio(0.0),
            fit: ForegroundFit::Width,
        };

        pub fn static_left(
            _left: impl Fn() -> Box<dyn Element>,
            _right: impl FnOnce() -> Box<dyn Element>,
        ) -> Box<dyn Element> {
            Empty::new().finish()
        }

        pub fn onboarding_right_panel_with_bg(
            _path: &str,
            _layout: ForegroundLayout,
        ) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }

    pub mod slide_content {
        use warp_core::ui::appearance::Appearance;
        use warpui::{elements::ClippedScrollStateHandle, elements::Empty, Element};

        pub fn onboarding_slide_content(
            _children: Vec<Box<dyn Element>>,
            _bottom_nav: Box<dyn Element>,
            _scroll_state: ClippedScrollStateHandle,
            _appearance: &Appearance,
        ) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }
}

// ---------------------------------------------------------------------------
// Callout surface — consumed by `app/src/terminal/view.rs`.
// ---------------------------------------------------------------------------

pub mod callout {
    use super::*;

    pub fn init(_app: &mut AppContext) {}

    /// Final state when the onboarding callout completes.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FinalState {
        Submit,
        Initialize,
        Skip,
        Finish,
        BackToTerminal,
    }

    /// Query produced by the callout (typed by the user during the tutorial).
    #[derive(Clone, Debug, Default)]
    pub enum OnboardingQuery {
        #[default]
        None,
        TerminalCommand(String),
        AgentPrompt(String),
    }

    /// Display strings for keybindings shown in the onboarding callout.
    #[derive(Clone, Debug, Default)]
    pub struct OnboardingKeybindings {
        pub toggle_input_mode: String,
        pub submit_to_local_agent: String,
        pub submit_to_cloud_agent: String,
    }

    #[derive(Clone, Debug)]
    pub enum OnboardingCalloutViewEvent {
        StateUpdated,
        Completed { final_state: FinalState },
        EnterAgentModality,
        NaturalLanguageDetectionToggled(bool),
    }

    /// Onboarding callout view — inert in warp-lite.
    pub struct OnboardingCalloutView {
        _keybindings: OnboardingKeybindings,
    }

    impl OnboardingCalloutView {
        pub fn new_universal_input(
            _has_project: bool,
            _initial_natural_language_detection_enabled: bool,
            keybindings: OnboardingKeybindings,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self { _keybindings: keybindings }
        }

        pub fn new_agent_modality(
            _has_project: bool,
            _intention: OnboardingIntention,
            _initial_natural_language_detection_enabled: bool,
            keybindings: OnboardingKeybindings,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self { _keybindings: keybindings }
        }

        pub fn has_project(&self, _app: &AppContext) -> bool {
            false
        }

        pub fn start_onboarding(&mut self, _ctx: &mut ViewContext<Self>) {}

        pub fn is_onboarding_active(&self, _app: &AppContext) -> bool {
            false
        }

        pub fn prompt_string(&self, _app: &AppContext) -> String {
            String::new()
        }

        pub fn prompt(&self, _app: &AppContext) -> OnboardingQuery {
            OnboardingQuery::None
        }

        pub fn should_position_above_zero_state(&self, _app: &AppContext) -> bool {
            false
        }
    }

    impl Entity for OnboardingCalloutView {
        type Event = OnboardingCalloutViewEvent;
    }

    impl View for OnboardingCalloutView {
        fn ui_name() -> &'static str {
            "OnboardingCalloutView"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }

    /// Onboarding-callout dispatched actions are no-op in warp-lite.
    #[derive(Clone, Debug)]
    pub enum OnboardingCalloutViewAction {
        NextClicked,
        SkipClicked,
        BackToTerminalClicked,
    }

    impl TypedActionView for OnboardingCalloutView {
        type Action = OnboardingCalloutViewAction;
    }
}

pub use callout::{OnboardingCalloutView, OnboardingKeybindings};
pub use slides::ProjectOnboardingSettings;

// Re-exports kept for the upstream `app/` import paths.
pub mod telemetry {
    pub use super::OnboardingEvent;
}

// Workspace-side onboarding tutorial type lives in `app/`; the `OnboardingTutorial`
// re-export from upstream is now defined there directly.

// Some upstream call sites reference `OnboardingTutorial` via the onboarding
// crate, but in warp-lite the type lives in `app/src/workspace/view/onboarding.rs`.
// We don't re-export it from here.

// ---------------------------------------------------------------------------
// Agent onboarding view — visible to `app/src/lib.rs::init`.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum AgentOnboardingAction {
    Next,
    Back,
    Skip,
}

#[derive(Clone, Debug)]
pub enum AgentOnboardingEvent {
    ThemeSelected { theme_name: String },
    SyncWithOsToggled { enabled: bool },
    OnboardingCompleted(SelectedSettings),
    OnboardingSkipped,
    UpgradeRequested,
    UpgradeCopyUrlRequested,
    UpgradePasteTokenFromClipboardRequested,
    PrivacySettingsFromTerminalThemeSlideRequested,
    LoginFromWelcomeRequested,
    AppBecameActive,
}

#[derive(Default)]
pub struct AgentOnboardingView;

impl AgentOnboardingView {
    /// Stub constructor mirroring the upstream signature. All arguments are
    /// ignored; the view renders an `Empty` element and emits no events.
    #[allow(clippy::too_many_arguments)]
    pub fn new<Themes, Models>(
        _themes: Themes,
        _is_skippable: bool,
        _models: Models,
        _default_model_id: LLMId,
        _workspace_enforces_autonomy: bool,
        _agent_modality_enabled: bool,
        _free_user_no_ai_experiment: bool,
        _agent_price_cents: Option<i32>,
        _auth_state: OnboardingAuthState,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    /// No-op in warp-lite — the runtime onboarding tutorial has been removed.
    pub fn start_onboarding(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn use_vertical_tabs(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub fn set_agent_price_cents(&mut self, _cents: Option<i32>, _ctx: &mut ViewContext<Self>) {}

    pub fn set_onboarding_models(
        &mut self,
        _models: Vec<slides::OnboardingModelInfo>,
        _default_model_id: LLMId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn set_workspace_enforces_autonomy(
        &mut self,
        _value: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn free_user_no_ai_experiment(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub fn set_free_user_no_ai_experiment(
        &mut self,
        _value: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn advance_to_agent_step(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn set_auth_state(
        &mut self,
        _auth_state: OnboardingAuthState,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}

impl Entity for AgentOnboardingView {
    type Event = AgentOnboardingEvent;
}

impl View for AgentOnboardingView {
    fn ui_name() -> &'static str {
        "AgentOnboardingView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for AgentOnboardingView {
    type Action = AgentOnboardingAction;
}

// ---------------------------------------------------------------------------
// Telemetry — no-op variants kept for API compat.
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OnboardingEvent {
    Started,
    Completed,
    Skipped,
}

// ---------------------------------------------------------------------------
// Init.
// ---------------------------------------------------------------------------

pub fn init(_app: &mut AppContext) {}
