use crate::autostart::{launched_by_autostart, FLAG};

fn args(rest: &[&str]) -> std::vec::IntoIter<String> {
    let mut all = vec!["/usr/bin/zokute".to_string()];
    all.extend(rest.iter().map(|arg| arg.to_string()));
    all.into_iter()
}

#[test]
fn detects_the_autostart_flag() {
    assert!(launched_by_autostart(args(&[FLAG])));
    assert!(launched_by_autostart(args(&["--other", FLAG])));
}

#[test]
fn a_normal_launch_is_not_an_autostart_launch() {
    assert!(!launched_by_autostart(args(&[])));
    assert!(!launched_by_autostart(args(&["--other"])));
}

#[test]
fn ignores_the_flag_in_argv_zero() {
    assert!(!launched_by_autostart(vec![FLAG.to_string()].into_iter()));
}
