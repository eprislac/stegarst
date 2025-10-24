//! CLI definition for stegarst - A simple steganography tool
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
// CLI for stegarst - A simple steganography tool
// ## Options:
// ```
// -o, --option <OPTION>        Specify 'read' to extract a message or 'write' to hide a message
// -f, --file <FILE>           Path to the file to hide (required for 'write' option)
// -i, --image <IMAGE>         Path to the image file
// --output <OUTPUT>           Path to output the result (message file or image file)
// ```
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
