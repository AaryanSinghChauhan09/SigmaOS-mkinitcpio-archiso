//! Hook definitions matching upstream `hooks/` and `install/` scripts.

use crate::HookError;

/// Kind of initramfs hook.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookKind {
    /// Runs inside the initramfs at boot (upstream `hooks/`).
    Runtime,
    /// Runs at image build time inside the airootfs chroot (upstream `install/`).
    Install,
}

/// An initramfs hook with its metadata.
#[derive(Debug, Clone)]
pub struct Hook {
    /// Hook name (matches upstream script basename).
    pub name: &'static str,
    /// Human description.
    pub description: &'static str,
    /// Hook dependencies (must run first).
    pub depends: &'static [&'static str],
    /// Kernel modules the hook requires to be installed.
    pub required_modules: &'static [&'static str],
    /// Hook kind.
    pub kind: HookKind,
}

/// All built-in hooks (runtime + install) from mkinitcpio-archiso.
pub static HOOKS: &[Hook] = &[
    // Runtime hooks (upstream hooks/*)
    Hook { name: "archiso", description: "Archiso live media support", depends: &["udev"], required_modules: &["ahci","sd_mod","sr_mod","usb_storage","uas","nvme","mmc_block","loop"], kind: HookKind::Runtime },
    Hook { name: "archiso_loop_mnt", description: "Archiso loop device mounting", depends: &["archiso"], required_modules: &["loop"], kind: HookKind::Runtime },
    Hook { name: "archiso_pxe_common", description: "Archiso PXE common functions", depends: &["archiso"], required_modules: &[], kind: HookKind::Runtime },
    Hook { name: "archiso_pxe_http", description: "Archiso PXE over HTTP", depends: &["archiso_pxe_common"], required_modules: &[], kind: HookKind::Runtime },
    Hook { name: "archiso_pxe_nbd", description: "Archiso PXE over NBD", depends: &["archiso_pxe_common"], required_modules: &["nbd"], kind: HookKind::Runtime },
    Hook { name: "archiso_pxe_nfs", description: "Archiso PXE over NFS", depends: &["archiso_pxe_common"], required_modules: &[], kind: HookKind::Runtime },
    // Install hooks (upstream install/*)
    Hook { name: "archiso_kms", description: "Archiso kernel mode setting helpers", depends: &[], required_modules: &[], kind: HookKind::Install },
];

/// Look up a built-in hook by name.
pub fn hook(name: &str) -> Option<&'static Hook> {
    HOOKS.iter().find(|h| h.name == name)
}

/// All known archiso hook names.
pub const KNOWN_HOOKS: &[&str] = &[
    "archiso",
    "archiso_loop_mnt",
    "archiso_pxe_common",
    "archiso_pxe_http",
    "archiso_pxe_nbd",
    "archiso_pxe_nfs",
    "archiso_kms",
];

/// Hooks that a SigmaOS live ISO initramfs must contain to boot from any
/// supported media (per upstream releng profile requirements).
pub const REQUIRED_HOOKS: &[&str] = &[
    "archiso",
    "archiso_loop_mnt",
    "udev",
    "block",
    "filesystems",
    "keyboard",
    "keymap",
    "consolefont",
    "autodetect",
];

/// Resolve a HOOKS list into a dependency-ordered execution plan.
///
/// Topologically sorts requested hooks so dependencies run first; unknown
/// names produce [`HookError::UnknownHook`]. Hooks not in [`hook`] (e.g.
/// standard mkinitcpio hooks like `udev`) are kept as opaque entries.
pub fn resolve_order(requested: &[String]) -> Result<Vec<String>, HookError> {
    let mut result = Vec::new();
    let mut visiting = std::collections::HashSet::new();
    let mut done = std::collections::HashSet::new();

    fn visit(
        name: &str,
        result: &mut Vec<String>,
        visiting: &mut std::collections::HashSet<String>,
        done: &mut std::collections::HashSet<String>,
    ) -> Result<(), HookError> {
        if done.contains(name) {
            return Ok(());
        }
        if !visiting.insert(name.to_string()) {
            return Err(HookError::ConfParse(format!("hook dependency cycle at {name}")));
        }
        if let Some(h) = hook(name) {
            for dep in h.depends {
                visit(dep, result, visiting, done)?;
            }
        }
        visiting.remove(name);
        done.insert(name.to_string());
        result.push(name.to_string());
        Ok(())
    }

    for name in requested {
        visit(name, &mut result, &mut visiting, &mut done)?;
    }
    Ok(result)
}

/// Collect all kernel modules required by the resolved hooks.
pub fn required_modules(ordered: &[String]) -> Vec<String> {
    let mut mods = std::collections::BTreeSet::new();
    for name in ordered {
        if let Some(h) = hook(name) {
            for m in h.required_modules {
                mods.insert(m.to_string());
            }
        }
    }
    mods.into_iter().collect()
}
