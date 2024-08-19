use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::ffi::OsString;
#[derive(Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Rect {
    pub(crate) x: i64,
    pub(crate) y: i64,
    pub(crate) width: i64,
    pub(crate) height: i64,
}

#[derive(Deserialize, Clone, PartialEq)]
pub struct Workspace {
    pub(crate) num: i64,
    pub(crate) name: OsString,
    pub(crate) visible: bool,
    pub(crate) focused: bool,
    pub(crate) urgent: bool,
    pub(crate) rect: Rect,
    pub(crate) output: OsString,
}

#[derive(Deserialize, Clone)]
pub struct CommandResult {
    pub success: bool,
    #[serde(default)]
    pub parse_error: Option<bool>,
    #[serde(default)]
    pub error: Option<OsString>,
}

#[repr(u32)]
pub enum EventType {
    Workspace = 0x80000000,
    Output,
    Mode,
    Window,
    BarconfigUpdate,
    Binding,
    Shutdown,
    Tick,
    BarStateUpdate = 0x80000014,
    Input,
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SwayNodeType {
    Root,
    Output,
    Workspace,
    Con,
    FloatingCon,
}
#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SwayBorderStyle {
    Normal,
    None,
    Pixel,
    Csd,
}
#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SwayLayout {
    Splith,
    Splitv,
    Stacked,
    Tabbed,
    Output,
}
#[derive(Clone, Copy, Deserialize_repr, PartialEq, Eq, Hash)]
#[serde(untagged)]
#[repr(u32)]
pub enum SwayFullscreenMode {
    None = 0u32,
    Workspace = 1u32,
    Global = 2u32,
}
#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum IdleInhibitorApplication {
    Enabled,
    None,
}
#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum IdleInhibitorUser {
    Focus,
    Fullscreen,
    Open,
    Visible,
    None,
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub struct IdleInhibitors {
    application: IdleInhibitorApplication,
    user: IdleInhibitorUser,
}
#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SwayBarPosition {
    Bottom,
    Top,
}
#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum SwayBarMode {
    Dock,
    Hide,
    Invisible,
}
#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Orientation {
    Vertical,
    Horizontal,
    None,
}
#[derive(Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Gaps {
    top: u64,
    right: u64,
    bottom: u64,
    left: u64,
}
pub mod sway_message_type {
    /// Sway parses and runs the payload as sway commands
    pub const RUN_COMMAND: u32 = 0u32;
    /// No payload, retrieves the list of workspaces
    pub const GET_WORKSPACES: u32 = 1u32;
    /// Subscribe this IPC connection to the event types specified in the array of events.
    pub const SUBSCRIBE: u32 = 2u32;
    /// No payload, retrieves the list of outputs
    pub const GET_OUTPUTS: u32 = 3u32;
    /// No payload, retrieves the tree of nodes
    pub const GET_TREE: u32 = 4u32;
    /// No payload, retrieves the currently set marks
    pub const GET_MARKS: u32 = 5u32;
    /// If payload is [`None`], retrieves a list of all configured bar ids.
    /// If payload is a bar id, retrieves the config associated with the specified bar id.
    pub const GET_BAR_CONFIG: u32 = 6u32;
    /// No payload, retrieves version information about the sway process
    pub const GET_VERSION: u32 = 7u32;
    /// No payload, retrieves the list of binding modes that are currently configured
    pub const GET_BINDING_MODES: u32 = 8u32;
    /// No payload, retrieves the entire last loaded config file as a string (There are probably better ways to get any information you might find in there!)
    pub const GET_CONFIG: u32 = 9u32;
    /// Sends a Tick event (This library calls it [`EventType::Tick`]) to all clients of this sway instance which have subscribed to tick events.
    /// The optional payload is included with the tick event to all subscribed processes.
    pub const SEND_TICK: u32 = 10u32;
    /// This command just fails, it's only present present in Sway for i3 backwards compatibility,
    /// and only present in this library for completeness.
    pub const SYNC: u32 = 11u32;
    /// No payload, retrieves the current binding mode
    pub const GET_BINDING_STATE: u32 = 12u32;
    /// No payload, retrieves the list of input devices currently available
    pub const GET_INPUTS: u32 = 100u32;
    /// No payload, retrieves the list of seats currently configured
    pub const GET_SEATS: u32 = 101u32;
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum SubpixelHinting {
    Rgb,
    Bgr,
    Vrgb,
    Vbgr,
    None,
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum Transform {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "90")]
    Ninety,
    #[serde(rename = "180")]
    OneEighty,
    #[serde(rename = "270")]
    TwoSeventy,
    #[serde(rename = "flipped-90")]
    Flipped90,
    #[serde(rename = "flipped-180")]
    Flipped180,
    #[serde(rename = "flipped-270")]
    Flipped270,
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq, Hash)]
pub struct OutputMode {
    pub(crate) width: u64,
    pub(crate) height: u64,
    pub(crate) refresh: u64,
}
