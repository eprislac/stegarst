use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// If is to read or to hide
    #[arg(short, long)]
    pub option: String,

    /// The path to the file to read
    #[arg(short, long)]
    pub file: Option<String>,

    /// The path to the image to use
    #[arg(short, long)]
    pub image: String,

    /// The path to output the file
    #[arg(long)]
    pub output: String,
}
