<div align="center">
  <img height="170" src="https://raw.githubusercontent.com/openbook-dex/resources/main/brand/OpenBook-Logomark.svg" />

  <h1>Openbook</h1>

  <p>
    <strong>Openbook Rust Monorepo</strong>
  </p>

  <p>
    <a href="https://opensource.org/licenses/Apache-2.0"><img alt="License" src="https://img.shields.io/github/license/openbook-dex/program?color=blue" /></a>
  </p>
</div>

## Program Deployments

| Program     | Devnet                                         | Mainnet Beta                                  |
| ----------- | ---------------------------------------------- | --------------------------------------------- |
| [DEX](/dex) | `EoTcMgcDRTJVZDMZWBoU6rhYHZfkNTVEAfz3uUJRcYGj` | `srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX` |

## Note

- **Openbook is in active development so all APIs and protocols are subject to
  change.**
- **The code is unaudited. Use at your own risk.**

## Contributing

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup component add rustfmt
```

On Linux systems you may need to install additional dependencies. On Ubuntu,

```bash
sudo apt-get install -y pkg-config build-essential python3-pip jq
```

### Install Solana

Directions can be found
[here](https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool).

### Download the source

```bash
git clone https://github.com/openbook-dex/program.git
```

### Build, deploy, and test programs

See individual crates for documentation. For example, to build the dex see its
[README](https://github.com/openbook-dex/program/tree/master/dex).

## Running a local Solana cluster

The easiest way to run a local cluster is to use
[solana-test-validator](https://docs.solana.com/developing/test-validator).

## Directories

- `assert-owner`: Solana utility program for checking account ownership.
- `common`: Common rust utilities.
- `dex`: OpenBook DEX program and client utility.
- `scripts`: Bash scripts for development.
