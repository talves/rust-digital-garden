use assert_cmd::Command;
use color_eyre::eyre::Result;

#[test]
fn test_help() -> Result<()> {
  let mut cmd = Command::cargo_bin("garden")?;
  let assert = cmd.arg("--help").assert();
  assert.success().stderr("");
  let assert = cmd.arg("-h").assert();
  assert.success().stderr("");

  Ok(())
}

#[test]
fn test_version() -> Result<()> {
  let mut cmd = Command::cargo_bin("garden")?;
  let assert = cmd.arg("--version").assert();
  assert.success().stderr("");
  let assert = cmd.arg("-v").assert();
  assert.success().stderr("");

  Ok(())
}

#[test]
/// make sure we have a write help option by running `garden write -h [--help]`
fn test_write_help() -> Result<()> {
  let mut cmd = Command::cargo_bin("garden")?;
  cmd.arg("write").arg("-h").assert().success().stderr("");
  cmd = Command::cargo_bin("garden")?;
  cmd.arg("write").arg("--help").assert().success().stderr("");

  Ok(())
}

#[test]
#[ignore] // this is pretty dope, we'll remove (add comment) when implimented!
/// make sure we have a write sub-command by running `garden write`
fn test_write() {
  let mut cmd = Command::cargo_bin("garden").unwrap();
  let assert = cmd.arg("write").assert();
  assert.success().stderr("");
}
