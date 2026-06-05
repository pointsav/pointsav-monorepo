use crossterm::event::{Event, KeyCode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use app_console_keys::cartridge::{Cartridge, CartridgeAction};
use app_console_keys::fkey::FKey;

use crate::health::{fetch_readyz, DoormanHealth};

const POLL_INTERVAL: Duration = Duration::from_secs(10);

enum BgMsg {
    Health(DoormanHealth),
    Err(String),
}

pub struct SlmCartridge {
    endpoint: String,
    health: Option<DoormanHealth>,
    last_updated: Option<Instant>,
    error: Option<String>,
    plain: bool,
    refresh_tx: mpsc::SyncSender<()>,
    bg_rx: mpsc::Receiver<BgMsg>,
    show_help: bool,
}

impl SlmCartridge {
    pub fn new(endpoint: &str, plain: bool) -> Self {
        let (refresh_tx, refresh_rx) = mpsc::sync_channel::<()>(4);
        let (bg_tx, bg_rx) = mpsc::channel::<BgMsg>();

        let ep = endpoint.to_string();
        let thread_tx = bg_tx;
        std::thread::spawn(move || {
            poll_loop(ep, thread_tx, refresh_rx);
        });

        Self {
            endpoint: endpoint.to_string(),
            health: None,
            last_updated: None,
            error: None,
            plain,
            refresh_tx,
            bg_rx,
            show_help: false,
        }
    }

    fn trigger_refresh(&mut self) {
        let _ = self.refresh_tx.try_send(());
        self.error = None;
    }

    fn render_dashboard(&self, frame: &mut Frame, area: Rect) {
        let outer = Block::default()
            .title(" F9 — SLM Infrastructure ")
            .borders(Borders::ALL);
        let inner = outer.inner(area);
        frame.render_widget(outer, area);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(6), // Doorman section
                Constraint::Length(4), // Knowledge Graph section
                Constraint::Length(5), // Brief Queue section
                Constraint::Min(1),    // Controls hint
            ])
            .split(inner);

        // Doorman section
        let doorman_block = Block::default().title(" Doorman ").borders(Borders::ALL);
        let doorman_inner = doorman_block.inner(chunks[0]);
        frame.render_widget(doorman_block, chunks[0]);

        let doorman_lines = self.build_doorman_lines();
        let doorman_para = Paragraph::new(doorman_lines);
        frame.render_widget(doorman_para, doorman_inner);

        // Knowledge graph section
        let graph_block = Block::default()
            .title(" Knowledge Graph ")
            .borders(Borders::ALL);
        let graph_inner = graph_block.inner(chunks[1]);
        frame.render_widget(graph_block, chunks[1]);

        let graph_para = Paragraph::new(self.build_graph_lines());
        frame.render_widget(graph_para, graph_inner);

        // Brief Queue section
        let queue_block = Block::default()
            .title(" Brief Queue ")
            .borders(Borders::ALL);
        let queue_inner = queue_block.inner(chunks[2]);
        frame.render_widget(queue_block, chunks[2]);

        let queue_para = Paragraph::new(self.build_queue_lines());
        frame.render_widget(queue_para, queue_inner);

        // Controls hint
        let updated = self
            .last_updated
            .map(|t| {
                let secs = t.elapsed().as_secs();
                if secs < 60 {
                    format!("{}s ago", secs)
                } else {
                    format!("{}m ago", secs / 60)
                }
            })
            .unwrap_or_else(|| "never".into());
        let hint = Paragraph::new(format!(" R=refresh  ?=help  [updated {}]", updated)).style(
            Style::default().fg(if self.plain {
                Color::Reset
            } else {
                Color::DarkGray
            }),
        );
        frame.render_widget(hint, chunks[3]);
    }

    fn build_doorman_lines(&self) -> Vec<Line<'static>> {
        match &self.health {
            None => {
                if let Some(e) = &self.error {
                    vec![
                        Line::from(vec![
                            Span::styled(
                                if self.plain { "[ERROR] " } else { "✗  " },
                                if self.plain {
                                    Style::default()
                                } else {
                                    Style::default().fg(Color::Red)
                                },
                            ),
                            Span::raw(e.clone()),
                        ]),
                        Line::raw("   Doorman unreachable"),
                    ]
                } else {
                    vec![Line::raw("   Loading...")]
                }
            }
            Some(h) => {
                let ai_label = if h.ai_available {
                    (
                        if self.plain {
                            "[OK] available"
                        } else {
                            "✓  available"
                        },
                        Color::Green,
                    )
                } else {
                    (
                        if self.plain {
                            "[WAIT] unavailable"
                        } else {
                            "⚠  unavailable"
                        },
                        Color::Yellow,
                    )
                };
                let circuit = if h.tier_b_circuit_state.is_empty() {
                    "—".to_string()
                } else {
                    h.tier_b_circuit_state.clone()
                };
                let tier = h.active_tier.as_deref().unwrap_or("—");

                vec![
                    Line::from(vec![
                        Span::raw("  Status:    "),
                        Span::styled(
                            if self.plain {
                                "[RUNNING]"
                            } else {
                                "● running"
                            },
                            if self.plain {
                                Style::default()
                            } else {
                                Style::default()
                                    .fg(Color::Green)
                                    .add_modifier(Modifier::BOLD)
                            },
                        ),
                        Span::raw(format!("  {}", self.endpoint)),
                    ]),
                    Line::from(vec![
                        Span::raw("  AI:        "),
                        Span::styled(
                            ai_label.0.to_string(),
                            if self.plain {
                                Style::default()
                            } else {
                                Style::default().fg(ai_label.1)
                            },
                        ),
                    ]),
                    Line::raw(format!("  Circuit:    {}", circuit)),
                    Line::raw(format!("  Active tier: {}", tier)),
                ]
            }
        }
    }

    fn build_queue_lines(&self) -> Vec<Line<'static>> {
        match &self.health {
            None => vec![Line::raw("  —")],
            Some(h) => {
                let pending = h.queue_pending.unwrap_or(0);
                let done = h.queue_done.unwrap_or(0);
                let poison = h.queue_poison.unwrap_or(0);
                let poison_style = if poison > 0 {
                    if self.plain {
                        Style::default()
                    } else {
                        Style::default().fg(Color::Red)
                    }
                } else {
                    Style::default()
                };
                vec![
                    Line::raw(format!("  Pending:  {}", pending)),
                    Line::raw(format!("  Done:     {}", done)),
                    Line::from(vec![
                        Span::raw("  Poison:   "),
                        Span::styled(poison.to_string(), poison_style),
                    ]),
                ]
            }
        }
    }

    fn build_graph_lines(&self) -> Vec<Line<'static>> {
        match &self.health {
            None => vec![Line::raw("  Entities:  —")],
            Some(h) => {
                let count = h
                    .entity_count
                    .map(|n| {
                        if n >= 1_000 {
                            format!("{},{:03}", n / 1_000, n % 1_000)
                        } else {
                            n.to_string()
                        }
                    })
                    .unwrap_or_else(|| "—".into());
                vec![
                    Line::raw(format!("  Entities:  {}", count)),
                    Line::raw("  (DataGraph via Doorman healthz)"),
                ]
            }
        }
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" F9 SLM — Help ")
            .borders(Borders::ALL);
        let inner = block.inner(area);
        frame.render_widget(block, area);
        let text = format!(
            "Endpoint: {}\n\nKeybindings:\n  R       Refresh Doorman health\n  ?       Toggle this help\n  Esc     Return to previous cartridge\n\nData sources:\n  /readyz  Doorman readiness + tier + circuit state\n  entity_count from /readyz or /healthz\n\nTo connect to Doorman, ensure local-doorman.service is running.",
            self.endpoint
        );
        let para = Paragraph::new(text);
        frame.render_widget(para, inner);
    }
}

