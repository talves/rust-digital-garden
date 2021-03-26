use color_eyre::eyre::Result;
use digital_garden::write; // doesn't exist right now
use structopt::StructOpt;

/// A CLI for the digital garden
///
/// Visit https://github.com/talves/rust-digital-garden
#[derive(StructOpt, Debug)]
#[structopt(name = "garden")]
struct Opt {
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

fn main() -> Result<()> {
    color_eyre::install()?;
    let opt = Opt::from_args();
    dbg!(&opt);
    match opt.cmd {
        Command::Write { title } => write(title),
    }
}
