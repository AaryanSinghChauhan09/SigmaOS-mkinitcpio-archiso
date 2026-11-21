//! Integration tests for sigmaos-mkinitcpio-archiso.

use mkinitcpio_archiso::{
    InstallContext, HookError, REQUIRED_HOOKS, hook, parse_mkinitcpio_conf, plan_installs,
};

#[test]
fn test_builtin_hooks_present() {
    for name in [
        "archiso",
        "archiso_loop_mnt",
        "archiso_pxe_common",
        "archiso_pxe_http",
        "archiso_pxe_nbd",
        "archiso_pxe_nfs",
    ] {
        assert!(hook(name).is_some(), "missing hook {name}");
    }
}

#[test]
fn test_pxe_hook_dependency_order() {
    let ordered = mkinitcpio_archiso::resolve_order(&[
        "archiso_pxe_http".to_string(),
        "archiso_pxe_common".to_string(),
    ])
    .unwrap();
    let pos = |n: &str| ordered.iter().position(|x| x == n).unwrap();
    assert!(pos("archiso") < pos("archiso_pxe_common"));
    assert!(pos("archiso_pxe_common") < pos("archiso_pxe_http"));
}

#[test]
fn test_plan_installs_default_context() {
    let plan = plan_installs(&InstallContext::default()).unwrap();
    assert!(!plan.steps.is_empty());
    assert!(plan.all_modules.contains(&"loop".to_string()));
    // archiso hook must be present with storage modules.
    let archiso = plan.steps.iter().find(|s| s.hook == "archiso").unwrap();
    assert!(archiso.modules.contains(&"nvme".to_string()));
}

#[test]
fn test_plan_installs_with_conf() {
    let dir = tempfile::tempdir().unwrap();
    let conf = dir.path().join("mkinitcpio.conf");
    std::fs::write(
        &conf,
        "MODULES=(ahci)\nHOOKS=(archiso archiso_loop_mnt udev)\n",
    )
    .unwrap();
    let ctx = InstallContext {
        conf_path: Some(conf),
        ..Default::default()
    };
    let plan = plan_installs(&ctx).unwrap();
    assert!(plan.steps.iter().any(|s| s.hook == "archiso_loop_mnt"));
}

#[test]
fn test_pxe_enabled_adds_pxe_hooks() {
    let ctx = InstallContext {
        pxe_enabled: true,
        ..Default::default()
    };
    let plan = plan_installs(&ctx).unwrap();
    assert!(plan.steps.iter().any(|s| s.hook == "archiso_pxe_nfs"));
}

#[test]
fn test_required_hooks_subset() {
    for h in REQUIRED_HOOKS {
        // REQUIRED_HOOKS mixes archiso + standard mkinitcpio hooks; only
        // archiso* must exist in HOOKS_BUILTIN.
        if h.starts_with("archiso") {
            assert!(hook(h).is_some());
        }
    }
}

#[test]
fn test_parse_mkinitcpio_conf_arrays() {
    let conf = parse_mkinitcpio_conf("HOOKS=(a b c)\nMODULES=(x y)\n").unwrap();
    assert_eq!(conf.hooks, vec!["a", "b", "c"]);
    assert_eq!(conf.modules, vec!["x", "y"]);
}

#[test]
fn test_unknown_hook_is_kept_opaque() {
    // Standard mkinitcpio hooks not in HOOKS_BUILTIN must not error.
    let ordered = mkinitcpio_archiso::resolve_order(&[
        "udev".to_string(),
        "archiso".to_string(),
    ])
    .unwrap();
    assert!(ordered.contains(&"udev".to_string()));
}
