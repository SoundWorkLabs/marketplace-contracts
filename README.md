<div align="center">
  <img style="margin-bottom:15px" src="https://i0.wp.com/soundwork.io/wp-content/uploads/2023/05/2nd-logo_TINY.png?w=1120&ssl=1" height="80px" />
  <h1><strong>Soundwork Marketplace Contracts</strong></h1>
  <p>
    <strong>The Soundwork NFT Marketplace contracts.</strong>
   </p>
  <p>
    <a target="_blank" href="https://discord.gg/Jyw67UfQ"><img alt="Discord Chat" src="https://img.shields.io/badge/chat-discord-blueviolet" /></a>
    <a target="_blank" href="https://github.com/SoundWorkLabs//blob/master/LICENSE"><img alt="License" src="https://img.shields.io/github/license/SoundWorkLabs/marketplace-contracts" /></a>
    <a target="_blank" href="https://www.npmjs.com/package/@soundwork-oss/soundwork-sdk"><img alt="SDK" src="https://img.shields.io/npm/v/%40soundwork-oss%2Fsoundwork-sdk"/></a>
  </p>
</div>

<hr />

This repo contains the latest programs for [soundwork.io](https://soundwork.io/) protocol. Using the new [Metaplex core asset](https://developers.metaplex.com/core) users are able to perform all traditional operations of an NFT marketplace like list and bid operations trustlessly executed by our programs.

## Programs

This project contains the following programs:

**soundwork-list**

<!-- -   Mainnet-beta: -->

-   Devnet: `EA4ptgF3TYjDBGYJApAoZoyCbCYw6P5mGU5noCe1Z97`

**soundwork-bid**

<!-- -   Mainnet-beta: -->

-   Devnet: `4mFDYND4AVREYEJXCPhjq1LnbjELHHebJqG3NZechA7X`

## Audits

These programs are not audited, so fork, deploy and use them at your own risk.

## Developers

You can interact with the list and bid programs via our SDK.

Typescript SDK: [`@soundwork-oss/soundwork-sdk`](https://www.npmjs.com/package/@soundwork-oss/soundwork-sdk)

<!-- Rust Crate: [`@soundwork-oss/soundwork-sdk`](https://www.npmjs.com/package/@soundwork-oss/soundwork-sdk) -->

## Developing

### Environmental Setup

1. Install [Rust](https://rustup.rs/).
2. Install [Solana](https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool).
3. Install [Anchor](https://www.anchor-lang.com/docs/installation).

### Install Dependencies

```
yarn
```

### Build the Program

```
anchor build
```

### Testing

```
cargo clippy --all-targets -- -D warnings
anchor test
```

### Patches

Should you encounter `failed to select a version for the requirement toml_edit = "^0.21.0"`

```bash
cargo update -p toml_edit@0.21.1 --precise 0.21.0
```
