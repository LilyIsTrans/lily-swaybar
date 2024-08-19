use crate::wrappers::*;
use serde::Deserialize;
use std::ffi::OsString;
#[derive(Deserialize, Clone, PartialEq)]
pub struct Output {
    pub(crate) name: OsString,
    pub(crate) make: OsString,
    pub(crate) model: OsString,
    pub(crate) serial: OsString,
    pub(crate) active: bool,
    /// Deprecated
    pub(crate) dpms: bool,
    pub(crate) power: bool,
    /// For i3 backwards-compatibility
    pub(crate) primary: bool,
    /// -1 if this output is disabled
    pub(crate) scale: f64,
    pub(crate) subpixel_hinting: SubpixelHinting,
    pub(crate) transform: Transform,
    pub(crate) current_workspace: Option<OsString>,
    pub(crate) modes: Vec<OutputMode>,
    pub(crate) current_mode: OutputMode,
    pub(crate) rect: Rect,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct SwayNode {
    pub(crate) id: u64,
    pub(crate) name: OsString,
    pub(crate) r#type: SwayNodeType, // Looks weird, but in Rust, r#name can be used for a raw identifier in the same way as r"text" can be used for raw strings, and as of version 1.0.73, serde correctly strips the r# from raw field identifiers
    pub(crate) border: SwayBorderStyle,
    pub(crate) current_border_width: u64,
    pub(crate) layout: SwayLayout,
    pub(crate) orientation: Orientation,
    pub(crate) percent: Option<f64>,
    pub(crate) rect: Rect,
    pub(crate) window_rect: Rect,
    pub(crate) deco_rect: Rect,
    pub(crate) geometry: Rect,
    #[serde(default)]
    pub(crate) urgent: bool,
    pub(crate) sticky: bool,
    #[serde(skip)]
    pub(crate) marks: (),
    pub(crate) focused: bool,
    pub(crate) focus: Vec<u64>,
    pub(crate) nodes: Vec<Self>,
    pub(crate) floating_nodes: Vec<Self>,
    #[serde(skip)]
    pub(crate) representation: (),
    pub(crate) fullscreen_mode: Option<SwayFullscreenMode>,
    pub(crate) app_id: Option<OsString>,
    pub(crate) pid: Option<u64>,
    pub(crate) visible: Option<bool>,
    pub(crate) shell: Option<OsString>,
    pub(crate) inhibit_idle: Option<bool>,
    pub(crate) idle_inhibitors: IdleInhibitors,
    pub(crate) window: Option<u64>,
    #[serde(skip)]
    pub(crate) window_properties: (),
}
