mod cli;
mod convert;
mod ipfs;

use anyhow::{Context, Result};
use clap::Parser;
use std::io::Read;
use yaml_rust2::YamlLoader;

#[tokio::main]
async fn main() -> Result<()> {
    let args = cli::Args::parse();

    let input = read_input(args.file.as_deref())?;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .context("failed to build HTTP client")?;

    let documents =
        YamlLoader::load_from_str(&input).context("failed to parse YAML input")?;

    if documents.is_empty() {
        anyhow::bail!("no YAML documents found in input");
    }

    let mut errors = 0usize;

    for (idx, document) in documents.into_iter().enumerate() {
        let doc_count = idx + 1;
        let json_value = convert::yaml_to_json(document);

        match ipfs::dag_put(&client, &args.kubo_url, &json_value, args.verbose).await {
            Ok(cid) => println!("{cid}"),
            Err(e) => {
                eprintln!("Error on document #{doc_count}: {e:#}");
                errors += 1;
            }
        }
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
