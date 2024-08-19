use crate::wrappers::*;
use serde::Deserialize;

use core::num::NonZeroU64;
use serde::de::Visitor;
use serde::Deserializer;

#[derive(Clone, Debug, Deserialize)]
pub struct SwayVersionInfo {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub human_readable: String,
    pub loaded_config_file_name: String,
}

fn de_nonzero<'de, D>(de: D) -> Result<Option<NonZeroU64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct NonZeroVisitor;
    impl<'de> Visitor<'de> for NonZeroVisitor {
        type Value = Option<NonZeroU64>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("A valid u64")
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(if v == 0u64 {
                None
            } else {
                Some(unsafe { NonZeroU64::new_unchecked(v) })
            })
        }
    }

    de.deserialize_u64(NonZeroVisitor)
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct BarConfigColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl<'de> Deserialize<'de> for BarConfigColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct ColorVisitor;
        impl<'de> Visitor<'de> for ColorVisitor {
            type Value = BarConfigColor;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("Expected a string of the format #RRGGBBAA, where RR, GG, BB, and AA are replaced with hexadecimal digits (i.e. #F0BA1234)")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if v.len() != 9 {
                    Err(serde::de::Error::invalid_length(v.len(), &self))
                } else {
                    let red = u8::from_str_radix(&v[1..=2], 16).map_err(|_| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&v[1..=2]),
                            &self,
                        )
                    })?;
                    let green = u8::from_str_radix(&v[3..=4], 16).map_err(|_| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&v[3..=4]),
                            &self,
                        )
                    })?;
                    let blue = u8::from_str_radix(&v[5..=6], 16).map_err(|_| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&v[5..=6]),
                            &self,
                        )
                    })?;
                    let alpha = u8::from_str_radix(&v[7..=8], 16).map_err(|_| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Str(&v[7..=8]),
                            &self,
                        )
                    })?;
                    Ok(BarConfigColor {
                        red,
                        green,
                        blue,
                        alpha,
                    })
                }
            }
        }
        deserializer.deserialize_str(ColorVisitor)
    }
}

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BarConfigColors {
    pub background: BarConfigColor,
    pub statusline: BarConfigColor,
    pub separator: BarConfigColor,
    pub focused_background: BarConfigColor,
    pub focused_statusline: BarConfigColor,
    pub focused_separator: BarConfigColor,
    pub focused_workspace_text: BarConfigColor,
    pub focused_workspace_bg: BarConfigColor,
    pub focused_workspace_border: BarConfigColor,
    pub active_workspace_text: BarConfigColor,
    pub active_workspace_bg: BarConfigColor,
    pub active_workspace_border: BarConfigColor,
    pub inactive_workspace_text: BarConfigColor,
    pub inactive_workspace_bg: BarConfigColor,
    pub inactive_workspace_border: BarConfigColor,
    pub urgent_workspace_text: BarConfigColor,
    pub urgent_workspace_bg: BarConfigColor,
    pub urgent_workspace_border: BarConfigColor,
    pub binding_mode_text: BarConfigColor,
    pub binding_mode_bg: BarConfigColor,
    pub binding_mode_border: BarConfigColor,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum GetBarConfigResult {
    IDs(Vec<String>),
    Config {
        id: String,
        mode: SwayBarMode,
        position: SwayBarPosition,
        status_command: String,
        font: String,
        workspace_buttons: bool,
        workspace_min_width: u64,
        binding_mode_indicator: bool,
        /// For i3 compatibility, always false
        verbose: bool,
        colors: BarConfigColors,
        gaps: Gaps,
        #[serde(deserialize_with = "de_nonzero")]
        bar_height: Option<NonZeroU64>,
        status_padding: u64,
        status_edge_padding: u64,
    },
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Output {
    pub(crate) name: String,
    pub(crate) make: String,
    pub(crate) model: String,
    pub(crate) serial: String,
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
    pub(crate) current_workspace: Option<String>,
    pub(crate) modes: Vec<OutputMode>,
    pub(crate) current_mode: OutputMode,
    pub(crate) rect: Rect,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct SwayNode {
    pub(crate) id: u64,
    pub(crate) name: String,
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
    pub(crate) marks: Option<std::collections::HashSet<String>>,
    pub(crate) focused: bool,
    pub(crate) focus: Vec<u64>,
    pub(crate) nodes: Vec<Self>,
    pub(crate) floating_nodes: Vec<Self>,
    pub(crate) representation: Option<String>,
    pub(crate) fullscreen_mode: Option<SwayFullscreenMode>,
    pub(crate) app_id: Option<String>,
    pub(crate) pid: Option<u64>,
    pub(crate) visible: Option<bool>,
    pub(crate) shell: Option<String>,
    pub(crate) inhibit_idle: Option<bool>,
    pub(crate) idle_inhibitors: Option<IdleInhibitors>,
    pub(crate) window: Option<u64>,
    #[serde(skip)]
    pub(crate) window_properties: (),
}

#[non_exhaustive]
#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SwayInputType {
    Keyboard,
    Pointer,
    Touch,
    TabletTool,
    TabletPad,
    Switch,
    /// Indicates that this crate is unaware of the device type in question, NOT that *sway* is unaware.
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LibInputSendEventsState {
    Enabled,
    Disabled,
    DisabledOnExternalMouse,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EnabledState {
    Enabled,
    Disabled,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ButtonMapping {
    Lmr,
    Lrm,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LibInputAccelProfile {
    None,
    Flat,
    Adaptive,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LibInputClickMethod {
    None,
    ButtonAreas,
    Clickfinger,
}
#[derive(Clone, Copy, PartialEq, Eq, Hash, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LibInputScrollMethod {
    None,
    TwoFinger,
    Edge,
    OnButtonDown,
}

#[derive(Clone, Copy, PartialEq, Deserialize, Debug)]
pub struct SwayLibinputDevice {
    pub send_events: Option<LibInputSendEventsState>,
    pub tap: Option<EnabledState>,
    pub tap_button_map: Option<ButtonMapping>,
    pub tap_drag: Option<EnabledState>,
    pub tap_drag_lock: Option<EnabledState>,
    pub accel_speed: Option<f64>,
    pub accel_profile: Option<LibInputAccelProfile>,
    pub natural_scroll: Option<EnabledState>,
    pub left_handed: Option<EnabledState>,
    pub click_method: Option<LibInputClickMethod>,
    pub middle_emulation: Option<EnabledState>,
    pub scroll_method: Option<LibInputScrollMethod>,
    pub scroll_button: Option<u64>, // TODO: Find out what type canonically represents an input event code from libinput and change this type to that
    pub scroll_button_lock: Option<EnabledState>,
    #[serde(rename = "dwt")]
    pub disable_while_typing: Option<EnabledState>,
    #[serde(rename = "dwtp")]
    pub disable_while_trackpointing: Option<EnabledState>,
    pub calibration_matrix: Option<[f64; 6]>,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct SwayInput {
    pub identifier: String,
    pub name: String,
    pub vendor: u64,
    pub product: u64,
    pub r#type: SwayInputType,
    pub xkb_active_layout_name: Option<String>,
    pub xkb_layout_names: Option<Vec<String>>,
    pub xkb_active_layout_index: Option<u64>,
    pub scroll_factor: Option<f64>,
    pub libinput: Option<SwayLibinputDevice>,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct SwaySeat {
    name: String,
    capabilities: u64,
    #[serde(deserialize_with = "de_nonzero")]
    focus: Option<NonZeroU64>,
    devices: Vec<SwayInput>,
}
