use std::process::{Command, Stdio};

//mod steamintegration;

#[cfg(feature = "integrations")]
pub mod integrations;

//mod traits;
#[cfg(feature = "components")]
pub mod components;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

// TODO: rewrite it to find this binary in PATH instead

/// Check if specified binary is available
/// 
/// ```
/// assert!(anime_launcher_sdk::is_available("bash"));
/// ```
#[allow(unused_must_use)]
#[tracing::instrument(level = "trace", ret)]
pub fn is_available(binary: &str) -> bool {
    tracing::trace!("Checking package availability");

    let Ok(mut child) = Command::new(binary).stdout(Stdio::null()).stderr(Stdio::null()).spawn() else {
        return false;
    };

    child.kill();

    true
}
