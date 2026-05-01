//! warp-lite Phase 3.x: stub of the original `computer_use` crate.
//!
//! The real implementation drove macOS / Linux / Windows mouse + keyboard +
//! screenshot capture for AI Agent's "computer use" tool calls. AI removal
//! makes that surface unreachable, but downstream type signatures across
//! `ai` and `app` still mention these names. We keep the public types as
//! inert shells so the workspace continues to compile.
//!
//! No constructor in warp-lite produces these values; if a path ever does,
//! it must short-circuit to "Cancelled" / `Err(...)`.

#![allow(dead_code)]

use std::borrow::Cow;

use async_trait::async_trait;
pub use pathfinder_geometry::vector::Vector2I;
use serde::{Deserialize, Serialize};
use serde_with::{DurationSecondsWithFrac, serde_as};

/// Stub: the platform that computer use is running on.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Platform {
    Mac,
    Windows,
    LinuxX11,
    LinuxWayland,
}

/// Stub: returns false in warp-lite (computer use disabled).
pub fn is_supported_on_current_platform() -> bool {
    false
}

/// Stub: returns a no-op actor.
pub fn create_actor() -> Box<dyn Actor> {
    Box::new(NoopActor)
}

#[async_trait]
pub trait Actor: Send + Sync + 'static {
    fn platform(&self) -> Option<Platform>;
    async fn perform_actions(
        &mut self,
        actions: &[Action],
        options: Options,
    ) -> Result<ActionResult, String>;
}

struct NoopActor;

#[async_trait]
impl Actor for NoopActor {
    fn platform(&self) -> Option<Platform> {
        None
    }

    async fn perform_actions(
        &mut self,
        _actions: &[Action],
        _options: Options,
    ) -> Result<ActionResult, String> {
        Err("computer_use disabled in warp-lite".to_string())
    }
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum Key {
    Keycode(i32),
    Char(char),
}

#[serde_as]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Wait(#[serde_as(as = "DurationSecondsWithFrac<f64>")] std::time::Duration),
    MouseDown {
        button: MouseButton,
        #[serde(with = "Vector2IDef")]
        at: Vector2I,
    },
    MouseUp {
        button: MouseButton,
    },
    MouseMove {
        #[serde(with = "Vector2IDef")]
        to: Vector2I,
    },
    MouseWheel {
        #[serde(with = "Vector2IDef")]
        at: Vector2I,
        direction: ScrollDirection,
        distance: ScrollDistance,
    },
    TypeText {
        text: String,
    },
    KeyDown {
        key: Key,
    },
    KeyUp {
        key: Key,
    },
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum ScrollDistance {
    Pixels(i32),
    Clicks(i32),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScreenshotRegion {
    #[serde(with = "Vector2IDef")]
    pub top_left: Vector2I,
    #[serde(with = "Vector2IDef")]
    pub bottom_right: Vector2I,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub struct ScreenshotParams {
    pub max_long_edge_px: Option<usize>,
    pub max_total_px: Option<usize>,
    #[serde(default)]
    pub region: Option<ScreenshotRegion>,
}

pub struct Options {
    pub screenshot_params: Option<ScreenshotParams>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,
    Forward,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ActionResult {
    pub screenshot: Option<Screenshot>,
    pub cursor_position: Option<Vector2I>,
}

#[derive(Clone, Eq, PartialEq)]
pub struct Screenshot {
    pub width: usize,
    pub height: usize,
    pub original_width: usize,
    pub original_height: usize,
    pub data: Vec<u8>,
    pub mime_type: Cow<'static, str>,
}

impl std::fmt::Debug for Screenshot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Screenshot")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("original_width", &self.original_width)
            .field("original_height", &self.original_height)
            .field("num_data_bytes", &self.data.len())
            .finish()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(remote = "Vector2I")]
struct Vector2IDef {
    #[serde(getter = "get_vector2i_x")]
    x: i32,
    #[serde(getter = "get_vector2i_y")]
    y: i32,
}

fn get_vector2i_x(v: &Vector2I) -> i32 {
    v.x()
}

fn get_vector2i_y(v: &Vector2I) -> i32 {
    v.y()
}

impl From<Vector2IDef> for Vector2I {
    fn from(def: Vector2IDef) -> Self {
        Vector2I::new(def.x, def.y)
    }
}
