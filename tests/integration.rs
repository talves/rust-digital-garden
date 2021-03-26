use assert_cmd::Command;

#[test]
fn test_help() {
  let mut cmd = Command::cargo_bin("garden").unwrap();
  let assert = cmd.arg("--help").assert();
  assert.success().stderr("");
}

#[test]
fn test_version() {
  let mut cmd = Command::cargo_bin("garden").unwrap();
  let assert = cmd.arg("--version").assert();
  assert.success().stderr("");
}
