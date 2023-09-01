use clap::{ArgGroup, Parser};
use serde::Deserialize;
use std::process;

use arti_hyper::*;
use arti_client::{TorClient, TorClientConfig};
use hyper::body::Buf;
use hyper::Client;
use hyper_tls::HttpsConnector;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

static WEBLATE_ENDPOINT: &str = "https://weblate.securedrop.org/api/projects/securedrop/languages/?format=json";


#[derive(Deserialize, Debug)]
struct Language {
    language: String,
    code: String,
    total: u32,
    translated: u32,
    translated_percent: f64,
    total_words: u32,
    translated_words: u32,
    translated_words_percent: f64,
    total_chars: u32,
    translated_chars: u32,
    translated_chars_percent: f64,
}

#[derive(Deserialize, Debug)]
struct Listing {
    title: String,
    onion_address: String,
}


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

async fn get_weblate_data() -> Result<Vec<Language>> {
    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    // let client = Client::new();
    let url = WEBLATE_ENDPOINT.parse().unwrap();
    let res = client.get(url).await?;

    let body = hyper::body::aggregate(res).await?;
    let languages = serde_json::from_reader(body.reader())?;
    Ok(languages)
}

// fn get_directory_data() -> Result<Vec<Listing>, Error> {

// }

#[tokio::main]
async fn main() -> Result<()> {
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
    /*let weblate_data = match get_weblate_data() {
        Ok(weblate_data) => println!("Found {:?} languages on Weblate!", weblate_data.len()),
        Err(e) => {
            eprintln!("Error retrieving Weblate info: {:?}", e);
            process::exit(1);
        },
    };
    */
    let weblate_data = get_weblate_data().await?;
    println!("Found {:?} languages on Weblate!", weblate_data.len());
    Ok(())
}
