use anyhow::Result;
use clap::Parser;
use reqwest::Url;
use std::str::FromStr;

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
    #[arg(value_parser = parse_url)]
    url: String,
    /// HTTP body
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Clone, Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('=');
        let err = || anyhow::format_err!("Failed to parse {}", s);

        Ok(Self {
            k: (split.next().ok_or_else(err)?).into(),
            v: (split.next().ok_or_else(err)?).into(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<KvPair> {
    Ok(s.parse()?)
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
