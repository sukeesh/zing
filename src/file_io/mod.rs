//! File I/O module for Zing text editor.
//!
//! This module provides functionality for opening, saving, and printing files.

use anyhow::{Context, Result};
use rfd::FileDialog;
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::buffer::TextBuffer;

/// Opens a file dialog for selecting a file to open.
pub fn open_file_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Text Files", &["txt", "md", "rs", "toml", "json", "yaml", "yml"])
        .add_filter("All Files", &["*"])
        .set_title("Open File")
        .pick_file()
}

/// Opens a file dialog for saving a file.
pub fn save_file_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("Text Files", &["txt", "md", "rs", "toml", "json", "yaml", "yml"])
        .add_filter("All Files", &["*"])
        .set_title("Save File")
        .save_file()
}

/// Loads a file into a text buffer.
pub async fn load_file<P: AsRef<Path>>(path: P) -> Result<TextBuffer> {
    TextBuffer::from_file(path).await
}

/// Saves a text buffer to a file.
pub async fn save_buffer_to_file(buffer: &mut TextBuffer, path: Option<PathBuf>) -> Result<()> {
    match path {
        Some(path) => buffer.save_to(path).await,
        None => {
            if buffer.file_path.is_some() {
                buffer.save().await
            } else {
                Err(anyhow::anyhow!("No file path provided and buffer has no associated path"))
            }
        }
    }
}

/// Prints the content of a text buffer.
///
/// This implementation saves the buffer to a temporary file and opens it with the system's
/// default application, which will allow the user to print it.
pub fn print_buffer(buffer: &TextBuffer) -> Result<()> {
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;
    use tempfile::NamedTempFile;
    
    log::info!("Preparing buffer with {} characters for printing", buffer.len_chars());
    
    // Create a temporary file
    let mut temp_file = NamedTempFile::new()?;
    let temp_path = temp_file.path().to_path_buf();
    
    // Write the buffer content to the temporary file
    temp_file.write_all(buffer.content.to_string().as_bytes())?;
    
    // Flush to ensure all data is written
    temp_file.flush()?;
    
    // Open the file with the system's default application
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg(&temp_path)
            .spawn()
            .context("Failed to open file for printing")?;
    }
    
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(&["/C", "start", "", temp_path.to_str().unwrap()])
            .spawn()
            .context("Failed to open file for printing")?;
    }
    
    #[cfg(target_os = "linux")]
    {
        Command::new("xdg-open")
            .arg(&temp_path)
            .spawn()
            .context("Failed to open file for printing")?;
    }
    
    log::info!("File opened with system default application for printing");
    
    // Don't delete the temp file immediately, as it needs to be accessed by the application
    // The OS will clean it up when the application closes
    std::mem::forget(temp_file);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[tokio::test]
    async fn test_load_file() -> Result<()> {
        // Create a temporary file with some content
        let mut temp_file = NamedTempFile::new()?;
        let content = "Hello, world!\nThis is a test.";
        write!(temp_file, "{}", content)?;
        
        // Load the file into a buffer
        let buffer = load_file(temp_file.path()).await?;
        
        // Check that the buffer contains the expected content
        assert_eq!(buffer.content.to_string(), content);
        assert_eq!(buffer.file_path, Some(temp_file.path().to_path_buf()));
        assert!(!buffer.modified);
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_save_buffer() -> Result<()> {
        // Create a buffer with some content
        let mut buffer = TextBuffer::from_str("Hello, world!");
        
        // Create a temporary file to save to
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path().to_path_buf();
        
        // Save the buffer to the file
        save_buffer_to_file(&mut buffer, Some(path.clone())).await?;
        
        // Check that the file contains the expected content
        let content = fs::read_to_string(&path).await?;
        assert_eq!(content, "Hello, world!");
        
        // Check that the buffer's file path was updated
        assert_eq!(buffer.file_path, Some(path));
        assert!(!buffer.modified);
        
        Ok(())
    }
} 