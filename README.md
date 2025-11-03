# ASIMOV Valkey Module

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Package on Crates.io](https://img.shields.io/crates/v/asimov-valkey-module)](https://crates.io/crates/asimov-valkey-module)
[![Documentation](https://docs.rs/asimov-valkey-module/badge.svg)](https://docs.rs/asimov-valkey-module)

[ASIMOV] module for [Valkey] / [Redis] database integration.

## ✨ Features

- Connects to Valkey or Redis instances via [`redis`] crate
- Scans, reads, and writes keys, values, and Pub/Sub messages
- Supports `.env` configuration with [ASIMOV Module] utilities
- CLI interface consistent with all ASIMOV ecosystem modules
- Distributed as static binaries with zero runtime dependencies

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition) if building from source code
- Running [Valkey] or [Redis] server (default: `redis://localhost:6379/0`)

## ⬇️ Installation

### Installation with the [ASIMOV CLI]

```bash
asimov module install valkey -v
```

### Installation from Source Code

```bash
cargo install asimov-valkey-module
```

## 👉 Examples

### 🔍 Listing Keys and Values (Cataloger)
List keys or JSON values stored in Valkey.

```bash
# Output stored values (JSONL)
asimov-valkey-cataloger --output jsonl

# Output only keys / URLs
asimov-valkey-cataloger --output url
```

### 📡 Subscribing to Channels (Reader)
Subscribe to one or more Valkey channels and print incoming messages.

```bash
# Output messages as JSON lines
asimov-valkey-reader --output jsonl asimov:events

# Extract only "@id" URLs from JSON messages
asimov-valkey-reader --output url asimov:events
```

### ✏️ Publishing Messages (Writer)
Publish messages from stdin to Valkey Pub/Sub channels.

```bash
# Publish JSON objects
echo '{"@id":"https://example.com/r/1","k":"v"}' \
  | asimov-valkey-writer asimov:events

# Publish and also echo to stdout
echo '{"msg":"test"}' \
  | asimov-valkey-writer --union asimov:events
```

## ⚙ Configuration

### Connection URL

You can configure the Valkey connection via environment variable or `.env` file.

#### Example `.env` file:
```ini
ASIMOV_VALKEY_URL=redis://127.0.0.1:6379/1
```

#### Example inline:
```bash
ASIMOV_VALKEY_URL="redis://127.0.0.1:6379/1" asimov-valkey-cataloger
```

## 📚 Reference

### Installed Binaries

| Binary                    | Description                                       |
|:--------------------------|:--------------------------------------------------|
| `asimov-valkey-cataloger` | Scans Valkey and prints stored keys or values     |
| `asimov-valkey-reader`    | Subscribes to Valkey channels and prints messages |
| `asimov-valkey-writer`    | Publishes messages from stdin to Valkey channels  |


### `asimov-valkey-cataloger`

```
Usage: asimov-valkey-cataloger [OPTIONS]

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -o, --output <FORMAT>  Specify the output format [jsonl, url] [default: jsonl] [possible values: jsonl, url]
  -h, --help             Print help
```

### `asimov-valkey-reader`

```
Usage: asimov-valkey-reader [OPTIONS] <CHANNELS>...

Arguments:
  <CHANNELS>...  Channels to subscribe to

Options:
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (may be repeated for more verbosity)
  -V, --version          Print version information
  -o, --output <FORMAT>  Specify the output format [jsonl, url] [default: jsonl] [possible values: jsonl, url]
  -h, --help             Print help
```

### `asimov-valkey-writer`

```
Usage: asimov-valkey-writer [OPTIONS] <CHANNELS>...

Arguments:
  <CHANNELS>...  Channels to publish to

Options:
  -U, --union            Copy stdin to stdout
  -d, --debug            Enable debugging output
      --license          Show license information
  -v, --verbose...       Enable verbose output (repeat for more verbosity)
  -V, --version          Print version information
  -h, --help             Print help
```

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-modules/asimov-valkey-module.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-modules/asimov-valkey-module&text=asimov-valkey-module)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-modules/asimov-valkey-module&title=asimov-valkey-module)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-modules/asimov-valkey-module&t=asimov-valkey-module)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-modules/asimov-valkey-module)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-modules/asimov-valkey-module)

[ASIMOV]: https://asimov.sh
[ASIMOV CLI]: https://cli.asimov.sh
[Rust]: https://rust-lang.org
[Valkey]: https://valkey.io
[Redis]: https://redis.io
