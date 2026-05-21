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
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
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

pub struct PairingInfo {
    pub fingerprint: String,
    pub host: String,
    pub port: u16,
}

pub struct AppConsoleKeys {
    cartridges: BTreeMap<FKey, Box<dyn Cartridge>>,
    active: FKey,
    previous: FKey,
    started: Instant,
    mba_status: MbaStatus,
    username: String,
    tenant: String,
    pairing_info: Option<PairingInfo>,
}

impl AppConsoleKeys {
    pub fn new(username: impl Into<String>, tenant: impl Into<String>) -> Self {
        Self {
            cartridges: BTreeMap::new(),
            active: FKey::F4,
            previous: FKey::F4,
            started: Instant::now(),
            mba_status: MbaStatus::Inactive("not configured".into()),
            username: username.into(),
            tenant: tenant.into(),
            pairing_info: None,
        }
    }

    pub fn set_pairing_info(&mut self, info: PairingInfo) {
        self.pairing_info = Some(info);
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

        // Pairing screen: shown when MBA is INACTIVE and we have key info
        let mba_inactive = matches!(self.mba_status, MbaStatus::Inactive(_));
        if mba_inactive {
            if let Some(info) = &self.pairing_info {
                Self::render_pairing_screen(
                    frame,
                    chunks[1],
                    &self.username,
                    &self.tenant,
                    &info.fingerprint,
                    &info.host,
                    info.port,
                );
            } else {
                frame.render_widget(
                    Paragraph::new("\n  MBA LINK INACTIVE — configure ~/.config/os-console/config.toml"),
                    chunks[1],
                );
            }
        } else if let Some(c) = self.cartridges.get_mut(&self.active) {
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
        // F12 always routes to The Anchor (SYS-ADR-10) — unconditional, stores previous
        if let Event::Key(key) = event {
            if key.code == KeyCode::F(12) {
                if self.active != FKey::F12 {
                    self.previous = self.active;
                }
                self.active = FKey::F12;
                return ChassisAction::None;
            }
        }

        // Delegate to active cartridge first; only handle globally if not consumed
        if let Some(c) = self.cartridges.get_mut(&self.active) {
            match c.handle_event(event) {
                CartridgeAction::Consumed => return ChassisAction::None,
                CartridgeAction::Quit     => return ChassisAction::Quit,
                CartridgeAction::GoBack   => {
                    self.active = self.previous;
                    return ChassisAction::None;
                }
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
                if self.active != fkey {
                    self.previous = self.active;
                }
                self.active = fkey;
                return ChassisAction::None;
            }
        }

        ChassisAction::None
    }

    pub fn set_mba_active(&mut self) {
        self.mba_status = MbaStatus::Active;
    }

    fn render_pairing_screen(
        frame: &mut Frame,
        area: ratatui::layout::Rect,
        username: &str,
        tenant: &str,
        fingerprint: &str,
        host: &str,
        port: u16,
    ) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .title(" MBA Pairing Required — Pairing as Permission ");
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                "  This machine is not yet paired with os-totebox.",
                Style::default().fg(Color::White),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Your fingerprint:  ", Style::default().fg(Color::DarkGray)),
                Span::styled(fingerprint.to_string(), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "  Have an operator run on the os-totebox machine:",
                Style::default().fg(Color::White),
            )),
            Line::from(""),
            Line::from(Span::styled(
                format!("    proofctl user add {} \\", username),
                Style::default().fg(Color::LightGreen),
            )),
            Line::from(Span::styled(
                format!("      --tenant {} \\", tenant),
                Style::default().fg(Color::LightGreen),
            )),
            Line::from(Span::styled(
                "      --key-file <path/to/your/id_ed25519.pub>",
                Style::default().fg(Color::LightGreen),
            )),
            Line::from(""),
            Line::from(vec![
                Span::styled("  Connecting to:  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{}:{}", host, port),
                    Style::default().fg(Color::Yellow),
                ),
            ]),
            Line::from(""),
            Line::from(Span::styled(
                "  After the operator has registered your key, restart os-console.",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "  [Q / Ctrl-C: quit]",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        frame.render_widget(Paragraph::new(lines), inner);
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
