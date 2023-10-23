use clap::Parser;

/// Scaffold a new post for your blog
#[derive(Parser, Debug)]
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
    output_dir: String,
}

fn main() {
    let args = Args::parse();
    dbg!(args);
}
