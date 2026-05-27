use crossterm::event::Event;
use ratatui::{layout::Rect, Frame};

use crate::fkey::FKey;

pub enum CartridgeAction {
    None,
    Consumed,
    Quit,
    GoBack,
}

pub trait Cartridge: Send {
    fn fkey(&self) -> FKey;
    fn title(&self) -> &str;
    fn is_installed(&self) -> bool {
        true
    }
    /// Called every frame before render; drain background channels into local state.
    fn tick(&mut self) {}
    /// Non-zero if this cartridge has a badge count (e.g. pending items).
    fn pending_badge(&self) -> u16 {
        0
    }
    fn render(&mut self, frame: &mut Frame, area: Rect);
    fn handle_event(&mut self, event: &Event) -> CartridgeAction;
}
