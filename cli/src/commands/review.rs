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
    _json: bool,
) -> Result<()> {
    let cfg = load_config()?;
    let _client = RakerClient::new(&cfg)?;

    let target_path = Path::new(path_str);
    if !target_path.exists() {
        anyhow::bail!("Path does not exist: {}", path_str);
    }

    // Traverse directory and collect file contents
    let mut files_content = String::new();
    let mut total_size = 0;
    let size_limit = 1_000_000; // 1MB text limit for initial pass

    println!("Scanning files in '{}'...", path_str);

    let walker = WalkBuilder::new(target_path)
        .hidden(true) // ignore hidden files
        .git_ignore(true)
        .build();

    for result in walker {
        let entry = result?;
        if !entry.file_type().map_or(false, |ft| ft.is_file()) {
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

    println!("Constructing review prompt and initiating agentic loop...");

    // Setup Gemini API Client via genai using environment variables
    let _api_key = std::env::var("GEMINI_API_KEY")
        .context("GEMINI_API_KEY environment variable is required for local agent loop")?;
    
    let ai_client = GenAIClient::default();

    // Initial analysis prompt
    let r_type = review_type.unwrap_or("general (infer from content)");
    let system_prompt = format!(
        "You are an expert Reviewer Agent specializing in {} review. \
        You have access to a remote Autocontext knowledge base with ID: {}. \
        You are part of a loop. I will provide you with the contents of the files to review. \
        Before making your final review, you should ask questions about the organizational context by outputting specific queries. \
        If you need to query the context, output exactly: `QUERY_CONTEXT: <your query>` \
        I will then provide you the results of that query. \
        Once you have enough context, output your final review starting with `FINAL_REVIEW:`",
        r_type, context_id
    );

    let mut chat_req = ChatRequest::new(vec![
        ChatMessage::system(system_prompt),
        ChatMessage::user(files_content),
    ]);

    println!("Thinking (Looping with context)...");
    
    loop {
        let response = ai_client.exec_chat("gemini-2.5-flash", chat_req.clone(), None).await?;
        
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
                    
                println!("Agent queried context: {}", query);
                
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
                chat_req = chat_req.append_message(ChatMessage::user(format!("Context Result:\n{}", context_result)));
            } else if content.contains("FINAL_REVIEW:") {
                let final_review = content.replace("FINAL_REVIEW:", "").trim().to_string();
                println!("\n=== Final Review Report ===\n");
                println!("{}", final_review);
                break;
            } else {
                // If the model didn't follow the exact format, just print it as the final output
                println!("\n=== Review Report ===\n");
                println!("{}", content);
                break;
            }
        } else {
            println!("No review output generated.");
            break;
        }
    }

    Ok(())
}
