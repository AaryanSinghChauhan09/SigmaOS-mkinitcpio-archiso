//! Error types for the mkinitcpio-archiso component.

/// Errors that can occur while resolving or planning initramfs hooks.
#[derive(Debug, thiserror::Error)]
pub enum HookError {
    /// A named hook is not known to this component.
    #[error("unknown hook: {0}")]
    UnknownHook(String),

    /// A required hook was not included in the HOOKS list.
    #[error("missing required hook: {0}")]
    MissingRequiredHook(String),

    /// The mkinitcpio.conf could not be parsed.
    #[error("conf parse error: {0}")]
    ConfParse(String),

    /// An install plan step referenced a missing hook.
    #[error("install plan error: {0}")]
    InstallPlan(String),
}
