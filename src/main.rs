use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(group(
            ArgGroup::new("url_source")
                .required(true)
                .args(["filename", "directory"]),
    ))]
struct Args {
    // Specify a file of onion URLs to check, one per line
    #[arg(short, long)]
    filename: Option<String>,   //The Option<> bit is how you specify that the arg is..optional
    // Took too long to realise that, not sure if I feel dumb or annoyed

    // Check onion URLs listed in https://securedrop.org/directory
    #[arg(short, long)]
    directory: bool,

    // Save scan results in JSON format to the specified file
    #[arg(short, long)]
    output: Option<String>,

    // Maximum number of URLs to check
    #[arg(short, long)]
    num_to_check: Option<u8>,

    // Maximum number of parallel checks
    #[arg(short, long, default_value_t = 5)]
    workers: u8,
}

fn main() {
    let args = Args::parse();
    println!("Use directory? {}!", args.directory);
    if let Some(filename) = args.filename.as_deref() {
        println!("File: {}", filename);
    }

    if let Some(output) = args.output.as_deref() {
        println!("Output: {}", output);
    }

    if let Some(max_n) = args.num_to_check {
        println!("Max URLs to check: {}", max_n)
    }

    println!("Worker threads: {}", args.workers);
}
