//! Text buffer module for Zing text editor.
//! 
//! This module provides an efficient text buffer implementation using the Ropey crate,
//! which is optimized for handling large text files and efficient editing operations.

use anyhow::{Context, Result};
use ropey::Rope;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;

/// Represents an edit operation that can be undone or redone.
#[derive(Debug, Clone)]
enum EditOperation {
    /// Insert text at a position
    Insert {
        position: usize,
        text: String,
    },
    /// Delete text from a range
    Delete {
        start: usize,
        end: usize,
        text: String,
    },
}

/// Represents a text buffer in the editor.
#[derive(Debug, Clone)]
pub struct TextBuffer {
    /// The text content stored as a rope data structure
    pub content: Rope,
    /// The file path associated with this buffer, if any
    pub file_path: Option<PathBuf>,
    /// Whether the buffer has unsaved changes
    pub modified: bool,
    /// History of edit operations for undo
    undo_stack: Vec<EditOperation>,
    /// History of edit operations for redo
    redo_stack: Vec<EditOperation>,
    /// Whether we're currently in an undo/redo operation
    in_undo_redo: bool,
}

impl TextBuffer {
    /// Creates a new empty text buffer.
    pub fn new() -> Self {
        Self {
            content: Rope::new(),
            file_path: None,
            modified: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            in_undo_redo: false,
        }
    }

    /// Creates a new text buffer from the given content.
    pub fn from_str(content: &str) -> Self {
        Self {
            content: Rope::from_str(content),
            file_path: None,
            modified: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            in_undo_redo: false,
        }
    }

    /// Loads a text buffer from a file.
    pub async fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read file: {}", path.display()))?;
        
        Ok(Self {
            content: Rope::from_str(&content),
            file_path: Some(path.to_path_buf()),
            modified: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            in_undo_redo: false,
        })
    }

    /// Saves the buffer content to the associated file.
    pub async fn save(&mut self) -> Result<()> {
        if let Some(path) = self.file_path.clone() {
            self.save_to(path).await?;
            self.modified = false;
            Ok(())
        } else {
            Err(anyhow::anyhow!("No file path associated with this buffer"))
        }
    }

    /// Saves the buffer content to a specific file path.
    pub async fn save_to<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        let content = self.content.to_string();
        
        fs::write(path, content)
            .await
            .with_context(|| format!("Failed to write to file: {}", path.display()))?;
        
        self.file_path = Some(path.to_path_buf());
        self.modified = false;
        Ok(())
    }

    /// Inserts text at the specified character position.
    pub fn insert(&mut self, char_idx: usize, text: &str) -> Result<()> {
        if char_idx <= self.content.len_chars() {
            // If not in an undo/redo operation, record this edit for undo
            if !self.in_undo_redo {
                self.undo_stack.push(EditOperation::Insert {
                    position: char_idx,
                    text: text.to_string(),
                });
                // Clear redo stack when a new edit is made
                self.redo_stack.clear();
            }
            
            self.content.insert(char_idx, text);
            self.modified = true;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Character index out of bounds"))
        }
    }

    /// Removes text in the specified character range.
    pub fn remove(&mut self, char_start: usize, char_end: usize) -> Result<()> {
        if char_start <= char_end && char_end <= self.content.len_chars() {
            // If not in an undo/redo operation, record this edit for undo
            if !self.in_undo_redo {
                let removed_text = self.content.slice(char_start..char_end).to_string();
                self.undo_stack.push(EditOperation::Delete {
                    start: char_start,
                    end: char_end,
                    text: removed_text,
                });
                // Clear redo stack when a new edit is made
                self.redo_stack.clear();
            }
            
            self.content.remove(char_start..char_end);
            self.modified = true;
            Ok(())
        } else {
            Err(anyhow::anyhow!("Character range out of bounds"))
        }
    }

    /// Performs an undo operation, reverting the last edit.
    pub fn undo(&mut self) -> Result<()> {
        if let Some(operation) = self.undo_stack.pop() {
            self.in_undo_redo = true;
            
            match operation {
                EditOperation::Insert { position, text } => {
                    // To undo an insert, we delete the inserted text
                    let end_pos = position + text.chars().count();
                    self.remove(position, end_pos)?;
                    
                    // Add to redo stack
                    self.redo_stack.push(EditOperation::Insert {
                        position,
                        text,
                    });
                },
                EditOperation::Delete { start, end: _, text } => {
                    // To undo a delete, we insert the deleted text
                    self.insert(start, &text)?;
                    
                    // Add to redo stack
                    self.redo_stack.push(EditOperation::Delete {
                        start,
                        end: start + text.chars().count(),
                        text,
                    });
                },
            }
            
            self.in_undo_redo = false;
            Ok(())
        } else {
            // Nothing to undo
            Ok(())
        }
    }

    /// Performs a redo operation, reapplying a previously undone edit.
    pub fn redo(&mut self) -> Result<()> {
        if let Some(operation) = self.redo_stack.pop() {
            self.in_undo_redo = true;
            
            match operation {
                EditOperation::Insert { position, text } => {
                    // To redo an insert, we insert the text again
                    self.insert(position, &text)?;
                    
                    // Add back to undo stack
                    self.undo_stack.push(EditOperation::Insert {
                        position,
                        text,
                    });
                },
                EditOperation::Delete { start, end, text } => {
                    // To redo a delete, we delete the text again
                    self.remove(start, end)?;
                    
                    // Add back to undo stack
                    self.undo_stack.push(EditOperation::Delete {
                        start,
                        end,
                        text,
                    });
                },
            }
            
            self.in_undo_redo = false;
            Ok(())
        } else {
            // Nothing to redo
            Ok(())
        }
    }

    /// Returns the total number of characters in the buffer.
    pub fn len_chars(&self) -> usize {
        self.content.len_chars()
    }

    /// Returns the total number of lines in the buffer.
    pub fn len_lines(&self) -> usize {
        self.content.len_lines()
    }

    /// Returns whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.content.len_chars() == 0
    }

    /// Gets a slice of the buffer as a string.
    pub fn slice(&self, char_start: usize, char_end: usize) -> Result<String> {
        if char_start <= char_end && char_end <= self.content.len_chars() {
            Ok(self.content.slice(char_start..char_end).to_string())
        } else {
            Err(anyhow::anyhow!("Character range out of bounds"))
        }
    }

    /// Gets a line from the buffer.
    pub fn line(&self, line_idx: usize) -> Result<String> {
        if line_idx < self.content.len_lines() {
            Ok(self.content.line(line_idx).to_string())
        } else {
            Err(anyhow::anyhow!("Line index out of bounds"))
        }
    }

    /// Converts a character index to a line and column.
    pub fn char_to_line_col(&self, char_idx: usize) -> Result<(usize, usize)> {
        if char_idx <= self.content.len_chars() {
            let line_idx = self.content.char_to_line(char_idx);
            let line_char_idx = self.content.line_to_char(line_idx);
            let col = char_idx - line_char_idx;
            Ok((line_idx, col))
        } else {
            Err(anyhow::anyhow!("Character index out of bounds"))
        }
    }

    /// Converts a line and column to a character index.
    pub fn line_col_to_char(&self, line: usize, col: usize) -> Result<usize> {
        if line < self.content.len_lines() {
            let line_char_idx = self.content.line_to_char(line);
            let line_len = self.content.line(line).len_chars();
            
            if col <= line_len {
                Ok(line_char_idx + col)
            } else {
                Err(anyhow::anyhow!("Column index out of bounds"))
            }
        } else {
            Err(anyhow::anyhow!("Line index out of bounds"))
        }
    }
    
    /// Updates the buffer content from a string and records it as a single edit operation.
    pub fn update_content(&mut self, new_content: &str) -> Result<()> {
        // Record the entire change as a single edit operation
        if !self.in_undo_redo {
            let old_content = self.content.to_string();
            
            // Only record if there's an actual change
            if old_content != new_content {
                // For simplicity, we'll treat this as a delete-all + insert-all operation
                self.undo_stack.push(EditOperation::Delete {
                    start: 0,
                    end: old_content.chars().count(),
                    text: old_content,
                });
                
                // Clear redo stack when a new edit is made
                self.redo_stack.clear();
            }
        }
        
        // Update the content
        self.content = Rope::from_str(new_content);
        self.modified = true;
        
        Ok(())
    }
}