impl Cartridge for SlmCartridge {
    fn fkey(&self) -> FKey {
        FKey::F9
    }

    fn title(&self) -> &str {
        "SLM"
    }

    fn tick(&mut self) {
        while let Ok(msg) = self.bg_rx.try_recv() {
            match msg {
                BgMsg::Health(h) => {
                    self.health = Some(h);
                    self.last_updated = Some(Instant::now());
                    self.error = None;
                }
                BgMsg::Err(e) => {
                    self.error = Some(e);
                    self.last_updated = Some(Instant::now());
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        if self.show_help {
            self.render_help(frame, area);
        } else {
            self.render_dashboard(frame, area);
        }
    }

    fn handle_event(&mut self, event: &Event) -> CartridgeAction {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    self.trigger_refresh();
                    CartridgeAction::Consumed
                }
                KeyCode::Char('?') => {
                    self.show_help = !self.show_help;
                    CartridgeAction::Consumed
                }
                KeyCode::Esc if self.show_help => {
                    self.show_help = false;
                    CartridgeAction::Consumed
                }
                _ => CartridgeAction::None,
            }
        } else {
            CartridgeAction::None
        }
    }
}

fn poll_loop(endpoint: String, tx: mpsc::Sender<BgMsg>, refresh_rx: mpsc::Receiver<()>) {
    // Initial fetch — ignore Result; Err means sender dropped, handled in loop below
    let _ = do_fetch(&endpoint, &tx);

    while let Ok(()) | Err(mpsc::RecvTimeoutError::Timeout) = refresh_rx.recv_timeout(POLL_INTERVAL)
    {
        if do_fetch(&endpoint, &tx).is_err() {
            // Sender dropped — chassis is shutting down
            break;
        }
    }
}

fn do_fetch(endpoint: &str, tx: &mpsc::Sender<BgMsg>) -> Result<(), ()> {
    let msg = match fetch_readyz(endpoint) {
        Ok(h) => BgMsg::Health(h),
        Err(e) => BgMsg::Err(e.to_string()),
    };
    tx.send(msg).map_err(|_| ())
}
