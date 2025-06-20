use super::{ProgressStage, WorkerCommand, WorkerEvent};
use crate::core::types::{CanonicalPath, OutputFormat, PatternString, ProgressCount, TokenCount};
use crossbeam::channel::{Receiver, Sender};
use glob::Pattern;
use rayon::prelude::*;
use std::fmt::Write;
use std::fs;
use std::path::Path;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

/// Main worker thread function for output generation
pub fn run_worker(cmd_rx: &Receiver<WorkerCommand>, event_tx: &Sender<WorkerEvent>) {
    let cancelled = Arc::new(AtomicBool::new(false));

    while let Ok(command) = cmd_rx.recv() {
        match command {
            WorkerCommand::GenerateOutput {
                root_path,
                selected_files,
                format,
                include_tree,
                ignore_patterns,
            } => {
                cancelled.store(false, Ordering::Relaxed);
                generate_output(
                    &root_path,
                    &selected_files,
                    format,
                    include_tree,
                    &ignore_patterns,
                    event_tx,
                    &cancelled,
                );
            }
            WorkerCommand::Cancel => {
                cancelled.store(true, Ordering::Relaxed);
                let _ = event_tx.send(WorkerEvent::Cancelled);
            }
        }
    }
}

fn generate_output(
    root_path: &CanonicalPath,
    selected_files: &[CanonicalPath],
    format: OutputFormat,
    include_tree: bool,
    ignore_patterns: &PatternString,
    event_tx: &Sender<WorkerEvent>,
    cancelled: &Arc<AtomicBool>,
) {
    // Send initial progress
    let _ = event_tx.send(WorkerEvent::Progress {
        stage: ProgressStage::ScanningFiles,
        progress: ProgressCount::new(0, selected_files.len()),
    });

    // Read file contents in parallel
    let file_contents = read_files_parallel(selected_files, event_tx, cancelled);

    if cancelled.load(Ordering::Relaxed) {
        let _ = event_tx.send(WorkerEvent::Cancelled);
        return;
    }

    // Build output
    let _ = event_tx.send(WorkerEvent::Progress {
        stage: ProgressStage::BuildingOutput,
        progress: ProgressCount::new(0, 1),
    });

    // Generate directory tree with ignore patterns
    let tree_string = if include_tree {
        let patterns = ignore_patterns.split();
        generate_filtered_tree_string(root_path.as_path(), &patterns)
    } else {
        String::new()
    };

    let (output, failed_files) = build_output(format, root_path, &file_contents, &tree_string);

    if !failed_files.is_empty() && !cancelled.load(Ordering::Relaxed) {
        let error_msg = format!(
            "Warning: Failed to read {} file(s): {}",
            failed_files.len(),
            failed_files.join(", ")
        );
        let _ = event_tx.send(WorkerEvent::Error(error_msg));
    }

    // Calculate token count
    let token_count = TokenCount::from_chars(output.chars().count());

    let _ = event_tx.send(WorkerEvent::Progress {
        stage: ProgressStage::BuildingOutput,
        progress: ProgressCount::new(1, 1),
    });

    if !cancelled.load(Ordering::Relaxed) {
        let _ = event_tx.send(WorkerEvent::OutputReady {
            content: output,
            token_count,
        });
    }
}

/// Read files in parallel with progress reporting
fn read_files_parallel(
    selected_files: &[CanonicalPath],
    event_tx: &Sender<WorkerEvent>,
    cancelled: &Arc<AtomicBool>,
) -> Vec<(CanonicalPath, Result<String, String>)> {
    let processed = Arc::new(AtomicUsize::new(0));
    let total_files = selected_files.len();

    selected_files
        .par_iter()
        .map(|path| {
            if cancelled.load(Ordering::Relaxed) {
                return (path.clone(), Err("Cancelled".to_string()));
            }

            let result =
                fs::read_to_string(path.as_path()).map_err(|e| format!("Failed to read file: {e}"));

            let current = processed.fetch_add(1, Ordering::Relaxed) + 1;
            let _ = event_tx.send(WorkerEvent::Progress {
                stage: ProgressStage::ReadingFiles,
                progress: ProgressCount::new(current, total_files),
            });

            (path.clone(), result)
        })
        .collect()
}

/// Build the output string based on the selected format
fn build_output(
    format: OutputFormat,
    root_path: &CanonicalPath,
    file_contents: &[(CanonicalPath, Result<String, String>)],
    tree_string: &str,
) -> (String, Vec<String>) {
    let mut output = String::new();
    let mut failed_files = Vec::new();

    match format {
        OutputFormat::Xml => {
            build_xml_output(
                &mut output,
                root_path,
                file_contents,
                tree_string,
                &mut failed_files,
            );
        }
        OutputFormat::Markdown => {
            build_markdown_output(
                &mut output,
                root_path,
                file_contents,
                tree_string,
                &mut failed_files,
            );
        }
    }

    (output, failed_files)
}

