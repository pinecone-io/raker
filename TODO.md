# Raker - Project TODOs & Roadmap

This document outlines the planned features and roadmap for the Raker CLI, inspired by the need for developers to seamlessly curate and manage contextual intelligence into Pinecone's Autocontext infrastructure. 

While the heavy lifting of knowledge curation (wiki compilation) and vector search happens remotely in Autocontext, Raker serves as the powerful local agent to orchestrate data movement, trigger refinements, and interact with the knowledge base.

## 1. Data Ingestion (The "Raw" Pipeline)
The goal is to make getting data from the developer's local environment or the web into Autocontext as frictionless as possible.

- [ ] **Local Directory Sync:** Implement `raker ingest local <dir>`. Watch and upload local files (Markdown, code, PDFs, images) into the remote Autocontext.
- [ ] **Web Clipper Equivalent:** Implement `raker ingest web <url>`. Fetch web pages, convert them to clean Markdown, automatically download embedded images, and push everything to Autocontext.
- [ ] **Repository Ingestion:** Implement `raker ingest repo <url>`. Clone and ingest GitHub/GitLab repositories, potentially understanding `.gitignore` and code structures.
- [ ] **Daemon Mode:** Create a background worker (`raker daemon start`) that watches a specific local `raw/` directory and automatically ingests new files into Autocontext without manual intervention.

## 2. Interaction & Querying (Q&A and Output)
Provide a rich CLI interface to query the remote knowledge base and format the outputs for local use.

- [ ] **Basic Q&A:** Implement `raker query "your complex question"`.
- [ ] **Rich Output Formats:** Support outputting answers not just to stdout, but rendering them into files. Add flags like `--format <markdown|marp|json>` and `--out <file>`.
- [ ] **Feedback Loop (Filing Outputs):** Add an option (e.g., `--save-to-context`) to take the LLM's generated answer/report and immediately ingest it back into the Autocontext knowledge base to enhance future queries.

## 3. Remote Maintenance & "Linting"
Expose commands to trigger the remote LLM to clean up, organize, and maintain the knowledge base.

- [ ] **Knowledge Health Checks:** Implement `raker lint` or `raker refine`. This triggers a remote autonomous task in Autocontext to scan the wiki for inconsistent data, deduplicate information, and flag missing data.
- [ ] **Connection Discovery:** A command to ask the remote LLM to suggest new connections, topics, or "missing" articles based on the currently ingested raw data.
- [ ] **Integrity Stats:** Implement `raker stats` to show the size of the remote knowledge base, number of articles, and health scores returned by the linting tasks.

## 4. IDE Integration & Local Viewing
While the "source of truth" and LLM wiki compilation happen remotely, developers still want to view the knowledge graph in their preferred local tools (like Obsidian).

- [ ] **Local Read-Only Sync:** Implement `raker export` or `raker sync-down`. This pulls down a compiled, read-only Markdown representation of the remote Autocontext wiki so users can open it in Obsidian or VSCode to browse the compiled knowledge visually.

## 5. Advanced / Future Explorations
- [ ] **Finetuning Triggers:** Implement `raker finetune start`. Trigger jobs on the remote Autocontext to generate synthetic Q&A pairs from the wiki and initiate a model fine-tuning process, embedding the knowledge into model weights.
- [ ] **Agentic Shell Tools:** Allow the remote Autocontext to request execution of safe, containerized local CLI tools via Raker to answer questions that require local machine state (e.g., "what's the git diff of my current project compared to the knowledge base?").