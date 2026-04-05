use crate::client::RakerClient;
use crate::config::load_config;
use anyhow::{Context, Result};
use genai::chat::{ChatMessage, ChatRequest};
use genai::Client as GenAIClient;
use ignore::WalkBuilder;
use std::path::Path;

pub async fn run(
    context_id: &str,
    path_str: &str,
    review_type: Option<&str>,
    diff: bool,
    json: bool,
) -> Result<()> {
    let cfg = load_config()?;
    let _client = RakerClient::new(&cfg)?;

    let target_path = Path::new(path_str);
    if !target_path.exists() {
        anyhow::bail!("Path does not exist: {}", path_str);
    }

    // Traverse directory and collect file contents or diff
    let mut files_content = String::new();
    let mut extracted_symbols_text = String::new();

    if diff {
        println!("Extracting git diff for '{}'...", path_str);
        let output = std::process::Command::new("git")
            .args(["diff", "HEAD", path_str])
            .output()
            .context("Failed to execute git diff")?;

        if !output.status.success() {
            anyhow::bail!(
                "git diff failed: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }

        let diff_text = String::from_utf8_lossy(&output.stdout).to_string();
        if diff_text.trim().is_empty() {
            println!("No diff found to review.");
            return Ok(());
        }
        files_content = format!("--- GIT DIFF ---\n{}\n", diff_text);
    } else {
        let mut total_size = 0;
        let size_limit = 1_000_000; // 1MB text limit for initial pass

        println!("Scanning files in '{}'...", path_str);

        let walker = WalkBuilder::new(target_path)
            .hidden(true) // ignore hidden files
            .git_ignore(true)
            .build();

        for result in walker {
            let entry = result?;
            if !entry.file_type().is_some_and(|ft| ft.is_file()) {
                continue;
            }

            let path = entry.path();
            if let Ok(content) = std::fs::read_to_string(path) {
                total_size += content.len();
                if total_size > size_limit {
                    eprintln!(
                        "Warning: Exceeded {} bytes. Truncating file read...",
                        size_limit
                    );
                    break;
                }

                if let Some(symbols) = crate::commands::parse::extract_symbols(path, &content) {
                    if !symbols.imports.is_empty() || !symbols.definitions.is_empty() {
                        extracted_symbols_text.push_str(&format!("- File {}:\n", path.display()));
                        if !symbols.imports.is_empty() {
                            extracted_symbols_text
                                .push_str(&format!("  Imports: {:?}\n", symbols.imports));
                        }
                        if !symbols.definitions.is_empty() {
                            extracted_symbols_text
                                .push_str(&format!("  Definitions: {:?}\n", symbols.definitions));
                        }
                    }
                }

                files_content.push_str(&format!(
                    "\n--- File: {} ---\n{}\n",
                    path.display(),
                    content
                ));
            }
        }

        if files_content.is_empty() {
            println!("No text files found to review.");
            return Ok(());
        }
    }

    println!("Constructing review prompt and initiating agentic loop...");

    // Setup Gemini API Client via genai using environment variables
    let _api_key = std::env::var("GEMINI_API_KEY")
        .context("GEMINI_API_KEY environment variable is required for local agent loop")?;

    let ai_client = GenAIClient::default();

    // Initial analysis prompt
    let r_type = review_type.unwrap_or("general (infer from content)");

    let json_instruction = if json {
        "Because the user requested JSON output, your FINAL_REVIEW must be EXACTLY a valid JSON object matching this schema:\n\
        { \"status\": \"pass|fail\", \"severity\": \"high|medium|low|none\", \"comments\": [{\"line\": 42, \"message\": \"...\"}] }\n\
        Do NOT wrap it in markdown block quotes (```json ... ```). Output raw JSON only."
    } else {
        ""
    };

    let extracted_instruction = if !extracted_symbols_text.is_empty() {
        format!("Here is a structural summary of the code you are reviewing (extracted via AST parsing):\n{}\n", extracted_symbols_text)
    } else {
        String::new()
    };

    let system_prompt = format!(
        "You are an expert Reviewer Agent specializing in {} review. \
        You have access to a remote Autocontext knowledge base with ID: {}. \
        You are part of a loop. I will provide you with the contents of the files to review (or their diffs). \
        {} \
        Before making your final review, you should ask questions about the organizational context by outputting specific queries. \
        If you need to query the context for past PRs, issues, or code, output exactly: `QUERY_CONTEXT: <your query>` \
        I will then provide you the summarized JSON results of that query from the Autocontext backend. \
        Once you have enough context, output your final review starting with `FINAL_REVIEW:`\n\
        {}",
        r_type, context_id, extracted_instruction, json_instruction
    );

    let mut chat_req = ChatRequest::new(vec![
        ChatMessage::system(system_prompt),
        ChatMessage::user(files_content),
    ]);

    println!("Thinking (Looping with context)...");

    let mut retries = 0;

    loop {
        let response = ai_client
            .exec_chat("gemini-2.5-flash", chat_req.clone(), None)
            .await?;

        if let Some(content) = response.into_first_text() {
            if content.contains("QUERY_CONTEXT:") {
                // Extract query
                let query = content
                    .lines()
                    .find(|l| l.starts_with("QUERY_CONTEXT:"))
                    .unwrap_or("")
                    .replace("QUERY_CONTEXT:", "")
                    .trim()
                    .to_string();

                if !json {
                    println!("Agent queried context: {}", query);
                }

                // Perform actual query to Autocontext
                use crate::types::CreateTaskRequest;
                let req = CreateTaskRequest {
                    instruction: format!("Retrieve context for: {}", query),
                    background: Some(false),
                    timeout_seconds: None,
                };

                let context_result;
                match _client.create_task(context_id, "retrieve", &req).await {
                    Ok(task) => {
                        if let Some(output) = task.output {
                            if let Some(text) = output.get("result").and_then(|v| v.as_str()) {
                                context_result = text.to_string();
                            } else {
                                context_result = format!("{:?}", output);
                            }
                        } else {
                            context_result = "No specific context found.".to_string();
                        }
                    }
                    Err(e) => {
                        context_result = format!("Error retrieving context: {}", e);
                    }
                }

                chat_req = chat_req.append_message(ChatMessage::assistant(content.clone()));
                chat_req = chat_req.append_message(ChatMessage::user(format!(
                    "Context Result (JSON):\n{}",
                    context_result
                )));
            } else if content.contains("FINAL_REVIEW:")
                || (!content.contains("QUERY_CONTEXT:") && !content.trim().is_empty())
            {
                let final_review = content.replace("FINAL_REVIEW:", "").trim().to_string();

                if json {
                    // Try parsing it
                    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&final_review);
                    if let Ok(json_val) = parsed {
                        println!("{}", serde_json::to_string_pretty(&json_val).unwrap());

                        // CI/CD Exit logic
                        if let Some(status) = json_val.get("status").and_then(|s| s.as_str()) {
                            if status.eq_ignore_ascii_case("fail") {
                                std::process::exit(1);
                            }
                        }
                        break;
                    } else {
                        // Self correction
                        if retries < 1 {
                            chat_req =
                                chat_req.append_message(ChatMessage::assistant(content.clone()));
                            chat_req = chat_req.append_message(ChatMessage::user(
                                "Your output was not valid JSON. Fix it and output ONLY valid JSON matching the schema.".to_string()
                            ));
                            retries += 1;
                            continue;
                        } else {
                            // Give up
                            eprintln!(
                                "Failed to generate valid JSON review. Raw output:\n{}",
                                final_review
                            );
                            std::process::exit(2);
                        }
                    }
                } else {
                    println!("\n=== Final Review Report ===\n");
                    println!("{}", final_review);
                    break;
                }
            } else {
                if !json {
                    println!("No readable output.");
                }
                break;
            }
        } else {
            if !json {
                println!("No review output generated.");
            }
            break;
        }
    }

    Ok(())
}
