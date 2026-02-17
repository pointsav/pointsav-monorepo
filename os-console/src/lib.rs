#![no_std]

//! # os-console (Tier 4)
//! The Graphical Interface Engine for the PointSav Monorepo.
//! 
//! ## Purpose
//! Provides a high-fidelity Text User Interface (TUI) without window managers.

/// Standard Result type for Console operations
pub type ConsoleResult<T> = Result<T, ConsoleError>;

#[derive(Debug)]
pub enum ConsoleError {
    BufferOverflow,
    RenderFailure,
    InputRejected,
}

/// The primary interface for drawing to the terminal.
pub trait ConsoleEngine {
    /// Initialize the video buffer.
    fn init(&mut self) -> ConsoleResult<()>;

    /// Clear the screen with a specific color code.
    fn clear_screen(&mut self, color: u8) -> ConsoleResult<()>;

    /// Write a string to the buffer at (x, y).
    fn write_text(&mut self, x: usize, y: usize, text: &str) -> ConsoleResult<()>;
}

pub struct TerminalMode;

impl TerminalMode {
    pub fn new() -> Self {
        TerminalMode
    }
}
