mod client;
mod commands;
mod config;
mod output;
mod types;

use clap::{Parser, Subcommand};
use std::io::{self, Write};

    /// Raker CLI — manage contexts and tasks via the Autocontext API.
#[derive(Parser)]
#[command(name = "raker", version, about, long_about = None)]
struct Cli {
    /// Output JSON instead of human-readable text (for LLM/script consumption)
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Authenticate with a Pinecone API key
    Login {
        /// Override the API base URL
        #[arg(long, env = "AUTOCONTEXT_API_URL")]
        api_url: Option<String>,

        /// Pinecone API key (prompted if not provided)
        #[arg(long, env = "PINECONE_API_KEY")]
        api_key: Option<String>,
    },

    /// Clear local credentials
    Logout,

    /// Manage contexts
    #[command(subcommand)]
    Context(ContextCmd),

    /// Sync a directory to the remote context and trigger curation
    Sync {
        /// The directory to sync (defaults to current directory)
        #[arg(default_value = ".")]
        dir: String,
        /// Context ID (optional if active context is set)
        #[arg(long)]
        context_id: Option<String>,
    },

    /// Trigger build/learn process
    Learn {
        /// Context ID (optional if active context is set)
        #[arg(long)]
        context_id: Option<String>,
    },

    /// Status of current context and system
    Status {
        /// Context ID (optional if active context is set)
        #[arg(long)]
        context_id: Option<String>,
    },

    /// Show the CLI version
    Version,
}

#[derive(Subcommand)]
enum ContextCmd {
    /// Create a new context
    Create {
        /// Context name
        #[arg(long)]
        name: String,
        /// Environment (dev or prod, default dev)
        #[arg(long)]
        environment: Option<String>,
        /// Description
        #[arg(long)]
        description: Option<String>,
        /// Guardrails applied to all workflow system prompts (inline text or @filepath)
        #[arg(long)]
        guardrails: Option<String>,
    },

    /// List all contexts
    List,

    /// Delete a context
    Delete {
        /// Context ID (UUID)
        #[arg(long)]
        id: String,
    },

    /// Switch to a context for the current session
    Switch {
        /// Context ID (UUID)
        id: String,
    },
}

/// Prompt for a secret value from stdin (no echo if terminal).
fn prompt_secret(prompt: &str) -> String {
    eprint!("{prompt}");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read from stdin");
    input.trim().to_string()
}

/// Run an interactive loop for retrieving answers.
async fn interactive_loop(json: bool) -> anyhow::Result<()> {
    let aid = config::resolve_context_id(None)?;
    println!("Raker Interactive Mode (Context: {})", aid);
    println!("Type 'exit' or 'quit' to leave.");

    loop {
        print!("raker> ");
        io::stdout().flush()?;

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        if let Err(e) = commands::tasks::retrieve(&aid, input, Some(false), json).await {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}

async fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let json = cli.json;

    match cli.command {
        Some(Commands::Login { api_url, api_key }) => {
            let key = match api_key {
                Some(k) => k,
                None => prompt_secret("Enter Pinecone API key: "),
            };
            commands::auth::login(api_url.as_deref(), &key).await
        }
        Some(Commands::Logout) => commands::auth::logout().await,
        Some(Commands::Context(cmd)) => match cmd {
            ContextCmd::Create {
                name,
                environment,
                description,
                guardrails,
            } => {
                commands::contexts::create(
                    &name,
                    environment.as_deref(),
                    description.as_deref(),
                    guardrails.as_deref(),
                    json,
                )
                .await
            }
            ContextCmd::List => commands::contexts::list(json).await,
            ContextCmd::Delete { id } => commands::contexts::delete(&id).await,
            ContextCmd::Switch { id } => commands::contexts::switch(&id, json).await,
        },
        Some(Commands::Sync { dir, context_id }) => {
            let aid = config::resolve_context_id(context_id.as_deref())?;
            println!("Syncing directory '{}' to context '{}'...", dir, aid);
            commands::files::upload(&aid, &dir, None, json).await?;
            println!("Triggering curation task...");
            commands::tasks::curate(&aid, json).await
        }
        Some(Commands::Learn { context_id }) => {
            let aid = config::resolve_context_id(context_id.as_deref())?;
            commands::tasks::build(&aid, json).await
        }
        Some(Commands::Status { context_id: _ }) => {
            // Display active context and global stats
            let _ = commands::contexts::which(json).await;
            commands::stats::global(json).await
        }
        Some(Commands::Version) => {
            let version = env!("CARGO_PKG_VERSION");
            if json {
                output::print_json(&serde_json::json!({ "cli_version": version }));
            } else {
                println!("Raker CLI Version: {}", version);
            }
            Ok(())
        }
        None => interactive_loop(json).await,
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}
