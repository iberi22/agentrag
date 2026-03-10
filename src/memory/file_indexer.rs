//! File Indexer - Indexa archivos markdown para memoria
//!
//! Lee archivos markdown, genera chunks y prepara sincronización incremental.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;
use tracing::{debug, info, warn};

use crate::memory::qmd_memory::QmdMemory;

#[derive(Debug, Clone)]
pub struct FileIndexerConfig {
    pub root_path: PathBuf,
    pub include_patterns: Vec<String>,
    pub exclude_patterns: Vec<String>,
    pub chunk_size: usize,
    pub chunk_overlap: usize,
}

impl Default for FileIndexerConfig {
    fn default() -> Self {
        Self {
            root_path: PathBuf::from("."),
            include_patterns: vec!["*.md".to_string()],
            exclude_patterns: vec![
                ".git/**".to_string(),
                "node_modules/**".to_string(),
                "__pycache__/**".to_string(),
            ],
            chunk_size: 400,
            chunk_overlap: 80,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexedFile {
    pub path: String,
    pub content: String,
    pub chunks: Vec<FileChunk>,
    pub last_modified: String,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChunk {
    pub index: usize,
    pub content: String,
    pub start_line: usize,
    pub end_line: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexResult {
    pub total_files: usize,
    pub total_chunks: usize,
    pub errors: Vec<String>,
    pub indexed_paths: Vec<String>,
}

pub struct FileIndexer {
    config: FileIndexerConfig,
}

impl FileIndexer {
    pub fn new(config: FileIndexerConfig) -> Self {
        Self { config }
    }

    pub async fn index_all(&self) -> Result<IndexResult> {
        info!("📂 Starting file indexing: {:?}", self.config.root_path);

        let mut result = IndexResult {
            total_files: 0,
            total_chunks: 0,
            errors: vec![],
            indexed_paths: vec![],
        };

        let mut stack = vec![self.config.root_path.clone()];

        while let Some(path) = stack.pop() {
            if path.is_file() {
                match self.index_file(&path).await {
                    Ok(file) => {
                        result.total_files += 1;
                        result.total_chunks += file.chunks.len();
                        result.indexed_paths.push(file.path.clone());
                        debug!("Indexed: {}", file.path);
                    }
                    Err(error) => {
                        warn!("Error indexing {:?}: {}", path, error);
                        result.errors.push(format!("{:?}: {}", path, error));
                    }
                }
                continue;
            }

            if let Ok(mut entries) = fs::read_dir(&path).await {
                while let Ok(Some(entry)) = entries.next_entry().await {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        if !self.should_exclude(&entry_path) {
                            stack.push(entry_path);
                        }
                        continue;
                    }

                    if self.should_index(&entry_path) {
                        stack.push(entry_path);
                    }
                }
            }
        }

        info!(
            "✅ Indexing complete: {} files, {} chunks",
            result.total_files, result.total_chunks
        );

        Ok(result)
    }

    pub async fn index_file(&self, path: &Path) -> Result<IndexedFile> {
        let content = fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read file: {:?}", path))?;

        let metadata = fs::metadata(path).await?;
        let last_modified = metadata
            .modified()
            .map(|time| {
                let datetime: chrono::DateTime<chrono::Utc> = time.into();
                datetime.to_rfc3339()
            })
            .unwrap_or_else(|_| "unknown".to_string());

        let chunks = self.generate_chunks(&content);

        Ok(IndexedFile {
            path: path.to_string_lossy().to_string(),
            content,
            chunks,
            last_modified,
            size: metadata.len() as usize,
        })
    }

    fn generate_chunks(&self, content: &str) -> Vec<FileChunk> {
        let mut chunks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();

        let mut current_chunk = String::new();
        let mut current_tokens = 0usize;
        let mut start_line = 0usize;

        for (index, line) in lines.iter().enumerate() {
            let estimated_tokens = line.split_whitespace().count().max(1);
            current_chunk.push_str(line);
            current_chunk.push('\n');
            current_tokens += estimated_tokens;

            if current_tokens >= self.config.chunk_size || (line.is_empty() && current_tokens > 0) {
                chunks.push(FileChunk {
                    index: chunks.len(),
                    content: current_chunk.clone(),
                    start_line,
                    end_line: index,
                });

                let overlap_lines = self
                    .config
                    .chunk_overlap
                    .min(index.saturating_sub(start_line));
                start_line = index.saturating_sub(overlap_lines) + 1;
                current_chunk.clear();
                current_tokens = 0;
            }
        }

        if !current_chunk.trim().is_empty() {
            chunks.push(FileChunk {
                index: chunks.len(),
                content: current_chunk,
                start_line,
                end_line: lines.len(),
            });
        }

        chunks
    }

    fn should_exclude(&self, path: &Path) -> bool {
        let path_str = path.to_string_lossy();
        self.config.exclude_patterns.iter().any(|pattern| {
            if pattern.contains("**") {
                let base = pattern.trim_start_matches("**/").trim_end_matches("/**");
                path_str.contains(base)
            } else {
                path_str.contains(pattern)
            }
        })
    }

    fn should_index(&self, path: &Path) -> bool {
        if self.should_exclude(path) {
            return false;
        }

        let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        self.config.include_patterns.iter().any(|pattern| {
            (pattern == "*.md" && extension.eq_ignore_ascii_case("md"))
                || path.file_name().and_then(|name| name.to_str()).is_some_and(|name| name == pattern)
        })
    }

    pub async fn sync_incremental(&self, _memory: &QmdMemory) -> Result<IndexResult> {
        info!("🔄 Starting incremental sync");
        self.index_all().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_generation() {
        let indexer = FileIndexer::new(FileIndexerConfig::default());
        let content = "Line 1\nLine 2\n\nLine 4\nLine 5\n";
        let chunks = indexer.generate_chunks(content);

        assert!(!chunks.is_empty());
    }

    #[test]
    fn test_exclude_patterns() {
        let indexer = FileIndexer::new(FileIndexerConfig::default());

        assert!(indexer.should_exclude(Path::new("/foo/node_modules/bar")));
        assert!(indexer.should_exclude(Path::new("/foo/.git/config")));
        assert!(!indexer.should_exclude(Path::new("/foo/src/main.rs")));
    }
}
