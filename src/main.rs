
mod error;
pub use error::Error;
use std::{env, fs::File, io::{BufReader, BufRead}, time::Duration};
use reqwest::Client;
use clap::Parser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    target_url: String,
    #[arg(short, long)]
    path_to_wordlist: String,
 }

async fn forcer(http_client: &Client, target: &str) -> Result<bool, reqwest::Error> {
    let response = http_client
    .get(target)
    .send()
    .await?
    .status();

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

    /* 
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        return Err(Error::CliUsage.into());
    }
    let target = args[1].as_str();
    let wordlist_file = File::open(&args[2])?;
    */
    let reader = BufReader::new(&wordlist_file);
    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;
    
    for line in reader.lines() {
        let line = line?;
        let common_filename = line.trim();
        let build = format!("{target}/{common_filename}");

        let lol = forcer(&http_client, &build).await?;

        if lol == true {
            println!("[200] {}", format!("{target}/{common_filename}"));
        }
    }

    Ok(())
}
