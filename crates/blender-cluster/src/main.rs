use anyhow::Result;
use clap::Parser;
use colored::*;
use serde_json::Value;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

use blender_cluster::ast::try_ast_fast_path;
use blender_cluster::cluster::ClusterClient;
use blender_cluster::verifier::CrateVerifier;

#[derive(Parser, Debug)]
#[command(name = "blender-cluster", about = "Blender Rust Distributed Cluster Transpiler")]
struct Cli {
    #[arg(short, long, default_value = "makesdna")]
    subsystem: String,

    #[arg(short, long, default_value_t = 10)]
    limit: usize,

    #[arg(long)]
    retry_failed: bool,

    #[arg(long)]
    fast_only: bool,

    #[arg(short, long, default_value = "qwen2.5-coder:0.5b")]
    model: String,
}

fn route_file(rel_str: &str) -> &'static str {
    if rel_str.contains("bmesh") {
        "blender-bmesh"
    } else if rel_str.contains("makesdna") || rel_str.contains("dna") {
        "blender-dna"
    } else if rel_str.contains("io") {
        "blender-io"
    } else if rel_str.contains("math") || rel_str.contains("vec") {
        "blender-math"
    } else if rel_str.contains("mem") || rel_str.contains("alloc") {
        "blender-mem"
    } else {
        "blender-io"
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::parse();
    let root_dir = PathBuf::from(std::env::current_dir()?);
    let progress_file = root_dir.join("PORTING_PROGRESS.json");

    println!("{}", "\n======================================================================".cyan().bold());
    println!("{}", " ⚡ blender-cluster: Native Rust Distributed Transpile Engine ".bold());
    println!("{}", "======================================================================\n".cyan().bold());

    let client = Arc::new(ClusterClient::new());
    let verifier = Arc::new(CrateVerifier::new(root_dir.clone()));

    // 1. Health check
    let health = client.check_health().await;
    println!("{}", "[*] ZimaBoard Cluster Health:".bold());
    for (name, ok) in health {
        if ok {
            println!("  {} {} is online", "✓".green().bold(), name);
        } else {
            println!("  {} {} is offline", "✗".red().bold(), name);
        }
    }
    println!();

    if !progress_file.exists() {
        eprintln!("{}", "[!] PORTING_PROGRESS.json not found.".red());
        return Ok(());
    }

    let progress_content = std::fs::read_to_string(&progress_file)?;
    let mut progress: Value = serde_json::from_str(&progress_content)?;

    let mut target_files: Vec<PathBuf> = Vec::new();

    if args.retry_failed {
        if let Some(failed_obj) = progress.get("failed").and_then(|v| v.as_object()) {
            for (rel_path, _) in failed_obj {
                if args.subsystem == "all" || rel_path.contains(&args.subsystem) {
                    let full_path = root_dir.join(rel_path);
                    if full_path.exists() {
                        target_files.push(full_path);
                    }
                }
            }
        }
    } else {
        let subsys_dir = root_dir.join("upstream").join("source").join("blender").join(&args.subsystem);
        if subsys_dir.exists() {
            for entry in walkdir::WalkDir::new(subsys_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    let is_target_ext = if args.fast_only || args.subsystem == "makesdna" {
                        matches!(ext, "h" | "hh")
                    } else {
                        matches!(ext, "h" | "hh" | "c" | "cc")
                    };
                    if is_target_ext {
                        if let Ok(rel) = path.strip_prefix(&root_dir) {
                            let rel_str = rel.to_str().unwrap_or_default();
                            let completed = progress.get("completed").and_then(|c| c.get(rel_str)).is_some();
                            let skipped = progress.get("skipped").and_then(|s| s.get(rel_str)).is_some();
                            if !completed && !skipped {
                                target_files.push(path.to_path_buf());
                            }
                        }
                    }
                }
            }
        }
    }

    target_files.sort();
    target_files.truncate(args.limit);

    println!(
        "[*] Active Queue: {} files to process (Subsystem: {})\n",
        target_files.len().to_string().cyan().bold(),
        args.subsystem.yellow().bold()
    );

    let mut success_count = 0;
    let mut fast_path_count = 0;
    let mut neural_count = 0;

    for (idx, file_path) in target_files.iter().enumerate() {
        let rel_str = file_path.strip_prefix(&root_dir)?.to_str().unwrap_or_default();
        let stem = file_path.file_stem().and_then(|s| s.to_str()).unwrap_or("module").replace('.', "_");
        let dest_crate = route_file(rel_str);

        println!(
            "[{}/{}] Processing {} → {}",
            idx + 1,
            target_files.len(),
            file_path.file_name().unwrap().to_str().unwrap().cyan().bold(),
            dest_crate.yellow()
        );

        let content = std::fs::read_to_string(file_path).unwrap_or_default();

        // Pass 1: Zero-Token AST Fast-Path (<5ms)
        let t0 = Instant::now();
        if let Some(ast_rust) = try_ast_fast_path(&content) {
            let dur = t0.elapsed();
            println!(
                "  {} Zero-token AST Fast-Path matched in {:.2}ms",
                "⚡".green().bold(),
                dur.as_secs_f64() * 1000.0
            );

            if let Ok(true) = verifier.write_and_verify(dest_crate, &stem, &ast_rust, rel_str) {
                println!("  {} Committed via AST Fast-Path!", "✓".green().bold());
                progress["completed"][rel_str] = serde_json::json!({
                    "crate": dest_crate,
                    "module": format!("{}.rs", stem),
                    "mode": "ast_fast_path"
                });
                if let Some(failed_obj) = progress.get_mut("failed").and_then(|v| v.as_object_mut()) {
                    failed_obj.remove(rel_str);
                }
                success_count += 1;
                fast_path_count += 1;
                continue;
            } else {
                let _ = std::fs::write("/tmp/ast_failed.rs", &ast_rust);
                println!("  {} AST Fast-Path verification failed. Falling back to cluster.", "!".yellow());
            }
        }

        if args.fast_only {
            println!("  {} Fast-only mode enabled; skipping neural fallback.", "-".dimmed());
            continue;
        }

        // Pass 2: Neural Transpile via ZimaBoard Cluster
        let node = &client.nodes[idx % client.nodes.len()];
        println!("  {} Querying {} with {}...", "📡".blue(), node.name.bold(), args.model);

        let prompt_sample: String = content.lines().take(60).collect::<Vec<&str>>().join("\n");
        let system_prompt = format!(
            "You are an expert Rust systems engineer. Convert the following C/C++ Blender definitions into 100% idiomatic, Safe Rust for crate '{}'. Return ONLY valid Rust inside a ```rust block.\n\n{}",
            dest_crate, prompt_sample
        );

        match client.query_node(node, &system_prompt, &args.model, 512).await {
            Ok((rust_code, elapsed, tok_s)) => {
                println!(
                    "  {} Received {} lines in {:.1}s ({:.2} tok/s)",
                    "✓".green(),
                    rust_code.lines().count(),
                    elapsed,
                    tok_s
                );

                if !rust_code.is_empty() {
                    match verifier.write_and_verify(dest_crate, &stem, &rust_code, rel_str) {
                        Ok(true) => {
                            println!("  {} Clean compilation & git commit!", "✓".green().bold());
                            progress["completed"][rel_str] = serde_json::json!({
                                "crate": dest_crate,
                                "module": format!("{}.rs", stem),
                                "mode": "zima_cluster"
                            });
                            if let Some(failed_obj) = progress.get_mut("failed").and_then(|v| v.as_object_mut()) {
                                failed_obj.remove(rel_str);
                            }
                            success_count += 1;
                            neural_count += 1;
                        }
                        Ok(false) => {
                            println!("  {} Compiler check failed (cleanly rolled back).", "✗".red());
                        }
                        Err(e) => {
                            println!("  {} Verification error: {}", "!".red(), e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("  {} Cluster query failed: {}", "!".red(), e);
            }
        }
    }

    std::fs::write(&progress_file, serde_json::to_string_pretty(&progress)?)?;

    println!("\n{}", "======================================================================".cyan().bold());
    println!(
        "{} {} files converted ({} via AST Fast-Path, {} via Zima Neural Cluster)",
        "Summary:".bold(),
        success_count.to_string().green().bold(),
        fast_path_count.to_string().cyan().bold(),
        neural_count.to_string().magenta().bold()
    );
    println!("{}", "======================================================================\n".cyan().bold());

    Ok(())
}
