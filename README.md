# Raker - Contextual Intelligence CLI

Raker is a CLI product designed to help developers curate and manage contextual intelligence from any machine — local or remote — into Pinecone's Autocontext infrastructure.

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (cargo, rustc)

### Installation

You can install the Raker CLI locally using the provided install script:

```bash
./scripts/install.sh
```

This will build the project in release mode and install the `raker` binary into your Cargo bin directory (usually `~/.cargo/bin`), making it available in your terminal.

Alternatively, you can build and install it directly with Cargo:

```bash
cd cli
cargo install --path .
```

### Building from Source

To simply build the CLI without installing it globally:

```bash
./scripts/build.sh
```

The compiled binary will be located at `cli/target/release/raker`.

## Usage

*(CLI commands and usage instructions will be documented here as features are implemented)*

```bash
raker --help
```

## Contributing

We welcome contributions! Please review our [Repository Guidelines (AGENTS.md)](./AGENTS.md) before getting started. It contains important information about our coding standards, project structure, and commit message conventions.

### Development Checks

Before submitting a pull request, please ensure your changes pass all format and linting checks:

```bash
cd cli
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## What does RAKER stand for?

RAKER stands for **Reinforced Agentic Knowledge Engine and Retrieval**. It represents the core philosophy of this tool: a local agent that intelligently curates, refines (reinforces), and retrieves your personal or organizational knowledge from Pinecone's remote infrastructure.
