<!-- Allow this file to not have a first line heading -->
<!-- markdownlint-disable-file MD041 -->

<!-- inline html -->
<!-- markdownlint-disable-file MD033 -->

<div align="center">

# 🌌 `rymder`

**Unofficial Rust client for [Agones](https://agones.dev/site/)**

[![Embark](https://img.shields.io/badge/embark-open%20source-blueviolet.svg)](https://embark.dev)
[![Embark](https://img.shields.io/badge/discord-ark-%237289da.svg?logo=discord)](https://discord.gg/dAuKfZS)
[![Crates.io](https://img.shields.io/crates/v/rymder.svg)](https://crates.io/crates/rymder)
[![Docs](https://docs.rs/rymder/badge.svg)](https://docs.rs/rymder)
[![dependency status](https://deps.rs/repo/github/EmbarkStudios/rymder/status.svg)](https://deps.rs/repo/github/EmbarkStudios/rymder)
[![Build status](https://github.com/EmbarkStudios/rymder/workflows/CI/badge.svg)](https://github.com/EmbarkStudios/rymder/actions)
</div>

## Usage

See the [Agones Rust guide](https://agones.dev/site/docs/tutorials/simple-gameserver-rust/) for context about how to use this client, which is _mostly_ the same as the official client in the [Agones repo](https://github.com/googleforgames/agones/tree/main/sdks/rust).

## Development

Generate new protocol buffers with by running [proto-gen](https://github.com/EmbarkStudios/proto-gen).  
`proto-gen --format --build-client generate -d src/proto -d src/proto/googleapis -f src/proto/sdk/sdk.proto -f src/proto/sdk/alpha/alpha.proto -o src/generated`

## Contribution

[![Contributor Covenant](https://img.shields.io/badge/contributor%20covenant-v1.4-ff69b4.svg)](CODE_OF_CONDUCT.md)

We welcome community contributions to this project.

Please read our [Contributor Guide](CONTRIBUTING.md) for more information on how to get started.
Please also read our [Contributor Terms](CONTRIBUTING.md#contributor-terms) before you make any contributions.

Any contribution intentionally submitted for inclusion in an Embark Studios project, shall comply with the Rust standard licensing model (MIT OR Apache 2.0) and therefore be dual licensed as described below, without any additional terms or conditions:

### License

This contribution is dual licensed under EITHER OF

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

For clarity, "your" refers to Embark or any other licensee/user of the contribution.
