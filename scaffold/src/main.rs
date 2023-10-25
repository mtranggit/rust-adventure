use clap::{error::ErrorKind, CommandFactory, Parser};
use std::{fs, path::PathBuf};

/// Scaffold a new post for your blog
#[derive(Parser, Debug)]
#[clap(version)]

struct Args {
    /// The layout the post should use
    #[clap(short, long, default_value = "post")]
    layout: String,

    /// Tags to include
    #[clap(short, long = "tag")]
    tags: Vec<String>,

    /// The title of the post
    ///
    /// If not provided, the filename will be generated
    #[clap(short = 'T', long, default_value = "A post")]
    title: String,

    /// Should this post be published?
    #[clap(short, long, default_value = "draft")]
    status: String,

    /// Location to put the file
    #[clap(short, long, default_value = "content")]
    output_dir: PathBuf,
}

// example to run the program
// `cargo run -- -t bevy -t rust -t shaders -T "New shaders in Bevy 0.11`
// or run the bin after cargo build
// `./target/debug/scaffold -t bevy -t rust -t shaders -T "New shaders in Bevy 0.11`
fn main() {
    let args = Args::parse();
    dbg!(&args);
    let mut filename = args.output_dir.join(&args.title);
    filename.set_extension("md");

    if let Err(error) = fs::write(&filename, args.title) {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::Io,
            format!(
                "failed to write file at `{}`\n\t{}",
                filename.display(),
                error
            ),
        )
        .exit();
    }
}
