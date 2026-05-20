use app_console_keys::{Cartridge, CartridgeAction, FKey};
use crossterm::event::Event;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub struct ContentCartridge;

impl ContentCartridge {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ContentCartridge {
    fn default() -> Self {
        Self::new()
    }
}

impl Cartridge for ContentCartridge {
    fn fkey(&self) -> FKey {
        FKey::F4
    }

    fn title(&self) -> &str {
        "Content"
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" F4: Content — Proofreader + Draft ");
        let para = Paragraph::new(concat!(
            "\n  Phase 1 chassis active.\n\n",
            "  Proofread and draft workflows: Phase 3.\n\n",
            "  Press  q  or  Ctrl-C  to quit.\n",
            "  Press  F1-F12  to switch cartridges.",
        ))
        .block(block);
        frame.render_widget(para, area);
    }

    fn handle_event(&mut self, _event: &Event) -> CartridgeAction {
        CartridgeAction::None
    }
}
