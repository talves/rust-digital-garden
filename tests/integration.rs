use assert_cmd::Command;
use color_eyre::eyre::Result;

#[test]
fn test_help() -> Result<()> {
  let mut cmd = Command::cargo_bin("garden").unwrap();
  let assert = cmd.arg("--help").assert();
  assert.success().stderr("");
  Ok(());
}

#[test]
fn test_version() -> Result<()> {
  let mut cmd = Command::cargo_bin("garden").unwrap();
  let assert = cmd.arg("--version").assert();
  assert.success().stderr("");
  Ok(());
}
