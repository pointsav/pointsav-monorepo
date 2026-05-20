use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

use crate::fkey::FKey;

pub enum CartridgeAction {
    None,
    Quit,
}

pub trait Cartridge: Send {
    fn fkey(&self) -> FKey;
    fn title(&self) -> &str;
    fn is_installed(&self) -> bool {
        true
    }
    fn render(&mut self, frame: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: &Event) -> CartridgeAction;
}
