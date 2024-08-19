use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;

/// Tries a few different methods to get the sway socket path.
pub fn get_sway_socket_path() -> Option<PathBuf> {
    std::env::var_os("SWAYSOCK")
        .map(std::path::PathBuf::from)
        .or_else(|| {
            std::process::Command::new("sway")
                .arg("--get-socketpath")
                .output()
                .ok()
                .map(|out| OsString::from_vec(out.stdout))
                .map(std::path::PathBuf::from)
        })
}
