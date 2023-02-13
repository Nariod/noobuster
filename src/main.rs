use clap::Parser;
use reqwest::Client;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    time::Duration,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 't', long = "target", name = "Target URL")]
    target_url: String,
    #[arg(short = 'w', long = "wordlist", name = "Path to wordlist")]
    path_to_wordlist: String,
    #[arg(short = 'r', long = "responses", name = "Valid HTTP responses")]
    http_responses: String,
}

async fn forcer(
    http_client: Client,
    target: &str,
    http_values: &u16,
) -> Result<bool, reqwest::Error> {
    let response = http_client.get(target).send().await?.status();
    if response == *http_values {
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
    let http_responses = args.http_responses;
    let http_responses: Vec<&str> = http_responses.split(',').collect();
    let http_responses: Vec<u16> = http_responses
        .iter()
        .map(|x| x.to_string().parse::<u16>().expect("Inputs are not valid HTTP codes. Please check your arguments."))
        .collect();

    let reader = BufReader::new(&wordlist_file);
    let http_timeout = Duration::from_secs(10);
    let http_client = Client::builder().timeout(http_timeout).build()?;

    let mut handles = vec![];

    for line in reader.lines() {
        let line = line?;
        let common_filename = line.trim();
        let http_responses = http_responses.clone();

        for code in http_responses {
            let build = format!("{target}{common_filename}");
            let http_client = http_client.clone();
            let handle = tokio::spawn(async move {
                let result = forcer(http_client, &build, &code).await;
                if let Ok(content) = result {
                    if content {
                        println!("[{code}] {build}");
                    }
                }
            });
            handles.push(handle);
        }
    }

    for handle in handles {
        handle.await?;
    }

    Ok(())
}
