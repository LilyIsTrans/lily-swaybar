use anyhow::Context;
use replies::SwayNode;
use serde::Deserialize;
use std::io::{Read, Write};
use wrappers::*;
mod init;

pub const SWAY_MAGIC_STRING: &[u8; 6] = b"i3-ipc";

pub fn get_tree_hopefully() -> anyhow::Result<SwayNode> {
    let sock = init::get_sway_socket_path().context("Failed to get socket path!")?;
    let mut sock = std::os::unix::net::UnixStream::connect(sock)?;
    let mut message: Vec<u8> = Vec::new();
    message.extend_from_slice(SWAY_MAGIC_STRING);
    message.extend_from_slice(&0u32.to_ne_bytes());
    message.extend_from_slice(&sway_message_type::GET_TREE.to_ne_bytes());
    sock.write_all(&message)?;
    sock.flush()?;
    let mut magic_string = [0u8; 6];
    sock.read_exact(&mut magic_string)?;
    assert_eq!(SWAY_MAGIC_STRING, &magic_string);
    let mut payload_length = [0u8; 4];
    sock.read_exact(&mut payload_length)?;
    let payload_length = u32::from_ne_bytes(payload_length);
    let mut payload_type = [0u8; 4];
    sock.read_exact(&mut payload_type)?;
    let payload_type = u32::from_ne_bytes(payload_type);
    assert_eq!(payload_type, sway_message_type::GET_TREE);
    let mut message: Vec<u8> = Vec::with_capacity(payload_length as usize);
    // This is safe, because all we do with the buffer is immediately overwrite with data from the stream.
    #[allow(clippy::uninit_vec)]
    unsafe {
        message.set_len(payload_length as usize)
    };
    sock.read_exact(&mut message)?;
    sock.shutdown(std::net::Shutdown::Both)?;
    let mut file = std::fs::File::create("message.json")?;
    file.write_all(&message)?;
    Ok(serde_json::from_slice(&message)?)
}

pub mod wrappers;

pub mod replies;

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
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
    GetMarks(std::collections::HashSet<String>) = wrappers::sway_message_type::GET_MARKS,
    /// Either the list of bar ids, or the configuration of a given bar, depending on the payload of the request
    GetBarConfig(replies::GetBarConfigResult) = wrappers::sway_message_type::GET_BAR_CONFIG,
    /// Version information about sway itself (NOT ABOUT THIS CRATE!)
    GetVersion(replies::SwayVersionInfo) = wrappers::sway_message_type::GET_VERSION,
    /// The list of currently configured binding modes
    GetBindingModes(Vec<String>) = wrappers::sway_message_type::GET_BINDING_MODES,
    /// The text of the last loaded config file
    GetConfig { config: String } = wrappers::sway_message_type::GET_CONFIG,
    /// The status (success or failure) of the tick request
    SendTick { success: bool } = wrappers::sway_message_type::SEND_TICK,
    /// Always false
    Sync { success: bool } = wrappers::sway_message_type::SYNC,
    /// The currently active binding mode
    GetBindingState { name: String } = wrappers::sway_message_type::GET_BINDING_STATE,
    /// A list of the input devices currently available
    GetInputs(Vec<replies::SwayInput>) = wrappers::sway_message_type::GET_INPUTS,
    /// A list of the current seats
    GetSeats(Vec<replies::SwaySeat>) = wrappers::sway_message_type::GET_SEATS,
}
