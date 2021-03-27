use std::path::PathBuf;

use color_eyre::eyre::{eyre, Context, Result};
use digital_garden::write;
use directories::UserDirs;
use structopt::StructOpt;

/// A CLI for the digital garden
///
/// Visit https://github.com/talves/rust-digital-garden
#[derive(StructOpt, Debug)]
#[structopt(name = "garden")]
struct Opt {
    #[structopt(parse(from_os_str), short = "p", long, env)]
    garden_path: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// write a post to your digital garden
    ///
    /// This command will open your $EDITOR, wait for you to write something, and then save the file to your garden
    Write {
        /// [optional] set a Title for what you are going to be writing about
        #[structopt(short, long)]
        title: Option<String>,
    },
}

fn get_default_garden_dir() -> Result<PathBuf> {
    let user_dirs = UserDirs::new().ok_or_else(|| eyre!("Couldn't find the home directory"))?;
    // return Err(eyre!("here"));
    Ok(user_dirs.home_dir().join(".garden"))
}

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();

    // we get the garden path here
    let garden_path = match opt.garden_path {
        Some(pathbuf) => Ok(pathbuf),
        None => get_default_garden_dir().wrap_err("`garden_path` was not defined"),
    }?;
    // find our command
    match opt.cmd {
        Command::Write { title } => write(garden_path, title),
    }
}
