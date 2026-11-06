use bundle::{target, unpack};
use std::path::Path;

#[test]
fn refuses_parent_dir_components() {
    let root = Path::new("/out");
    assert!(target(root, "../etc/x").is_err());
    assert!(target(root, "a/../../x").is_err());
    assert!(target(root, "a/..").is_err());
    assert!(target(root, "..\\x").is_err());
    assert!(target(root, "C:\\x").is_err());
    assert!(target(root, "\\\\server\\share\\x").is_err());
    assert_eq!(target(root, "a/b.txt").unwrap(), root.join("a/b.txt"));
    assert_eq!(target(root, "./a.txt").unwrap(), root.join("./a.txt"));
}

#[test]
fn unpack_keeps_entries_below_root() {
    let base = std::env::temp_dir().join(format!("bundle-parent-dir-{}", std::process::id()));
    let root = base.join("out");
    let outside = base.join("outside.txt");
    let res = unpack(&root, "== ../outside.txt\nhello\n");
    let wrote_outside = outside.exists();
    let _ = std::fs::remove_dir_all(&base);
    assert!(res.is_err());
    assert!(!wrote_outside);
}
