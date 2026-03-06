# barcodefyi

[![crates.io](https://img.shields.io/crates/v/barcodefyi)](https://crates.io/crates/barcodefyi)
[![docs.rs](https://docs.rs/barcodefyi/badge.svg)](https://docs.rs/barcodefyi)

Async Rust client for the [BarcodeFYI](https://barcodefyi.com) API. Look up barcode symbologies (UPC, EAN, Code 128, QR), GS1 standards, industry applications, and encoding specifications.

## Install

```toml
[dependencies]
barcodefyi = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use barcodefyi::Client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let results = client.search("upc").await?;
    println!("Found {} results for 'upc'", results.total);
    Ok(())
}
```

## API Methods

| Method | Description |
|--------|-------------|
| `search(query)` | Search symbologies, standards, and glossary |
| `symbology(slug)` | Get barcode symbology details |
| `family(slug)` | Get barcode family details |
| `standard(slug)` | Get standard details |
| `component(slug)` | Get component details |
| `glossary_term(slug)` | Get glossary term definition |
| `compare(slug_a, slug_b)` | Compare two symbologies |
| `random()` | Get a random symbology |
| `industry(slug)` | Get industry application details |

All methods are async and return `Result<T, BarcodeFyiError>`.

## Also Available

| Language | Package | Install |
|----------|---------|---------|
| Python | [barcodefyi](https://pypi.org/project/barcodefyi/) | `pip install barcodefyi` |
| TypeScript | [barcodefyi](https://www.npmjs.com/package/barcodefyi) | `npm install barcodefyi` |
| Go | [barcodefyi-go](https://pkg.go.dev/github.com/fyipedia/barcodefyi-go) | `go get github.com/fyipedia/barcodefyi-go` |
| Rust | [barcodefyi](https://crates.io/crates/barcodefyi) | `cargo add barcodefyi` |
| Ruby | [barcodefyi](https://rubygems.org/gems/barcodefyi) | `gem install barcodefyi` |

## Code FYI Family

| Site | Domain | Focus |
|------|--------|-------|
| BarcodeFYI | [barcodefyi.com](https://barcodefyi.com) | Barcode symbologies & standards |
| QRCodeFYI | [qrcodefyi.com](https://qrcodefyi.com) | QR code types & encoding |
| NFCFYI | [nfcfyi.com](https://nfcfyi.com) | NFC chips & protocols |
| BLEFYI | [blefyi.com](https://blefyi.com) | Bluetooth Low Energy |
| RFIDFYI | [rfidfyi.com](https://rfidfyi.com) | RFID tags & readers |
| SmartCardFYI | [smartcardfyi.com](https://smartcardfyi.com) | Smart card platforms |

## License

MIT