impl Default for TextBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_buffer() {
        let buffer = TextBuffer::new();
        assert!(buffer.is_empty());
        assert_eq!(buffer.len_chars(), 0);
        assert_eq!(buffer.len_lines(), 1); // Empty buffer has one line
    }

    #[test]
    fn test_from_str() {
        let text = "Hello, world!\nThis is a test.";
        let buffer = TextBuffer::from_str(text);
        assert_eq!(buffer.len_chars(), text.len());
        assert_eq!(buffer.len_lines(), 2);
    }

    #[test]
    fn test_insert_remove() {
        let mut buffer = TextBuffer::new();
        
        // Insert text
        buffer.insert(0, "Hello").unwrap();
        assert_eq!(buffer.content.to_string(), "Hello");
        
        // Insert more text
        buffer.insert(5, ", world!").unwrap();
        assert_eq!(buffer.content.to_string(), "Hello, world!");
        
        // Remove text
        buffer.remove(5, 13).unwrap();
        assert_eq!(buffer.content.to_string(), "Hello!");
    }
    
    #[test]
    fn test_undo_redo() {
        let mut buffer = TextBuffer::new();
        
        // Insert text
        buffer.insert(0, "Hello").unwrap();
        assert_eq!(buffer.content.to_string(), "Hello");
        
        // Insert more text
        buffer.insert(5, ", world!").unwrap();
        assert_eq!(buffer.content.to_string(), "Hello, world!");
        
        // Undo the second insert
        buffer.undo().unwrap();
        assert_eq!(buffer.content.to_string(), "Hello");
        
        // Redo the second insert
        buffer.redo().unwrap();
        assert_eq!(buffer.content.to_string(), "Hello, world!");
        
        // Undo both inserts
        buffer.undo().unwrap();
        buffer.undo().unwrap();
        assert_eq!(buffer.content.to_string(), "");
    }

    #[test]
    fn test_line_operations() {
        let text = "Line 1\nLine 2\nLine 3";
        let buffer = TextBuffer::from_str(text);
        
        assert_eq!(buffer.line(0).unwrap(), "Line 1");
        assert_eq!(buffer.line(1).unwrap(), "Line 2");
        assert_eq!(buffer.line(2).unwrap(), "Line 3");
        
        assert_eq!(buffer.char_to_line_col(0).unwrap(), (0, 0));
        assert_eq!(buffer.char_to_line_col(6).unwrap(), (0, 6));
        assert_eq!(buffer.char_to_line_col(7).unwrap(), (1, 0));
        
        assert_eq!(buffer.line_col_to_char(0, 0).unwrap(), 0);
        assert_eq!(buffer.line_col_to_char(0, 6).unwrap(), 6);
        assert_eq!(buffer.line_col_to_char(1, 0).unwrap(), 7);
    }
} 