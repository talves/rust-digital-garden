use assert_cmd::Command;
use assert_fs::prelude::*;
use color_eyre::eyre::Result;
use predicates::prelude::*;

fn setup_command() -> (Command, assert_fs::TempDir) {
    let temp_dir = assert_fs::TempDir::new().unwrap();
    let mut cmd = Command::cargo_bin("garden").unwrap();
    let fake_editor_path = std::env::current_dir()
        .expect("expected in a dir")
        .join("tests")
        .join("fake-editor.sh");
    if !fake_editor_path.exists() {
        panic!("fake-editor.sh script could not be found")
    }

    cmd.env("EDITOR", fake_editor_path.into_os_string())
        .env("GARDEN_PATH", temp_dir.path());
    (cmd, temp_dir)
}

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
// #[ignore] // this is pretty dope, we'll remove (add comment) when implimented!
/// make sure we have a write sub-command by running `garden write`
fn test_write_without_filename() {
    let (mut cmd, temp_dir) = setup_command();

    let assert = cmd.arg("write").write_stdin("N\n".as_bytes()).assert();

    assert.success();

    temp_dir
        .child("testing.md")
        .assert(predicate::path::exists());
}

#[test]
// #[ignore] // this is pretty dope, we'll remove (add comment) when implimented!
/// make sure we have a write sub-command by running `garden write`
fn test_write() {
    // let temp_dir = assert_fs::TempDir::new().unwrap();

    // let mut cmd = Command::cargo_bin("garden").unwrap();
    // let fake_editor_path = std::env::current_dir()
    //     .expect("expected in a dir")
    //     .join("tests")
    //     .join("fake-editor.sh");
    // if !fake_editor_path.exists() {
    //     panic!("fake-editor.sh script could not be found")
    // }

    let (mut cmd, temp_dir) = setup_command();

    let assert = cmd
        .arg("write")
        .arg("-t")
        .arg("atitle")
        .write_stdin("N\n".as_bytes())
        .assert();

    assert.success();

    temp_dir
        .child("atitle.md")
        .assert(predicate::path::exists());
}
