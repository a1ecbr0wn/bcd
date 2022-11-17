use std::env;

/// Checks to see whether we are operating within a snap
pub(crate) fn check_in_snap() -> bool {
    env::var("SNAP_NAME").is_ok()
}

/// Check whether the dot-bashrc snap interface is connected
pub(crate) fn snap_connect_bashrc() -> bool {
    false
}

/// Check whether the dot-zshrc snap interface is connected
pub(crate) fn snap_connect_zshrc() -> bool {
    false
}
