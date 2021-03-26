use color_eyre::{eyre::WrapErr, Result};
use edit::{edit_file, Builder};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::PathBuf;

const TEMPLATE: &[u8; 2] = b"# ";

pub fn write(garden_path: PathBuf, title: Option<String>) -> Result<()> {
    dbg!(&garden_path, &title);
    let (mut file, filepath) = Builder::new()
        .suffix(".md")
        .rand_bytes(5)
        .tempfile_in(&garden_path)
        .wrap_err("Failed to create WIP file")?
        .keep()
        .wrap_err("Failed to keep tempfile")?;
    file.write_all(TEMPLATE)?;
    // user edits the file in their fav editor before we read in the contents
    edit_file(filepath)?;

    // Read in the user's changes back from the file into a string
    let mut contents = String::new();
    file.seek(SeekFrom::Start(0))?;
    file.read_to_string(&mut contents)?;

    //use `title` if the user passed it in
    // otherwise find a heading in the markdown
    let document_title = title.or_else(|| {
        contents
            .lines()
            .find(|line| line.starts_with("# "))
            // md headings are required to have `# ` with a space
            .map(|maybe_line| maybe_line.trim_start_matches("# ").to_string())
    });

    // let template = "# ";
    // let content_from_user = edit::edit(template).wrap_err("unable to read writing")?;
    // dbg!(content_from_user);

    dbg!(contents, document_title);
    todo!()
}
