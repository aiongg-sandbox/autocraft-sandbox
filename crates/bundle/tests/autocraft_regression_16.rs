use std::fs;

#[test]
fn text_before_first_header_is_an_error() {
    let root = std::env::temp_dir().join(format!("bundle-reg16-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let err = bundle::unpack(&root, "hello\n== a.txt\nhi\n").unwrap_err();
    assert!(err.to_string().contains("hello"), "{err}");
    assert!(!root.join("a.txt").exists());
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn bundle_starting_with_header_still_unpacks() {
    let root = std::env::temp_dir().join(format!("bundle-reg16ok-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let w = bundle::unpack(&root, "== a.txt\nhi\n").unwrap();
    assert_eq!(w.len(), 1);
    assert_eq!(fs::read_to_string(&w[0]).unwrap(), "hi\n");
    fs::remove_dir_all(&root).unwrap();
}
