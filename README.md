# Raker - Your intelligent, context-aware Reviewer Agent

Raker is a powerful CLI used by AI and humans to review their work against cloud-scale context. Built on Pinecone's Autocontext infrastructure, it acts as your comprehensive Reviewer Agent.

## Core Capabilities

- **Code Review:** Apply organizational code intelligence to ensure commits align with internal best practices and your specific tech stack. Flag known anti-patterns, past bugs, and architectural deviations before they reach production.
- **Design Review:** Automatically review code and assets against your company's design system and brand guidelines to maintain visual and experiential consistency.
- **Doc Review:** Analyze extensive internal documentation to identify style deviations, factual inconsistencies, overlapping content, and potential plagiarism.
- **Security Review:** Proactively surface vulnerabilities and security risks by referencing historical incidents, unpatched flaws, and unstructured organizational security guidelines.

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
