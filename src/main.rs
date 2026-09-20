use clap::Parser;
use regex::Regex;
use std::path::PathBuf;
use std::process::Command;

/// A tool that measures the amount of lines added and deleted in a git codebase
#[derive(Parser, Debug)]
#[command(name = "cargo-churn")]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the git repository (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Regex pattern for files to ignore (can be specified multiple times)
    #[arg(short = 'i', long = "ignore", value_name = "PATTERN")]
    ignore: Vec<String>,
}

fn format_count(count: u64) -> String {
    if count >= 1_000_000 {
        format!("{:.0}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.0}k", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

fn main() {
    let args = Args::parse();

    // Compile ignore patterns
    let ignore_patterns: Vec<Regex> = args
        .ignore
        .iter()
        .filter_map(|pattern| match Regex::new(pattern) {
            Ok(re) => Some(re),
            Err(e) => {
                eprintln!("Warning: Invalid regex pattern '{}': {}", pattern, e);
                None
            }
        })
        .collect();

    // Run git log with --numstat to get additions/deletions per file
    let output = Command::new("git")
        .args([
            "-C",
            args.path.to_str().unwrap_or("."),
            "log",
            "--numstat",
            "--pretty=format:",
            "--all",
        ])
        .output()
        .expect("Failed to execute git command. Is git installed and is this a git repository?");

    if !output.status.success() {
        eprintln!(
            "Git command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut total_added: u64 = 0;
    let mut total_deleted: u64 = 0;

    for line in stdout.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Format: <added>\t<deleted>\t<filename>
        // For binary files, added/deleted will be "-"
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 3 {
            continue;
        }

        let added_str = parts[0];
        let deleted_str = parts[1];
        let filename = parts[2..].join("\t"); // Handle filenames with tabs (rare but possible)

        // Skip binary files (shown as "-")
        if added_str == "-" || deleted_str == "-" {
            continue;
        }

        // Check if file matches any ignore pattern
        if ignore_patterns.iter().any(|re| re.is_match(&filename)) {
            continue;
        }

        // Parse the numbers
        if let (Ok(added), Ok(deleted)) = (added_str.parse::<u64>(), deleted_str.parse::<u64>()) {
            total_added += added;
            total_deleted += deleted;
        }
    }

    println!(
        "+{} / -{}",
        format_count(total_added),
        format_count(total_deleted)
    );
}
