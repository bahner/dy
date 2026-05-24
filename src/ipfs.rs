use anyhow::{bail, Context, Result};
use reqwest::multipart;
use serde_json::Value as JsonValue;

/// PUT a JSON value into IPFS as a dag-cbor node via the Kubo HTTP API.
///
/// Returns the CID string of the created node.
/// When `verbose` is true the full Kubo response is printed to stderr.
pub async fn dag_put(
    client: &reqwest::Client,
    kubo_url: &str,
    json: &JsonValue,
    verbose: bool,
) -> Result<String> {
    let json_bytes = serde_json::to_vec(json).context("failed to serialize JSON")?;

    let part = multipart::Part::bytes(json_bytes)
        .file_name("data")
        .mime_str("application/octet-stream")
        .context("invalid mime type")?;

    let form = multipart::Form::new().part("file", part);

    let url = format!("{}/api/v0/dag/put", kubo_url.trim_end_matches('/'));

    let response = client
        .post(&url)
        .query(&[
            ("store-codec", "dag-cbor"),
            ("input-codec", "dag-json"),
            ("pin", "false"),
        ])
        .multipart(form)
        .send()
        .await
        .with_context(|| format!("failed to connect to Kubo at {url}"))?;

    let status = response.status();
    let body_bytes = response
        .bytes()
        .await
        .context("failed to read Kubo response body")?;

    if body_bytes.is_empty() {
        bail!("Kubo returned HTTP {status} with an empty response body");
    }

    let body: JsonValue = serde_json::from_slice(&body_bytes).with_context(|| {
        let text = String::from_utf8_lossy(&body_bytes);
        format!("failed to parse Kubo response as JSON (raw body: {text})")
    })?;

    if verbose {
        eprintln!(
            "{}",
            serde_json::to_string_pretty(&body).unwrap_or_default()
        );
    }

    if !status.is_success() {
        let msg = body
            .get("Message")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown error");
        bail!("Kubo returned HTTP {status}: {msg}");
    }

    let cid = body
        .pointer("/Cid/~1")
        .and_then(|v| v.as_str())
        .with_context(|| format!("unexpected Kubo response shape: {body}"))?;

    Ok(cid.to_owned())
}
