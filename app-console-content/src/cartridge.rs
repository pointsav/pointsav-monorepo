use std::sync::mpsc;
use std::thread;

use app_console_keys::{Cartridge, CartridgeAction, FKey};
use crossterm::event::{Event, KeyCode, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
    Frame,
};
use tui_textarea::TextArea;

use crate::draft::{self, DraftEvent};
use crate::drafts_out;
use crate::proofreader::{self, ProofreadResponse, DEFAULT_PROTOCOL_IDX, PROTOCOLS};

const SPINNER: &[&str] = &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
const PLACEHOLDER: &str =
    "Paste or type text to proofread — Ctrl-S to submit · Tab to pick protocol · /new <title> to draft";

// ── State machine ─────────────────────────────────────────────────────────────

enum ContentState {
    Input {
        protocol_idx: usize,
    },
    PickProtocol {
        saved_text: Vec<String>,
        selected: usize,
    },
    Submitting {
        original: String,
        #[allow(dead_code)]
        protocol_idx: usize,
        rx: mpsc::Receiver<anyhow::Result<ProofreadResponse>>,
        spinner: usize,
    },
    Results {
        response: ProofreadResponse,
        original: String,
        scroll: u16,
    },
    DraftingNew {
        title: String,
        protocol_idx: usize,
        rx: mpsc::Receiver<DraftEvent>,
        buffer: String,
        done: bool,
        error: Option<String>,
        scroll: u16,
    },
    Error {
        message: String,
    },
}

// ── ContentCartridge ──────────────────────────────────────────────────────────

pub struct ContentCartridge {
    username: String,
    tenant: String,
    proof_endpoint: String,
    slm_endpoint: String,
    drafts_outbound_path: String,
    state: ContentState,
    textarea: TextArea<'static>,
}

impl ContentCartridge {
    pub fn new() -> Self {
        Self::new_for(
            "operator",
            "local",
            "http://127.0.0.1:9092",
            "http://localhost:9080",
            format!(
                "{}/.local/share/os-console/drafts-outbound",
                std::env::var("HOME").unwrap_or_else(|_| ".".into())
            ),
        )
    }

    pub fn new_for(
        username: impl Into<String>,
        tenant: impl Into<String>,
        proof_endpoint: impl Into<String>,
        slm_endpoint: impl Into<String>,
        drafts_outbound_path: impl Into<String>,
    ) -> Self {
        let mut ta = TextArea::default();
        ta.set_placeholder_text(PLACEHOLDER);
        Self {
            username: username.into(),
            tenant: tenant.into(),
            proof_endpoint: proof_endpoint.into(),
            slm_endpoint: slm_endpoint.into(),
            drafts_outbound_path: drafts_outbound_path.into(),
            state: ContentState::Input {
                protocol_idx: DEFAULT_PROTOCOL_IDX,
            },
            textarea: ta,
        }
    }

    fn reset_textarea(&mut self, protocol_idx: usize) {
        let mut ta = TextArea::default();
        ta.set_placeholder_text(PLACEHOLDER);
        self.textarea = ta;
        self.state = ContentState::Input { protocol_idx };
    }

    // ── Render helpers ────────────────────────────────────────────────────────

