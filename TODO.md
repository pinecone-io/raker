# Raker - Project TODOs & Roadmap

This document outlines the planned features and roadmap for the Raker CLI, which acts as a Reviewer Agent for cross-referencing code, design, documentation, and security against private cloud-scale context.

## 1. Context Building (The Foundation)
To perform accurate reviews, Raker needs access to the org-wide knowledge base in Autocontext.
- [x] **Data Ingestion:** Sync local directories into Autocontext (`raker sync`).
- [ ] **Web & Repo Ingestion:** Implement features to ingest web pages or clone and index whole GitHub/GitLab repositories.

## 2. Review Engine (Core)
The primary capability is using context to review local files.
- [x] **File/Directory Review:** Implement `raker review <path>` to read local contents and execute retrieval tasks against Autocontext.
- [ ] **Targeted Review Subcommands:** Implement `raker review code`, `raker review design`, `raker review docs`, and `raker review security` for explicit review targeting and optimized prompts.
- [ ] **Diff Reviews:** Add support for `raker review --git-diff` to review only uncommitted or staged changes rather than entire files.

## 3. Automation & Agentic Integration
Raker is built to be used by both humans and AI agents.
- [x] **JSON Output:** Support `--json` flag for machine-readable output.
- [ ] **CI/CD Integration:** Provide actions/scripts to easily integrate `raker review` into GitHub Actions or GitLab CI to fail builds on critical anti-patterns or vulnerabilities.
- [ ] **Report Generation:** Add functionality to output review results as standard markdown reports or formats like SARIF for security tools.

## 4. Remote Maintenance
- [x] **Knowledge Building:** Trigger the remote LLM to compile and organize the knowledge base (`raker learn`).
- [ ] **Integrity Checks:** Implement commands to scan the remote wiki for inconsistent data or missing information.
