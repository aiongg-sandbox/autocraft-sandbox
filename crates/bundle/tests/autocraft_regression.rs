use std::fs;

#[test]
fn duplicate_path_is_an_error_naming_the_path() {
    let root = std::env::temp_dir().join(format!("bundle-dup-{}", std::process::id()));
    let res = bundle::unpack(&root, "== a.txt\none\n== a.txt\ntwo\n");
    let _ = fs::remove_dir_all(&root);
    let err = res.expect_err("duplicate path must be rejected");
    assert!(err.to_string().contains("a.txt"), "got: {err}");
}

#[test]
fn distinct_paths_still_unpack() {
    let root = std::env::temp_dir().join(format!("bundle-ok-{}", std::process::id()));
    let w = bundle::unpack(&root, "== a.txt\none\n== b.txt\ntwo\n").unwrap();
    assert_eq!(w.len(), 2);
    fs::remove_dir_all(&root).unwrap();
}