    fn render_input(&mut self, frame: &mut Frame, area: Rect, protocol_idx: usize) {
        let (slug, display) = PROTOCOLS[protocol_idx];
        let outer = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" F4: Content — Proofreader ");
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1), Constraint::Length(1)])
            .split(inner);

        frame.render_widget(&self.textarea, chunks[0]);

        let hint = Paragraph::new(format!(
            " Protocol: {}  —  {}    [Tab: change  Ctrl-S: submit  /new <title> Ctrl-S: draft  q/Ctrl-C: quit]",
            slug, display
        ))
        .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(hint, chunks[1]);
    }

    fn render_picker(frame: &mut Frame, area: Rect, selected: usize) {
        let outer = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow))
            .title(" F4: Content — Pick Protocol    [↑↓: navigate  Enter: select  Esc: back] ");
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        let items: Vec<ListItem> = PROTOCOLS
            .iter()
            .enumerate()
            .map(|(i, (slug, name))| {
                let style = if i == selected {
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Cyan)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                };
                ListItem::new(format!("  {}  —  {}", slug, name)).style(style)
            })
            .collect();

        frame.render_widget(List::new(items), inner);
    }

    fn render_submitting(frame: &mut Frame, area: Rect, spinner: usize) {
        let outer = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan))
            .title(" F4: Content — Proofreading... ");
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        let mid = Rect {
            y: inner.y + inner.height / 2,
            height: 2,
            ..inner
        };
        frame.render_widget(
            Paragraph::new(format!(
                "  {} Sending to service-proofreader — please wait (up to 300s)…",
                SPINNER[spinner % SPINNER.len()]
            ))
            .style(Style::default().fg(Color::Yellow)),
            mid,
        );
    }

    fn render_results(
        frame: &mut Frame,
        area: Rect,
        response: &ProofreadResponse,
        original: &str,
        scroll: u16,
    ) {
        use similar::{ChangeTag, TextDiff};

        let degraded_str = if response.degraded.is_empty() {
            String::new()
        } else {
            format!("  [DEGRADED: {}]", response.degraded.join(", "))
        };
        let title = format!(
            " F4: Content — Results{}    [A: accept  R: reject  Esc: back  ↑↓: scroll] ",
            degraded_str
        );

        let outer = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green))
            .title(title.as_str());
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Fill(1)])
            .split(inner);

        // Info line
        let tier = response.tier_used.as_deref().unwrap_or("?");
        let tmpl = response
            .template_display_name
            .as_deref()
            .unwrap_or(&response.protocol);
        frame.render_widget(
            Paragraph::new(format!(" tier: {}  │  {}", tier, tmpl))
                .style(Style::default().fg(Color::DarkGray)),
            chunks[0],
        );

        // Diff pane
        let diff = TextDiff::from_lines(original, &response.improved_text);
        let mut lines: Vec<Line> = Vec::new();

        for change in diff.iter_all_changes() {
            let (prefix, style) = match change.tag() {
                ChangeTag::Delete => ("- ", Style::default().fg(Color::Red)),
                ChangeTag::Insert => ("+ ", Style::default().fg(Color::LightGreen)),
                ChangeTag::Equal => ("  ", Style::default().fg(Color::DarkGray)),
            };
            let text = change.value().trim_end_matches('\n');
            lines.push(Line::from(Span::styled(
                format!("{}{}", prefix, text),
                style,
            )));
        }

        if lines.is_empty() || original == response.improved_text {
            lines = vec![Line::from(Span::styled(
                "  (No changes — service degraded or text already clean)",
                Style::default().fg(Color::DarkGray),
            ))];
        }

        let total = lines.len() as u16;
        let visible = chunks[1].height;
        let offset = scroll.min(total.saturating_sub(visible));

        frame.render_widget(Paragraph::new(lines.clone()).scroll((offset, 0)), chunks[1]);

        if total > visible {
            let mut sb_state = ScrollbarState::new(total as usize).position(offset as usize);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                chunks[1],
                &mut sb_state,
            );
        }
    }

    fn render_error(frame: &mut Frame, area: Rect, message: &str) {
        let outer = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Red))
            .title(" F4: Content — Error    [any key: back] ");
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        let mid = Rect {
            y: inner.y + inner.height / 2,
            height: 3,
            ..inner
        };
        frame.render_widget(
            Paragraph::new(format!("  Error: {}", message)).style(Style::default().fg(Color::Red)),
            mid,
        );
    }

    fn render_drafting(
        frame: &mut Frame,
        area: Rect,
        title: &str,
        buffer: &str,
        done: bool,
        error: Option<&str>,
        scroll: u16,
    ) {
        let (border_color, status) = if let Some(err) = error {
            (Color::Red, format!(" Error: {} ", err))
        } else if done {
            (Color::Green, " [Enter: accept  Esc: discard] ".to_string())
        } else {
            (Color::Yellow, " [drafting…  Esc: cancel] ".to_string())
        };

        let title_str = format!(" F4: Content — Draft: \"{}\" {} ", title, status);
        let outer = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(title_str.as_str());
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        let lines: Vec<Line> = buffer
            .lines()
            .map(|l| {
                Line::from(Span::styled(
                    l.to_string(),
                    Style::default().fg(Color::White),
                ))
            })
            .collect();

        let total = lines.len() as u16;
        let visible = inner.height;
        // Auto-scroll to bottom while streaming; user can override once done
        let offset = if done {
            scroll.min(total.saturating_sub(visible))
        } else {
            total.saturating_sub(visible)
        };

        frame.render_widget(Paragraph::new(lines.clone()).scroll((offset, 0)), inner);

        if total > visible {
            let mut sb = ScrollbarState::new(total as usize).position(offset as usize);
            frame.render_stateful_widget(
                Scrollbar::new(ScrollbarOrientation::VerticalRight),
                inner,
                &mut sb,
            );
        }
    }

    // ── Event handlers ────────────────────────────────────────────────────────

    fn on_input_key(&mut self, event: &Event, protocol_idx: usize) -> CartridgeAction {
        let Event::Key(key) = event else {
            return CartridgeAction::None;
        };

        // Let F-keys and Ctrl-C fall through to chassis
        if matches!(key.code, KeyCode::F(_)) {
            return CartridgeAction::None;
        }
        if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
            return CartridgeAction::None;
        }

        // Tab → protocol picker
        if key.code == KeyCode::Tab {
            let saved: Vec<String> = self
                .textarea
                .lines()
                .iter()
                .map(|s| s.to_string())
                .collect();
            self.state = ContentState::PickProtocol {
                saved_text: saved,
                selected: protocol_idx,
            };
            return CartridgeAction::Consumed;
        }

        // Ctrl-S → proofread OR /new <title> → draft mode
        if key.code == KeyCode::Char('s') && key.modifiers.contains(KeyModifiers::CONTROL) {
            let text = self.textarea.lines().join("\n");
            if text.trim().is_empty() {
                return CartridgeAction::Consumed;
            }

            // Intercept /new <title> as draft-mode trigger
            let trimmed = text.trim();
            if let Some(rest) = trimmed.strip_prefix("/new") {
                let title = rest.trim().to_string();
                let title = if title.is_empty() {
                    "Untitled".to_string()
                } else {
                    title
                };
                let protocol = PROTOCOLS[protocol_idx].0.to_string();
                let tenant = self.tenant.clone();
                let slm = self.slm_endpoint.clone();
                // Use the title as the prompt — operator can refine before submitting
                let prompt = title.clone();
                let (tx, rx) = mpsc::channel();
                thread::spawn(move || {
                    draft::stream_draft(&prompt, &protocol, &tenant, &slm, tx);
                });
                self.state = ContentState::DraftingNew {
                    title,
                    protocol_idx,
                    rx,
                    buffer: String::new(),
                    done: false,
                    error: None,
                    scroll: 0,
                };
                return CartridgeAction::Consumed;
            }

            let protocol = PROTOCOLS[protocol_idx].0.to_string();
            let tenant = self.tenant.clone();
            let endpoint = self.proof_endpoint.clone();
            let text_clone = text.clone();
            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                let _ = tx.send(proofreader::submit_proofread(
                    &text_clone,
                    &protocol,
                    &tenant,
                    &endpoint,
                ));
            });
            self.state = ContentState::Submitting {
                original: text,
                protocol_idx,
                rx,
                spinner: 0,
            };
            return CartridgeAction::Consumed;
        }

        // Everything else → textarea
        self.textarea
            .input(tui_textarea::Input::from(event.clone()));
        CartridgeAction::Consumed
    }

    fn on_picker_key(
        &mut self,
        key: &crossterm::event::KeyEvent,
        selected: usize,
        saved: &[String],
    ) -> CartridgeAction {
        match key.code {
            KeyCode::Esc | KeyCode::BackTab => {
                let saved_clone = saved.to_vec();
                let mut ta = TextArea::from(saved_clone);
                ta.set_placeholder_text(PLACEHOLDER);
                self.textarea = ta;
                self.state = ContentState::Input {
                    protocol_idx: selected,
                };
            }
            KeyCode::Up => {
                if let ContentState::PickProtocol { selected: s, .. } = &mut self.state {
                    if *s > 0 {
                        *s -= 1;
                    }
                }
            }
            KeyCode::Down => {
                if let ContentState::PickProtocol { selected: s, .. } = &mut self.state {
                    if *s < PROTOCOLS.len() - 1 {
                        *s += 1;
                    }
                }
            }
            KeyCode::Enter => {
                let saved_clone = saved.to_vec();
                let mut ta = TextArea::from(saved_clone);
                ta.set_placeholder_text(PLACEHOLDER);
                self.textarea = ta;
                self.state = ContentState::Input {
                    protocol_idx: selected,
                };
            }
            _ => {}
        }
        CartridgeAction::Consumed
    }

    fn on_drafting_key(&mut self, key: &crossterm::event::KeyEvent) -> CartridgeAction {
        // Extract needed values without keeping a borrow across mutable calls
        let (done, title, protocol_idx, buffer) = match &self.state {
            ContentState::DraftingNew {
                done,
                title,
                protocol_idx,
                buffer,
                ..
            } => (*done, title.clone(), *protocol_idx, buffer.clone()),
            _ => return CartridgeAction::Consumed,
        };

        match key.code {
            KeyCode::Esc => {
                self.reset_textarea(protocol_idx);
            }
            KeyCode::Enter | KeyCode::Char('a') | KeyCode::Char('A') if done => {
                let protocol = PROTOCOLS[protocol_idx].0.to_string();
                let tenant = self.tenant.clone();
                let username = self.username.clone();
                let outdir = self.drafts_outbound_path.clone();
                thread::spawn(move || {
                    let _ = drafts_out::write_draft(
                        &title, &protocol, &buffer, &tenant, &username, &outdir,
                    );
                });
                self.reset_textarea(DEFAULT_PROTOCOL_IDX);
            }
            KeyCode::Up if done => {
                if let ContentState::DraftingNew { scroll, .. } = &mut self.state {
                    *scroll = scroll.saturating_sub(1);
                }
            }
            KeyCode::Down if done => {
                if let ContentState::DraftingNew { scroll, .. } = &mut self.state {
                    *scroll = scroll.saturating_add(1);
                }
            }
            _ => {}
        }
        CartridgeAction::Consumed
    }

    fn on_results_key(&mut self, key: &crossterm::event::KeyEvent) -> CartridgeAction {
        match key.code {
            KeyCode::Char('q') => {
                return CartridgeAction::None; // chassis quits
            }
            KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Enter | KeyCode::Esc => {
                if let ContentState::Results { response, .. } = &self.state {
                    let rid = response.request_id.clone();
                    let tenant = self.tenant.clone();
                    let endpoint = self.proof_endpoint.clone();
                    thread::spawn(move || {
                        let _ = proofreader::post_verdict(&rid, &tenant, "accept", &endpoint);
                    });
                }
                self.reset_textarea(DEFAULT_PROTOCOL_IDX);
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                if let ContentState::Results { response, .. } = &self.state {
                    let rid = response.request_id.clone();
                    let tenant = self.tenant.clone();
                    let endpoint = self.proof_endpoint.clone();
                    thread::spawn(move || {
                        let _ = proofreader::post_verdict(&rid, &tenant, "reject", &endpoint);
                    });
                }
                self.reset_textarea(DEFAULT_PROTOCOL_IDX);
            }
            KeyCode::Up => {
                if let ContentState::Results { scroll, .. } = &mut self.state {
                    *scroll = scroll.saturating_sub(1);
                }
            }
            KeyCode::Down => {
                if let ContentState::Results { scroll, .. } = &mut self.state {
                    *scroll = scroll.saturating_add(1);
                }
            }
            _ => {}
        }
        CartridgeAction::Consumed
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
        // Poll for HTTP result (non-blocking)
        let new_state: Option<ContentState> =
            if let ContentState::Submitting { rx, original, .. } = &mut self.state {
                match rx.try_recv() {
                    Ok(Ok(resp)) => Some(ContentState::Results {
                        response: resp,
                        original: original.clone(),
                        scroll: 0,
                    }),
                    Ok(Err(e)) => Some(ContentState::Error {
                        message: e.to_string(),
                    }),
                    Err(mpsc::TryRecvError::Disconnected) => Some(ContentState::Error {
                        message: "HTTP thread disconnected unexpectedly".into(),
                    }),
                    Err(mpsc::TryRecvError::Empty) => None,
                }
            } else {
                None
            };
        if let Some(ns) = new_state {
            self.state = ns;
        }

        // Drain SSE tokens for DraftingNew
        if let ContentState::DraftingNew {
            rx,
            buffer,
            done,
            error,
            ..
        } = &mut self.state
        {
            loop {
                match rx.try_recv() {
                    Ok(DraftEvent::Token(tok)) => buffer.push_str(&tok),
                    Ok(DraftEvent::Done) => {
                        *done = true;
                        break;
                    }
                    Ok(DraftEvent::Error(e)) => {
                        *error = Some(e);
                        *done = true;
                        break;
                    }
                    Err(mpsc::TryRecvError::Empty) => break,
                    Err(mpsc::TryRecvError::Disconnected) => {
                        *done = true;
                        break;
                    }
                }
            }
        }

        // Tick spinner
        if let ContentState::Submitting { spinner, .. } = &mut self.state {
            *spinner = spinner.wrapping_add(1);
        }

        // Extract state data — borrow ends before calling render helpers
        enum Cmd {
            Input(usize),
            Picker(usize),
            Submitting(usize),
            Results(ProofreadResponse, String, u16),
            Drafting(String, String, bool, Option<String>, u16),
            Error(String),
        }
        let cmd = match &self.state {
            ContentState::Input { protocol_idx } => Cmd::Input(*protocol_idx),
            ContentState::PickProtocol { selected, .. } => Cmd::Picker(*selected),
            ContentState::Submitting { spinner, .. } => Cmd::Submitting(*spinner),
            ContentState::Results {
                response,
                original,
                scroll,
            } => Cmd::Results(response.clone(), original.clone(), *scroll),
            ContentState::DraftingNew {
                title,
                buffer,
                done,
                error,
                scroll,
                ..
            } => Cmd::Drafting(title.clone(), buffer.clone(), *done, error.clone(), *scroll),
            ContentState::Error { message } => Cmd::Error(message.clone()),
        };

        match cmd {
            Cmd::Input(pidx) => self.render_input(frame, area, pidx),
            Cmd::Picker(sel) => Self::render_picker(frame, area, sel),
            Cmd::Submitting(sp) => Self::render_submitting(frame, area, sp),
            Cmd::Results(resp, orig, sc) => Self::render_results(frame, area, &resp, &orig, sc),
            Cmd::Drafting(t, buf, done, err, sc) => {
                Self::render_drafting(frame, area, &t, &buf, done, err.as_deref(), sc)
            }
            Cmd::Error(msg) => Self::render_error(frame, area, &msg),
        }
    }

    fn handle_event(&mut self, event: &Event) -> CartridgeAction {
        let Event::Key(key) = event else {
            return CartridgeAction::None;
        };

        // Extract state discriminant and data (borrow ends before we mutate)
        enum StateKind {
            Input(usize),
            Picker(usize, Vec<String>),
            Submitting,
            Results,
            Drafting,
            Error,
        }
        let kind = match &self.state {
            ContentState::Input { protocol_idx } => StateKind::Input(*protocol_idx),
            ContentState::PickProtocol {
                selected,
                saved_text,
            } => StateKind::Picker(*selected, saved_text.clone()),
            ContentState::Submitting { .. } => StateKind::Submitting,
            ContentState::Results { .. } => StateKind::Results,
            ContentState::DraftingNew { .. } => StateKind::Drafting,
            ContentState::Error { .. } => StateKind::Error,
        };

        match kind {
            StateKind::Input(pidx) => self.on_input_key(event, pidx),
            StateKind::Picker(sel, saved) => self.on_picker_key(key, sel, &saved),
            StateKind::Submitting => CartridgeAction::Consumed,
            StateKind::Results => self.on_results_key(key),
            StateKind::Drafting => self.on_drafting_key(key),
            StateKind::Error => {
                self.reset_textarea(DEFAULT_PROTOCOL_IDX);
                CartridgeAction::Consumed
            }
        }
    }
}
