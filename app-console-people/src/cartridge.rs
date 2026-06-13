use std::sync::mpsc;
use std::time::Duration;

use app_console_keys::cartridge::{Cartridge, CartridgeAction};
use app_console_keys::fkey::FKey;
use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};
use serde::Deserialize;

// ── Contact model ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Deserialize)]
pub struct Contact {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub linkedin_url: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ContactsResponse {
    contacts: Vec<Contact>,
}

// ── State machine ─────────────────────────────────────────────────────────────

enum PeopleState {
    Loading,
    List,
    Detail,
    Error(String),
}

enum BgMsg {
    Contacts(Vec<Contact>),
    Err(String),
}

// ── PeopleCartridge ───────────────────────────────────────────────────────────

pub struct PeopleCartridge {
    state: PeopleState,
    contacts: Vec<Contact>,
    list_state: ListState,
    plain: bool,
    truecolor: bool,
    bg_rx: mpsc::Receiver<BgMsg>,
}

impl PeopleCartridge {
    pub fn new(endpoint: impl Into<String>, plain: bool) -> Self {
        let endpoint = endpoint.into();
        let (bg_tx, bg_rx) = mpsc::channel::<BgMsg>();

        std::thread::spawn(move || {
            fetch_contacts(&endpoint, &bg_tx);
        });

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            state: PeopleState::Loading,
            contacts: Vec::new(),
            list_state,
            plain,
            truecolor: false,
            bg_rx,
        }
    }

    fn accent_color(&self) -> Color {
        if self.truecolor {
            Color::Rgb(32, 178, 170)
        } else {
            Color::Cyan
        }
    }

    fn selection_bg(&self) -> Color {
        if self.truecolor {
            Color::Rgb(0, 95, 135)
        } else {
            Color::DarkGray
        }
    }

    fn move_up(&mut self) {
        if self.contacts.is_empty() {
            return;
        }
        let i = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some(if i == 0 {
            self.contacts.len() - 1
        } else {
            i - 1
        }));
    }

    fn move_down(&mut self) {
        if self.contacts.is_empty() {
            return;
        }
        let i = self.list_state.selected().unwrap_or(0);
        self.list_state.select(Some((i + 1) % self.contacts.len()));
    }

    fn selected_contact(&self) -> Option<&Contact> {
        self.list_state
            .selected()
            .and_then(|i| self.contacts.get(i))
    }

    fn render_list(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" F2: People — Directory ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.accent_color()));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(1)])
            .split(inner);

        if self.contacts.is_empty() {
            frame.render_widget(
                Paragraph::new("  No contacts.").style(Style::default().fg(Color::DarkGray)),
                chunks[0],
            );
        } else {
            let items: Vec<ListItem> = self
                .contacts
                .iter()
                .map(|c| {
                    let tz = c.timezone.as_deref().unwrap_or("—");
                    let line = Line::from(vec![
                        Span::styled(
                            format!("  {:<32}", truncate(&c.name, 31)),
                            Style::default().fg(Color::White),
                        ),
                        Span::styled(
                            format!("  {}", truncate(tz, 20)),
                            Style::default().fg(Color::DarkGray),
                        ),
                    ]);
                    ListItem::new(line)
                })
                .collect();

            let list = List::new(items)
                .highlight_style(if self.plain {
                    Style::default().add_modifier(Modifier::REVERSED)
                } else {
                    Style::default()
                        .bg(self.selection_bg())
                        .add_modifier(Modifier::BOLD)
                })
                .highlight_symbol("> ");
            frame.render_stateful_widget(list, chunks[0], &mut self.list_state);
        }

        let hint_text = " j/k=navigate  Enter=detail  Esc=back";
        frame.render_widget(
            Paragraph::new(hint_text).style(Style::default().fg(Color::DarkGray)),
            chunks[1],
        );
    }

    fn render_detail(&self, frame: &mut Frame, area: Rect) {
        let Some(c) = self.selected_contact() else {
            return;
        };
        let block = Block::default()
            .title(format!(" {} ", c.name))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.accent_color()));
        let inner = block.inner(area);
        frame.render_widget(block, area);

        let mut lines = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  Name:      ", Style::default().fg(Color::DarkGray)),
                Span::styled(c.name.clone(), Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("  ID:        ", Style::default().fg(Color::DarkGray)),
                Span::styled(c.id.clone(), Style::default().fg(Color::DarkGray)),
            ]),
            Line::from(vec![
                Span::styled("  Timezone:  ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    c.timezone.clone().unwrap_or_else(|| "—".into()),
                    Style::default().fg(Color::White),
                ),
            ]),
        ];
        if let Some(url) = &c.linkedin_url {
            lines.push(Line::from(vec![
                Span::styled("  LinkedIn:  ", Style::default().fg(Color::DarkGray)),
                Span::styled(url.clone(), Style::default().fg(self.accent_color())),
            ]));
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  [Esc: back to list]",
            Style::default().fg(Color::DarkGray),
        )));

        frame.render_widget(Paragraph::new(lines).wrap(Wrap { trim: false }), inner);
    }

    fn render_loading(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().title(" F2: People ").borders(Borders::ALL);
        let inner = block.inner(area);
        frame.render_widget(block, area);
        frame.render_widget(
            Paragraph::new("  Loading contacts…").style(Style::default().fg(Color::DarkGray)),
            inner,
        );
    }

    fn render_error_state(&self, frame: &mut Frame, area: Rect, msg: &str) {
        let block = Block::default()
            .title(" F2: People — Unavailable ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray));
        let inner = block.inner(area);
        frame.render_widget(block, area);
        frame.render_widget(
            Paragraph::new(vec![
                Line::from(""),
                Line::from(Span::styled(
                    format!(
                        "  {} service-people unavailable",
                        if self.plain { "[!]" } else { "⚠" }
                    ),
                    Style::default().fg(Color::Yellow),
                )),
                Line::from(""),
                Line::from(Span::styled(
                    format!("  {}", msg),
                    Style::default().fg(Color::DarkGray),
                )),
            ]),
            inner,
        );
    }
}

