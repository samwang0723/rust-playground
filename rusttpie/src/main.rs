use anyhow::Result;
use clap::Parser;
use reqwest::Url;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

/// Get subcommand performs Get request and feedback response
#[derive(Parser, Debug)]
struct Get {
    /// HTTP URL
    #[arg(value_parser = parse_url)]
    url: String,
}

/// Post subcommand performs Post request and feedback response, post will use JSON
#[derive(Parser, Debug)]
struct Post {
    /// HTTP URL
    url: String,
    /// HTTP body
    body: Vec<String>,
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
