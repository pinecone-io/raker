use crate::client::RakerClient;
use crate::config::load_config;
use crate::output;
use anyhow::Result;
use std::path::{Path, PathBuf};

fn get_all_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if dir.is_file() {
        files.push(dir.to_path_buf());
    } else if dir.is_dir() {
        let walker = ignore::WalkBuilder::new(dir).hidden(false).build();
        for result in walker {
            match result {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() {
                        files.push(path.to_path_buf());
                    }
                }
                Err(err) => eprintln!("Warning: {}", err),
            }
        }
    }
    Ok(files)
}

pub async fn upload(
    context_id: &str,
    file_path: &str,
    target_path: Option<&str>,
    json: bool,
) -> Result<()> {
    let cfg = load_config()?;
    let client = RakerClient::new(&cfg)?;

    let base_path = Path::new(file_path);
    if !base_path.exists() {
        anyhow::bail!("Path not found: {}", file_path);
    }

    let files = get_all_files(base_path)?;
    let mut results = Vec::new();

    for file in files {
        let abs_path = std::fs::canonicalize(&file).unwrap_or_else(|_| file.clone());
        let abs_parent = abs_path
            .parent()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default()
            .replace("\\", "/");

        let sub_path = if let Some(target) = target_path {
            let rel_path = file
                .strip_prefix(if base_path.is_file() {
                    base_path.parent().unwrap_or(Path::new(""))
                } else {
                    base_path
                })
                .unwrap_or(&file);

            let rel_parent = rel_path
                .parent()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_default()
                .replace("\\", "/");

            if target.is_empty() {
                if rel_parent.is_empty() {
                    Some("".to_string())
                } else {
                    Some(rel_parent)
                }
            } else {
                let clean_target = target.trim_end_matches('/');
                if rel_parent.is_empty() {
                    Some(clean_target.to_string())
                } else {
                    Some(format!("{}/{}", clean_target, rel_parent))
                }
            }
        } else {
            let clean_parent = abs_parent.trim_start_matches('/');
            if clean_parent.is_empty() {
                None
            } else {
                Some(clean_parent.to_string())
            }
        };

        let result = client
            .upload_file(context_id, &file.to_string_lossy(), sub_path.as_deref())
            .await?;

        if !json {
            println!("File {} uploaded successfully.", abs_path.display());
        }
        results.push(result);
    }

    if json {
        output::print_json(&results);
    } else if results.len() > 1 {
        println!("All {} files uploaded successfully.", results.len());
    }

    Ok(())
}
