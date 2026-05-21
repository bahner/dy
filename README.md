# dag-yaml / `dy`

Convert YAML to [IPFS](https://ipfs.tech/) [dag-cbor](https://ipld.io/docs/codecs/known/dag-cbor/) IPLD nodes via the [Kubo](https://github.com/ipfs/kubo) HTTP API.

Write IPLD nodes in YAML — `dy` handles the conversion to JSON and the `dag put` call.

## Install

```bash
cargo install --path .
```

Or download a pre-built binary from the [Releases](../../releases) page.

## Usage

```bash
# From stdin
echo 'name: Alice' | dy

# From a file
dy data.yaml

# Multi-document YAML — one CID per document
dy multi.yaml

# Custom Kubo URL
dy --kubo-url http://localhost:5001 data.yaml

# Verbose: print full Kubo response to stderr
dy --verbose data.yaml
```

### Options

| Flag | Default | Description |
|---|---|---|
| `--kubo-url` | `http://127.0.0.1:5003` | Kubo API base URL |
| `--verbose` / `-v` | off | Print full Kubo JSON response to stderr |
| `FILE` | stdin | YAML file to read |

## How it works

1. Reads YAML from stdin or a file.
2. Parses each YAML document (multi-document files separated by `---` are supported).
3. Converts each document to JSON.
4. `POST`s the JSON to `{kubo-url}/api/v0/dag/put?store-codec=dag-cbor&input-codec=dag-json`.
5. Prints the resulting CID to stdout — one line per document.

## Example

```yaml
# node.yaml
kind: person
name: Alice
tags:
  - admin
  - user
meta:
  created: 2026-01-01
```

```bash
$ dy node.yaml
bafyreidfnfd7w5k33efvx7mkscctqpzk4nhb32jhbdgnqp5v5mwmobgbge
```

Verify with Kubo:

```bash
ipfs dag get bafyreidfnfd7w5k33efvx7mkscctqpzk4nhb32jhbdgnqp5v5mwmobgbge
```

## Build

```bash
make build      # debug
make release    # optimised (LTO + stripped)
make test       # unit tests
make check      # cargo check + clippy
make install    # install to ~/.cargo/bin
```

### Cross-compilation (requires [`cross`](https://github.com/cross-rs/cross))

```bash
make build-linux-gnu       # x86_64-unknown-linux-gnu
make build-linux-musl      # x86_64-unknown-linux-musl
make build-linux-arm-musl  # aarch64-unknown-linux-musl
make build-macos-arm       # aarch64-apple-darwin  (macOS host only)
make build-windows         # x86_64-pc-windows-gnu
make build-all             # all non-macOS targets
```

## Releases

Pushing a tag of the form `v<semver>` (e.g. `v1.0.0`) triggers GitHub Actions to build all platform targets and publish a GitHub Release with the binaries attached:

| File | Platform |
|---|---|
| `dy-aarch64-apple-darwin` | macOS ARM64 (Apple Silicon) |
| `dy-x86_64-unknown-linux-gnu` | GNU/Linux x86\_64 |
| `dy-x86_64-unknown-linux-musl` | musl Linux x86\_64 (static) |
| `dy-aarch64-unknown-linux-musl` | musl Linux ARM64 (static) |
| `dy-x86_64-pc-windows-gnu.exe` | Windows x86\_64 |

## Requirements

A running [Kubo](https://github.com/ipfs/kubo) node with the HTTP API enabled (default port `5001`; this tool defaults to `5003` — adjust with `--kubo-url`).

## License

MIT
