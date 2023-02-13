mod error;
use clap::Parser;
pub use error::Error;
use reqwest::Client;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Duration,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 't', long = "target", name = "target_url")]
    target_url: String,
    #[arg(short = 'w', long = "wordlist", name = "path_to_wordlist")]
    path_to_wordlist: String,
}

async fn forcer(http_client: Client, target: &str) -> Result<bool, reqwest::Error> {
    let response = http_client.get(target).send().await?.status();
    if response == 200 {
        Ok(true)
    } else {
        Ok(false)
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();
    let target = args.target_url;
    let wordlist_file = File::open(args.path_to_wordlist)?;

    let reader = BufReader::new(&wordlist_file);
    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;

    let mut handles = vec![];

    for line in reader.lines() {
        let line = line?;
        let common_filename = line.trim();
        let build = format!("{target}{common_filename}");
        let http_client = http_client.clone();

        let handle = tokio::spawn(async move {
            let result = forcer(http_client, &build).await;

            if let Ok(content) = result {
                if content {
                    println!("[200] {build}");
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