/// Build XML format output
fn build_xml_output(
    output: &mut String,
    root_path: &CanonicalPath,
    file_contents: &[(CanonicalPath, Result<String, String>)],
    tree_string: &str,
    failed_files: &mut Vec<String>,
) {
    output.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<codebase>\n");

    // Add directory tree if enabled
    if !tree_string.is_empty() {
        output.push_str("  <directory_tree>\n");
        output.push_str("<![CDATA[\n");
        output.push_str(tree_string);
        output.push_str("]]>\n");
        output.push_str("  </directory_tree>\n\n");
    }

    // Add file contents
    output.push_str("  <files>\n");

    for (path, content_result) in file_contents {
        let relative_path = path
            .as_path()
            .strip_prefix(root_path.as_path())
            .unwrap_or(path.as_path());
        let path_str = relative_path.to_string_lossy();

        match content_result {
            Ok(content) => {
                let _ = writeln!(output, "    <file path=\"{path_str}\">");
                output.push_str("<![CDATA[\n");
                output.push_str(content);
                if !content.ends_with('\n') {
                    output.push('\n');
                }
                output.push_str("]]>\n");
                output.push_str("    </file>\n");
            }
            Err(e) => {
                failed_files.push(format!("{path_str}: {e}"));
            }
        }
    }

    output.push_str("  </files>\n</codebase>");
}

/// Build Markdown format output
fn build_markdown_output(
    output: &mut String,
    root_path: &CanonicalPath,
    file_contents: &[(CanonicalPath, Result<String, String>)],
    tree_string: &str,
    failed_files: &mut Vec<String>,
) {
    output.push_str("# Codebase Export\n\n");

    // Add directory tree if enabled
    if !tree_string.is_empty() {
        output.push_str("## Directory Structure\n\n```\n");
        output.push_str(tree_string);
        output.push_str("```\n\n");
    }

    // Add file contents
    output.push_str("## Files\n\n");

    for (path, content_result) in file_contents {
        let relative_path = path
            .as_path()
            .strip_prefix(root_path.as_path())
            .unwrap_or(path.as_path());
        let path_str = relative_path.to_string_lossy();

        match content_result {
            Ok(content) => {
                let _ = writeln!(output, "### {path_str}\n");

                let lang = get_language_from_extension(path.as_path());

                let _ = writeln!(output, "```{lang}");
                output.push_str(content);
                if !content.ends_with('\n') {
                    output.push('\n');
                }
                output.push_str("```\n\n");
            }
            Err(e) => {
                failed_files.push(format!("{path_str}: {e}"));
            }
        }
    }
}

/// Get the language identifier from a file extension
fn get_language_from_extension(path: &Path) -> &'static str {
    let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

    match extension {
        "rs" => "rust",
        "js" => "javascript",
        "ts" => "typescript",
        "py" => "python",
        "java" => "java",
        "c" | "h" => "c",
        "cpp" | "hpp" | "cc" | "cxx" => "cpp",
        "cs" => "csharp",
        "go" => "go",
        "rb" => "ruby",
        "php" => "php",
        "swift" => "swift",
        "kt" => "kotlin",
        "scala" => "scala",
        "r" => "r",
        "m" => "objective-c",
        "pl" => "perl",
        "lua" => "lua",
        "sh" | "bash" => "bash",
        "sql" => "sql",
        "html" | "htm" => "html",
        "css" => "css",
        "xml" => "xml",
        "json" => "json",
        "yaml" | "yml" => "yaml",
        "toml" => "toml",
        "md" => "markdown",
        _ => "",
    }
}

/// Generate a tree string with ignore patterns applied
fn generate_filtered_tree_string(root_path: &Path, ignore_patterns: &[String]) -> String {
    // Compile patterns
    let patterns: Vec<Pattern> = ignore_patterns
        .iter()
        .filter_map(|p| Pattern::new(p).ok())
        .collect();

    let mut output = String::new();
    generate_filtered_tree_recursive(root_path, &mut output, "", true, 0, &patterns);
    output
}

fn generate_filtered_tree_recursive(
    path: &Path,
    output: &mut String,
    prefix: &str,
    is_last: bool,
    depth: usize,
    patterns: &[Pattern],
) {
    const MAX_DEPTH: usize = 10;

    // Prevent infinite recursion
    if depth > MAX_DEPTH {
        return;
    }

    // Get the file/folder name
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| path.to_str().unwrap_or("?"));

    // Check if this entry should be ignored
    for pattern in patterns {
        if pattern.matches(name) {
            return;
        }
    }

    // Add the current node
    let connector = if is_last { "└── " } else { "├── " };
    let icon = if path.is_dir() { "📁" } else { "📄" };

    output.push_str(prefix);
    output.push_str(connector);
    output.push_str(icon);
    output.push(' ');
    output.push_str(name);
    output.push('\n');

    // Process directory children
    if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            let mut entries: Vec<_> = entries
                .filter_map(std::result::Result::ok)
                .map(|e| e.path())
                .collect();

            // Sort entries: directories first, then alphabetically
            entries.sort_by(|a, b| match (a.is_dir(), b.is_dir()) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.file_name().cmp(&b.file_name()),
            });

            // Filter out ignored entries
            #[allow(clippy::unnecessary_map_or)] // is_none_or is unstable
            let filtered_entries: Vec<_> = entries
                .into_iter()
                .filter(|entry| {
                    entry
                        .file_name()
                        .and_then(|n| n.to_str())
                        .map_or(true, |name| !patterns.iter().any(|p| p.matches(name)))
                })
                .collect();

            let new_prefix = format!("{}{}", prefix, if is_last { "    " } else { "│   " });
            let entry_count = filtered_entries.len();

            for (index, entry) in filtered_entries.iter().enumerate() {
                let is_last_child = index == entry_count - 1;
                generate_filtered_tree_recursive(
                    entry,
                    output,
                    &new_prefix,
                    is_last_child,
                    depth + 1,
                    patterns,
                );
            }
        }
    }
}
