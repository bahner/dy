use clap::Parser;

/// Convert YAML to IPFS dag-cbor IPLD nodes via Kubo.
///
/// Reads from FILE if provided, otherwise reads from stdin.
/// For multi-document YAML (separated by ---), one node is created per document.
/// Each CID is printed to stdout on its own line.
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Kubo API base URL
    #[arg(long, default_value = "http://127.0.0.1:5003")]
    pub kubo_url: String,

    /// Print the full Kubo API response to stderr in addition to the CID
    #[arg(long, short)]
    pub verbose: bool,

    /// YAML file to read (reads stdin if omitted)
    pub file: Option<std::path::PathBuf>,
}
