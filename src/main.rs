mod cli;
mod convert;
mod ipfs;

use anyhow::{Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    let input = read_input(args.file.as_deref())?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .context("failed to build HTTP client")?;

    let mut doc_count = 0usize;
    let mut errors = 0usize;

    for document in serde_yaml::Deserializer::from_str(&input) {
        doc_count += 1;

        let yaml_value = serde_yaml::Value::deserialize(document)
            .with_context(|| format!("failed to parse YAML document #{doc_count}"))?;

        let json_value = convert::yaml_to_json(yaml_value);

        match ipfs::dag_put(&client, &args.kubo_url, &json_value, args.verbose).await {
            Ok(cid) => println!("{cid}"),
            Err(e) => {
                eprintln!("Error on document #{doc_count}: {e:#}");
                errors += 1;
            }
        }
    }

    if doc_count == 0 {
        anyhow::bail!("no YAML documents found in input");
    }

    if errors > 0 {
        std::process::exit(1);
    }

    Ok(())
}

fn read_input(path: Option<&std::path::Path>) -> Result<String> {
    match path {
        Some(p) => std::fs::read_to_string(p)
            .with_context(|| format!("failed to read file '{}'", p.display())),
        None => {
            let mut buf = String::new();
            std::io::stdin()
                .read_to_string(&mut buf)
                .context("failed to read stdin")?;
            Ok(buf)
        }
    }
}
