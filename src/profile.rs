//! mkinitcpio.conf parsing.

use crate::HookError;

/// Parsed subset of `/etc/mkinitcpio.conf` relevant to hook planning.
#[derive(Debug, Clone, Default)]
pub struct MkinitcpioConf {
    /// MODULES=( ... ) entries.
    pub modules: Vec<String>,
    /// BINARIES=( ... ) entries.
    pub binaries: Vec<String>,
    /// FILES=( ... ) entries.
    pub files: Vec<String>,
    /// HOOKS=( ... ) entries.
    pub hooks: Vec<String>,
}

/// Parse a mkinitcpio.conf file.
///
/// Handles `KEY=(item item ...)` arrays and `KEY=value` scalars; `#` comments
/// and blank lines are ignored. Unknown keys are skipped.
pub fn parse_mkinitcpio_conf(text: &str) -> Result<MkinitcpioConf, HookError> {
    let mut conf = MkinitcpioConf::default();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        // Strip inline comments (naive but sufficient for conf format).
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        let items: Vec<String> = if value.starts_with('(') && value.ends_with(')') {
            value[1..value.len() - 1]
                .split_whitespace()
                .map(str::to_string)
                .collect()
        } else {
            vec![value.trim_matches('"').to_string()]
        };
        match key {
            "MODULES" => conf.modules = items,
            "BINARIES" => conf.binaries = items,
            "FILES" => conf.files = items,
            "HOOKS" => conf.hooks = items,
            _ => {}
        }
    }
    Ok(conf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_full_conf() {
        let text = r#"
# comment
MODULES=(ahci sd_mod sr_mod usb_storage uas nvme mmc_block loop)
BINARIES=(/usr/bin/foo)
FILES=(/etc/motd)
HOOKS=(base udev autodetect microcode modconf block filesystems keyboard keymap consolefont)
"#;
        let conf = parse_mkinitcpio_conf(text).unwrap();
        assert_eq!(conf.modules.len(), 8);
        assert_eq!(conf.hooks.len(), 10);
        assert!(conf.hooks.contains(&"block".to_string()));
    }

    #[test]
    fn test_parse_scalar_and_unknown() {
        let conf = parse_mkinitcpio_conf("FOO=bar\nHOOKS=(a b)\n").unwrap();
        assert_eq!(conf.hooks, vec!["a", "b"]);
        assert!(conf.modules.is_empty());
    }
}
