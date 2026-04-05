# Raker - Your intelligent, context-aware Reviewer Agent

Raker is a powerful CLI designed to be used by AI and humans to cross-reference work against private cloud-scale context to verify and complete complex tasks. Built on Pinecone's Autocontext infrastructure, it acts as your comprehensive Reviewer Agent.

## Core Capabilities

- **Code Review:** Org-wide Code Intelligence to help review code that aligns with the company best practices and stack. Understands every issue known so far, indexed all public repos of interest. Can alert and identify problems before they hit production.
- **Design Review:** Check consistency of designs to stay on brand.
- **Doc Review:** Cross Reference 1000s of documents to identify plagiarism, check facts, overlaps, style defects, etc.
- **Security Review:** Cross Reference vulnerabilities and potential issues in unstructured environments.

## Getting Started

### Prerequisites

- [Rust toolchain](https://rustup.rs/) (cargo, rustc)

### Installation

You can install the Raker CLI locally using the provided install script:

```bash
./scripts/install.sh
```

Alternatively, you can build and install it directly with Cargo:

```bash
cd cli
cargo install --path .
```

## Usage

Use the `review` command to review the contents of a directory or file:

```bash
# Review current directory
raker review

# Review a specific file
raker review src/main.rs
```

*(For more CLI commands, run `raker --help`)*

## Contributing

We welcome contributions! Please review our [Repository Guidelines (AGENTS.md)](./AGENTS.md) before getting started.

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
