use serde::{de::Visitor, Deserialize, Deserializer};
use std::{ffi::OsString, num::NonZeroU64};
use wrappers::*;
mod init;

pub mod wrappers;

pub mod replies;

pub mod color;

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

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum GetBarConfigResult {
    IDs(Vec<OsString>),
    Config {
        id: OsString,
        mode: SwayBarMode,
        position: SwayBarPosition,
        status_command: OsString,
        font: OsString,
        workspace_buttons: bool,
        workspace_min_width: u64,
        binding_mode_indicator: bool,
        /// For i3 compatibility, always false
        verbose: bool,
        colors: color::Colors,
        gaps: Gaps,
        #[serde(deserialize_with = "de_nonzero")]
        bar_height: Option<NonZeroU64>,
        status_padding: u64,
        status_edge_padding: u64,
    },
}

#[derive(Deserialize, Clone)]
#[repr(u32)]
pub enum SwayMessageReply {
    /// Contains a vector of `CommandResult`s, one for each command submitted in the corresponding command message
    RunCommand(Vec<wrappers::CommandResult>) = wrappers::sway_message_type::RUN_COMMAND,
    /// Contains a vector of `Workspace`s, one for each extant workspace
    GetWorkspaces(Vec<wrappers::Workspace>) = wrappers::sway_message_type::GET_WORKSPACES,
    /// Indicates whether the subscription was successful
    Subscribe { success: bool } = wrappers::sway_message_type::SUBSCRIBE,
    /// A Vec of all the outputs sway can see
    GetOutputs(Vec<replies::Output>) = wrappers::sway_message_type::GET_OUTPUTS,
    /// A tree of all Sway nodes
    GetTree(replies::SwayNode) = wrappers::sway_message_type::GET_TREE,
    /// A set of all set marks
    GetMarks(std::collections::HashSet<OsString>) = wrappers::sway_message_type::GET_MARKS,
    /// Either the list of bar ids, or the configuration of a given bar, depending on the payload of the request
    GetBarConfig,
}
