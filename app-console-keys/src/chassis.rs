use std::{
    collections::BTreeMap,
    io::{self, Write},
    sync::mpsc,
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::Paragraph,
    Frame, Terminal,
};

use crate::{
    cartridge::{Cartridge, CartridgeAction},
    fkey::FKey,
    widgets::status_bar::MbaStatus,
};
use std::collections::BTreeSet;

pub enum ChassisAction {
    None,
    Quit,
}

pub struct AppConsoleKeys {
    cartridges: BTreeMap<FKey, Box<dyn Cartridge>>,
    active: FKey,
    started: Instant,
    mba_status: MbaStatus,
    username: String,
    tenant: String,
}

impl AppConsoleKeys {
    pub fn new(username: impl Into<String>, tenant: impl Into<String>) -> Self {
        Self {
            cartridges: BTreeMap::new(),
            active: FKey::F4,
            started: Instant::now(),
            mba_status: MbaStatus::Inactive("not configured".into()),
            username: username.into(),
            tenant: tenant.into(),
        }
    }

    pub fn register(&mut self, cartridge: Box<dyn Cartridge>) {
        let fkey = cartridge.fkey();
        // Default to first registered cartridge if none set yet
        if self.cartridges.is_empty() {
            self.active = fkey;
        }
        self.cartridges.insert(fkey, cartridge);
    }

    fn installed(&self) -> BTreeSet<FKey> {
        self.cartridges.keys().copied().collect()
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // F-key tab strip
                Constraint::Fill(1),   // cartridge content
                Constraint::Length(1), // status bar
            ])
            .split(area);

        let installed = self.installed();
        crate::widgets::fkey_strip::render(frame, chunks[0], self.active, &installed);

        if let Some(c) = self.cartridges.get_mut(&self.active) {
            c.render(frame, chunks[1]);
        } else {
            frame.render_widget(
                Paragraph::new(format!("\n  {} — not installed", self.active.label())),
                chunks[1],
            );
        }

        let elapsed = self.started.elapsed().as_secs();
        crate::widgets::status_bar::render(
            frame,
            chunks[2],
            &self.username,
            &self.tenant,
            &self.mba_status,
            self.active,
            elapsed,
        );
    }

    pub fn handle_event(&mut self, event: &Event) -> ChassisAction {
        // F12 always routes to The Anchor (SYS-ADR-10) — unconditional
        if let Event::Key(key) = event {
            if key.code == KeyCode::F(12) {
                self.active = FKey::F12;
                return ChassisAction::None;
            }
        }

        // Delegate to active cartridge first; only handle globally if not consumed
        if let Some(c) = self.cartridges.get_mut(&self.active) {
            match c.handle_event(event) {
                CartridgeAction::Consumed => return ChassisAction::None,
                CartridgeAction::Quit => return ChassisAction::Quit,
                CartridgeAction::None => {}
            }
        }

        // Cartridge did not consume — apply chassis-level bindings
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('q')
                || (key.code == KeyCode::Char('c')
                    && key.modifiers.contains(KeyModifiers::CONTROL))
            {
                return ChassisAction::Quit;
            }
            if let Some(fkey) = FKey::from_keycode(key.code) {
                self.active = fkey;
                return ChassisAction::None;
            }
        }

        ChassisAction::None
    }

    pub fn set_mba_active(&mut self) {
        self.mba_status = MbaStatus::Active;
    }

    /// Run driven by raw SSH bytes; `terminal` writes to a TerminalHandle defined in os-console.
    pub fn run_with_bytes<W: Write + Send>(
        mut self,
        mut terminal: ratatui::Terminal<CrosstermBackend<W>>,
        rx: mpsc::Receiver<u8>,
    ) {
        let mut parser = crate::input_bytes::ByteParser::new();
        loop {
            if terminal.draw(|f| self.render(f)).is_err() {
                break;
            }
            match rx.recv_timeout(Duration::from_millis(16)) {
                Ok(byte) => {
                    if let Some(ev) = parser.push(byte) {
                        if let ChassisAction::Quit = self.handle_event(&ev) {
                            break;
                        }
                    }
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
    }

    /// Run in local crossterm PTY mode (default; no SSH required).
    pub fn run_local(mut self) -> Result<()> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let run_result = (|| -> Result<()> {
            loop {
                terminal.draw(|f| self.render(f))?;
                if event::poll(Duration::from_millis(16))? {
                    let ev = event::read()?;
                    if let ChassisAction::Quit = self.handle_event(&ev) {
                        break;
                    }
                }
            }
            Ok(())
        })();

        let _ = execute!(terminal.backend_mut(), LeaveAlternateScreen, cursor::Show);
        let _ = disable_raw_mode();

        run_result
    }
}
