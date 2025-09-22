# BDIC Verifier

A simple desktop application for hashing Bitcoin extended public keys (xpubs) using SHA256. Built in Rust and the Iced GUI framework.

## Overview

BDIC Verifier provides a simple interface to:

- Validate Bitcoin extended public keys (xpubs)
- Compute SHA256 hashes of valid xpubs

### Why does BDIC Verifier exist?

We, [Resolvr](https://resolvr.io), offer insurance products to protect your self-custodial Bitcoin stack against loss and/or theft. To protect your privacy, we don't want to know what coins you control. However, if you make a claim and attest that your key(s) were lost, we need to be able to ensure that the coins aren't moved on-chain afterwards in order to detect fraudulent claims. For this reason, we require that new insurees provide a SHA256 hash of their xpubs. This way we don't know what coins you control, but you can prove it to us at a later point if necessary by providing us with your actual xpub, which we can then hash ourselves and compare to the hash you originally provided. This is conceptually similar to a password hash.

BDIC Verifier exists to allow you, the insuree, to produce a hash of your xpub without revealing the xpub itself to anyone. You can download a published release, or you can audit the code (it's less than 200 lines), then build and run it from source.

## Installation

### Prerequisites

- Rust 1.85 or later (can be installed [here](https://www.rust-lang.org/tools/install))

### Building and Running from Source

1. Clone the repository:

```bash
git clone https://github.com/resolvr-io/bdic-verifier.git
cd bdic-verifier
```

2. Build the project:

```bash
cargo build --release
```

3. Run the application:

```bash
cargo run --release
```

The compiled binary will be available at `target/release/bdic-verifier`.

### Example xpub

For testing, you can use this example xpub:

```
xpub661MyMwAqRbcFtXgS5sYJABqqG9YLmC4Q1Rdap9gSE8NqtwybGhePY2gZ29ESFjqJoCu1Rupje8YtGqsefD265TMg7usUDFdp6W1EGMcet8
```

Which should produce the following hash:

```
ec71164a05b609c13b85dc898cff717b30b992ed1dfb35e0d2827626d23272ee
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

Copyright (c) 2025 Resolvr
