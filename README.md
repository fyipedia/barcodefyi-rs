# barcodefyi

[![crates.io](https://img.shields.io/crates/v/barcodefyi.svg)](https://crates.io/crates/barcodefyi)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Rust client for the [BarcodeFYI](https://barcodefyi.com) REST API. Barcode formats. Uses `reqwest` for HTTP.

> **Explore at [barcodefyi.com](https://barcodefyi.com)** — interactive tools and comprehensive reference.

## Install

```toml
[dependencies]
barcodefyi = "0.1"
```

## Quick Start

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = barcodefyi::Client::new();
    let result = client.search("query")?;
    println!("{:?}", result);
    Ok(())
}
```

## Also Available

| Platform | Install | Link |
|----------|---------|------|
| **Python** | `pip install barcodefyi` | [PyPI](https://pypi.org/project/barcodefyi/) |
| **npm** | `npm install barcodefyi` | [npm](https://www.npmjs.com/package/barcodefyi) |
| **Go** | `go get github.com/fyipedia/barcodefyi-go` | [pkg.go.dev](https://pkg.go.dev/github.com/fyipedia/barcodefyi-go) |
| **Rust** | `cargo add barcodefyi` | [crates.io](https://crates.io/crates/barcodefyi) |
| **Ruby** | `gem install barcodefyi` | [rubygems](https://rubygems.org/gems/barcodefyi) |


## Links

- **Site**: [barcodefyi.com](https://barcodefyi.com)
- **API**: [barcodefyi.com/api/v1/](https://barcodefyi.com/api/v1/)
- **OpenAPI**: [barcodefyi.com/api/v1/schema/](https://barcodefyi.com/api/v1/schema/)
- **Glossary**: [barcodefyi.com/glossary/](https://barcodefyi.com/glossary/)
- **Guides**: [barcodefyi.com/guide/](https://barcodefyi.com/guide/)
- **Tools**: [barcodefyi.com/tools/](https://barcodefyi.com/tools/)
Part of the [FYIPedia](https://fyipedia.com) open-source developer tools ecosystem.

## Tag FYI Family

Part of the [FYIPedia](https://fyipedia.com) open-source developer tools ecosystem.

| Site | Domain | Focus |
|------|--------|-------|
| **BarcodeFYI** | [barcodefyi.com](https://barcodefyi.com) | Barcode formats, EAN, UPC, ISBN standards |
| BLEFYI | [blefyi.com](https://blefyi.com) | Bluetooth Low Energy, GATT, beacons |
| NFCFYI | [nfcfyi.com](https://nfcfyi.com) | NFC chips, tag types, NDEF, standards |
| QRCodeFYI | [qrcodefyi.com](https://qrcodefyi.com) | QR code types, versions, encoding modes |
| RFIDFYI | [rfidfyi.com](https://rfidfyi.com) | RFID tags, frequency bands, standards |
| SmartCardFYI | [smartcardfyi.com](https://smartcardfyi.com) | Smart cards, EMV, APDU, Java Card |

## License

MIT