impl Cartridge for PeopleCartridge {
    fn fkey(&self) -> FKey {
        FKey::F2
    }

    fn title(&self) -> &str {
        "People"
    }

    fn set_graphics_caps(
        &mut self,
        _kitty: bool,
        _sixel: bool,
        _font_size: (u16, u16),
        truecolor: bool,
    ) {
        self.truecolor = truecolor;
    }

    fn tick(&mut self) {
        while let Ok(msg) = self.bg_rx.try_recv() {
            match msg {
                BgMsg::Contacts(contacts) => {
                    self.contacts = contacts;
                    if !self.contacts.is_empty() && self.list_state.selected().is_none() {
                        self.list_state.select(Some(0));
                    }
                    self.state = PeopleState::List;
                }
                BgMsg::Err(e) => {
                    self.state = PeopleState::Error(e);
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        // render_list needs &mut self (stateful widget); separate it before the shared borrow.
        if matches!(self.state, PeopleState::List) {
            self.render_list(frame, area);
            return;
        }
        match &self.state {
            PeopleState::Loading => self.render_loading(frame, area),
            PeopleState::Detail => self.render_detail(frame, area),
            PeopleState::Error(msg) => {
                let msg = msg.clone();
                self.render_error_state(frame, area, &msg);
            }
            PeopleState::List => unreachable!(),
        }
    }

    fn handle_event(&mut self, event: &Event) -> CartridgeAction {
        let Event::Key(key) = event else {
            return CartridgeAction::None;
        };

        if matches!(self.state, PeopleState::List) {
            return match key.code {
                KeyCode::Char('j') | KeyCode::Down => {
                    self.move_down();
                    CartridgeAction::Consumed
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    self.move_up();
                    CartridgeAction::Consumed
                }
                KeyCode::Enter => {
                    if self.selected_contact().is_some() {
                        self.state = PeopleState::Detail;
                    }
                    CartridgeAction::Consumed
                }
                KeyCode::Esc => CartridgeAction::GoBack,
                _ => CartridgeAction::None,
            };
        }

        if matches!(self.state, PeopleState::Detail) {
            return match key.code {
                KeyCode::Esc | KeyCode::Char('q') => {
                    self.state = PeopleState::List;
                    CartridgeAction::Consumed
                }
                _ => CartridgeAction::None,
            };
        }

        CartridgeAction::None
    }
}

// ── Background fetch ──────────────────────────────────────────────────────────

fn fetch_contacts(endpoint: &str, tx: &mpsc::Sender<BgMsg>) {
    let client = match reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            let _ = tx.send(BgMsg::Err(format!("client: {e}")));
            return;
        }
    };
    let url = format!("{}/v1/people", endpoint);
    match client.get(&url).send() {
        Ok(resp) if resp.status().is_success() => match resp.json::<ContactsResponse>() {
            Ok(r) => {
                let _ = tx.send(BgMsg::Contacts(r.contacts));
            }
            Err(e) => {
                let _ = tx.send(BgMsg::Err(format!("parse: {e}")));
            }
        },
        Ok(resp) => {
            let _ = tx.send(BgMsg::Err(format!("HTTP {}", resp.status())));
        }
        Err(e) => {
            let _ = tx.send(BgMsg::Err(format!("service-people unavailable: {e}")));
        }
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(max.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}
