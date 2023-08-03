use std::str::FromStr;

use clap::{Args, Parser, Subcommand};
use reqwest::{Client, Url};

use anyhow::{anyhow, Ok, Result};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Get(Get),
    Post(Post),
}

#[derive(Debug, Args)]
struct Get {
    #[arg(value_parser=parse_url)]
    url: String,
}

#[derive(Debug, Args)]
struct Post {
    #[arg(value_parser=parse_url)]
    url: String,
    #[arg(value_parser=parse_kv)]
    body: Vec<KVPair>,
}

#[derive(Debug, Clone)]
struct KVPair {
    k: String,
    v: String,
}

impl FromStr for KVPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!("arg error");
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

fn parse_kv(s: &str) -> Result<KVPair> {
    Ok(s.parse()?)
}

fn get(client: &Client, params: &Get) {}
fn post(client: &Client, params: &Get) {}

fn main() {
    let opts = Opts::parse();
    let client = Client::new();
    match opts.command {
        Commands::Get(ref get) => {}
        Commands::Post(ref args) => {}
    }
    println!("Hello, world! {:?}", opts);
}
