use color_eyre::{eyre::WrapErr, Result};
use std::path::PathBuf;

pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<()> {
    dbg!(garden_path, title);
    let template = "# ";
    let content_from_user = edit::edit(template).wrap_err("unable to read writing")?;
    dbg!(content_from_user);

    todo!()
}
