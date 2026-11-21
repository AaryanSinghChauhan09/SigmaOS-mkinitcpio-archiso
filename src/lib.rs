//! SigmaOS mkinitcpio-archiso component.
//!
//! Rust implementation of `archlinux/mkinitcpio-archiso`: the initramfs hooks
//! and install scripts used to build Arch live ISO initramfs images.
//!
//! Upstream provides shell hooks (`archiso`, `archiso_loop_mnt`,
//! `archiso_pxe_common`, `archiso_pxe_http`, `archiso_pxe_nbd`,
//! `archiso_pxe_nfs`) plus matching `install/*` scripts that mkinitcpio
//! executes inside the airootfs chroot. This port models the hook metadata
//! and install-script semantics: HOOKS ordering/dependency resolution,
//! required-module collection, and generation of the install commands
//! mkinitcpio would run.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

pub mod error;
pub mod hook;
pub mod install;
pub mod profile;

pub use error::HookError;
pub use hook::{Hook, HookKind, KNOWN_HOOKS, REQUIRED_HOOKS, hook, required_modules, resolve_order};
pub use install::{InstallContext, InstallPlan, plan_installs};
pub use profile::parse_mkinitcpio_conf;
