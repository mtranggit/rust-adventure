use camino::Utf8PathBuf;
use clap::{error::ErrorKind, CommandFactory, Parser};
use serde::Serialize;
use slug::slugify;
use std::fs;

/// Scaffold a new post for your blog
#[derive(Parser, Debug)]
#[clap(version)]

struct Args {
    /// The layout the post should use
    #[clap(short, long, default_value_t, value_enum)]
    layout: Layout,

    /// Tags to include
    #[clap(short, long = "tag")]
    tags: Vec<String>,

    /// The title of the post
    ///
    /// If not provided, the filename will be generated
    #[clap(short = 'T', long, default_value = "A post")]
    title: String,

    /// Should this post be published?
    #[clap(short, long, default_value_t, value_enum)]
    status: PostStatus,

    /// Location to put the file
    #[clap(short, long, default_value = "content")]
    output_dir: Utf8PathBuf,
}

// example to run the program
// `cargo run -- -t bevy -t rust -t shaders -T "New shaders in Bevy 0.11`
// or run the bin after cargo build
// `./target/debug/scaffold -t bevy -t rust -t shaders -T "New shaders in Bevy 0.11`
fn main() {
    let args = Args::parse();
    dbg!(&args);

    if !args.output_dir.exists() {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::ValueValidation,
            format!("output directory `{}` doesn't exist", args.output_dir),
        )
        .exit();
    }

    let slug = slugify(&args.title);
    let mut filename = args.output_dir.join(&slug);

    filename.set_extension("md");

    let frontmatter = Frontmatter {
        layout: args.layout,
        tags: args.tags,
        status: args.status,
        title: args.title.clone(),
        slug: slug,
    };
    let yaml = serde_yaml::to_string(&frontmatter).unwrap();
    let file_contents = format!(
        "{yaml}
---        

# {}",
        args.title
    );

    if let Err(error) = fs::write(&filename, file_contents) {
        let mut cmd = Args::command();
        cmd.error(
            ErrorKind::Io,
            format!("failed to write file at `{filename}`\n\t{error}"),
        )
        .exit();
    }
}

#[derive(Debug, Serialize)]
struct Frontmatter {
    layout: Layout,
    tags: Vec<String>,
    status: PostStatus,
    title: String,
    slug: String,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Layout {
    /// blog post
    #[default]
    Post,
    /// image gallery
    Gallery,
    /// code example
    Code,
}

#[derive(clap::ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
enum PostStatus {
    /// Draft, don't publish yet
    #[default]
    Draft,
    /// Needs Review
    NeedsReview,
    /// Publishable
    Publish,
}
