#[test]
fn test() {
    assert_cmd::Command::cargo_bin("hello-world")
        .unwrap()
        .assert()
        .success();
}
