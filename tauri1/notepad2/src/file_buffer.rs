// src/file_buffer.rs
use std::fs;
use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::path::PathBuf;

#[derive(Clone)]
pub struct FileBuffer {
    pub name: String,
    pub content: String,
    pub file_path: Option<String>,
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,
    pub last_edit_time: Instant,
}

impl FileBuffer {
    pub fn new(name: &str, content: String, path: Option<String>) -> Self {
        Self {
            name: name.to_string(),
            content,
            file_path: path,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            last_edit_time: Instant::now(),
        }
    }

    pub fn push_undo(&mut self) {
        if self.undo_stack.last().map_or(true, |last| last != &self.content) {
            self.undo_stack.push(self.content.clone());
        }
    }

    pub fn undo(&mut self) {
        if let Some(prev) = self.undo_stack.pop() {
            self.redo_stack.push(self.content.clone());
            self.content = prev;
        }
    }

    pub fn redo(&mut self) {
        if let Some(next) = self.redo_stack.pop() {
            self.undo_stack.push(self.content.clone());
            self.content = next;
        }
    }

    pub fn needs_autosave(&self) -> bool {
        self.last_edit_time.elapsed() > Duration::from_secs(10)
    }

    pub fn reset_edit_timer(&mut self) {
        self.last_edit_time = Instant::now();
    }

    pub fn save(&mut self) -> io::Result<()> {
        if let Some(path) = &self.file_path {
            fs::write(path, &self.content)?;
            self.mark_clean();
            Ok(())
        } else {
            // Could show a "Save As" dialog in future
            Err(io::Error::new(io::ErrorKind::Other, "No file path set"))
        }
    }

    pub fn mark_clean(&mut self) {
        self.last_edit_time = std::time::Instant::now();
    }
} 