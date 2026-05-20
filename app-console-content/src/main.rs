use anyhow::Result;
use crossterm::{
    cursor,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use russh::{
    server::{Auth, Config, Handler, Msg, Server, Session},
    Channel, ChannelId,
};
use std::{
    io::Write,
    net::SocketAddr,
    sync::{mpsc, Arc},
    time::{Duration, Instant},
};

use app_console_content::{
    auth::compute_fingerprint,
    db::{find_user, open_db},
    session::User,
    ui::status_bar::{fetch_health, HealthState},
};

// ---------------------------------------------------------------------------
// TerminalHandle — bridges ratatui's Write output to the SSH channel
// ---------------------------------------------------------------------------

struct TerminalHandle {
    sender: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
    sink: Vec<u8>,
}

impl TerminalHandle {
    fn new(handle: russh::server::Handle, channel_id: ChannelId) -> Self {
        let (sender, mut receiver) = tokio::sync::mpsc::unbounded_channel::<Vec<u8>>();
        tokio::spawn(async move {
            while let Some(data) = receiver.recv().await {
                if handle.data(channel_id, data).await.is_err() {
                    break;
                }
            }
        });
        Self { sender, sink: Vec::new() }
    }
}

impl Write for TerminalHandle {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.sink.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let data = self.sink.clone();
        self.sink.clear();
        self.sender
            .send(data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::BrokenPipe, e))
    }
}

// ---------------------------------------------------------------------------
// AppSession — handles one SSH client connection
// ---------------------------------------------------------------------------

struct AppSession {
    handle: Option<russh::server::Handle>,
    channel_id: Option<ChannelId>,
    term_cols: u32,
    term_rows: u32,
    input_tx: Option<mpsc::SyncSender<u8>>,
    user: Option<User>,
}

impl AppSession {
    fn new() -> Self {
        Self {
            handle: None,
            channel_id: None,
            term_cols: 220,
            term_rows: 50,
            input_tx: None,
            user: None,
        }
    }
}

impl Handler for AppSession {
    type Error = anyhow::Error;

    async fn auth_publickey(
        &mut self,
        _ssh_user: &str,
        key: &russh::keys::PublicKey,
    ) -> Result<Auth, Self::Error> {
        let fingerprint = compute_fingerprint(key);
        match open_db().and_then(|conn| find_user(&conn, &fingerprint)) {
            Ok(Some(user)) => {
                eprintln!("proof: auth accepted for {}@{}", user.username, user.tenant.as_str());
                self.user = Some(user);
                Ok(Auth::Accept)
            }
            Ok(None) => {
                eprintln!("proof: auth rejected — fingerprint not registered: {fingerprint}");
                Ok(Auth::Reject { proceed_with_methods: None, partial_success: false })
            }
            Err(e) => {
                eprintln!("proof: auth db error: {e}");
                Ok(Auth::Reject { proceed_with_methods: None, partial_success: false })
            }
        }
    }

    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        session: &mut Session,
    ) -> Result<bool, Self::Error> {
        self.handle = Some(session.handle());
        self.channel_id = Some(channel.id());
        Ok(true)
    }

    async fn pty_request(
        &mut self,
        channel: ChannelId,
        _term: &str,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _modes: &[(russh::Pty, u32)],
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.term_cols = col_width;
        self.term_rows = row_height;
        session.channel_success(channel)?;
        Ok(())
    }

    async fn shell_request(
        &mut self,
        channel: ChannelId,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        let cols = self.term_cols as u16;
        let rows = self.term_rows as u16;

        let user = match self.user.take() {
            Some(u) => u,
            None => {
                eprintln!("proof: shell_request without authenticated user — rejecting");
                return Ok(());
            }
        };

        // Fetch health state before entering the blocking render loop.
        let health = fetch_health().await;

        let handle = self.handle.take().expect("handle set in channel_open_session");
        let ch_id = self.channel_id.unwrap_or(channel);

        let terminal_handle = TerminalHandle::new(handle, ch_id);
        let backend = CrosstermBackend::new(terminal_handle);
        let mut terminal = Terminal::new(backend)?;
        terminal.resize(ratatui::layout::Rect {
            x: 0,
            y: 0,
            width: cols,
            height: rows,
        })?;

        let (input_tx, input_rx) = mpsc::sync_channel::<u8>(256);
        self.input_tx = Some(input_tx);

        session.channel_success(channel)?;

        let started = Instant::now();
        tokio::task::spawn_blocking(move || run_tui(terminal, input_rx, user, health, started));

        Ok(())
    }

    async fn data(
        &mut self,
        _channel: ChannelId,
        data: &[u8],
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        if let Some(tx) = &self.input_tx {
            for &byte in data {
                let _ = tx.send(byte);
            }
        }
        Ok(())
    }

    async fn window_change_request(
        &mut self,
        _channel: ChannelId,
        col_width: u32,
        row_height: u32,
        _pix_width: u32,
        _pix_height: u32,
        _session: &mut Session,
    ) -> Result<(), Self::Error> {
        self.term_cols = col_width;
        self.term_rows = row_height;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// run_tui — ratatui render loop (runs in spawn_blocking)
// ---------------------------------------------------------------------------

fn run_tui(
    mut terminal: Terminal<CrosstermBackend<TerminalHandle>>,
    input_rx: mpsc::Receiver<u8>,
    user: User,
    health: HealthState,
    started: Instant,
) {
    {
        let backend = terminal.backend_mut();
        let _ = execute!(backend, EnterAlternateScreen, cursor::Hide);
    }

    loop {
        let elapsed = started.elapsed().as_secs();

        let draw_result = terminal.draw(|f| {
            let area = f.area();

            // Split: content fills remaining rows; status bar is 1 row at bottom.
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Length(1)])
                .split(area);

            // --- Content area ---
            let title = format!(" PROOFREADER — {}@{} ", user.username, user.tenant.as_str());
            let outer = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(title)
                .title_alignment(Alignment::Center);
            f.render_widget(outer, chunks[0]);

            let inner = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1), Constraint::Length(1)])
                .margin(1)
                .split(chunks[0]);

            let body = format!(
                "Session 2 — Auth complete\n\nLogged in as: {}@{}\n\nPress  q  to quit",
                user.username,
                user.tenant.as_str(),
            );
            let content = Paragraph::new(body)
                .alignment(Alignment::Center)
                .style(Style::default().fg(Color::White).add_modifier(Modifier::BOLD));
            f.render_widget(content, inner[0]);

            // --- Status bar ---
            app_console_content::ui::status_bar::render(f, chunks[1], &user, &health, elapsed);
        });

        if draw_result.is_err() {
            break;
        }

        match input_rx.recv_timeout(Duration::from_millis(500)) {
            Ok(b'q') | Ok(3) => break,
            Ok(_) => {}
            Err(mpsc::RecvTimeoutError::Timeout) => {}
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    {
        let backend = terminal.backend_mut();
        let _ = execute!(backend, LeaveAlternateScreen, cursor::Show);
    }
}

// ---------------------------------------------------------------------------
// AppServer — creates a new AppSession per incoming connection
// ---------------------------------------------------------------------------

#[derive(Clone)]
struct AppServer;

impl Server for AppServer {
    type Handler = AppSession;

    fn new_client(&mut self, _peer_addr: Option<SocketAddr>) -> AppSession {
        AppSession::new()
    }
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

#[tokio::main]
async fn main() -> Result<()> {
    let key = russh::keys::PrivateKey::random(
        &mut rand::rng(),
        russh::keys::Algorithm::Ed25519,
    )
    .expect("ed25519 key generation failed");

    let config = Arc::new(Config {
        inactivity_timeout: Some(Duration::from_secs(3600)),
        auth_rejection_time: Duration::from_secs(0),
        auth_rejection_time_initial: Some(Duration::from_secs(0)),
        keys: vec![key],
        ..Default::default()
    });

    let addr: SocketAddr = "0.0.0.0:2222".parse()?;
    eprintln!("proof: listening on {addr}  (ssh -p 2222 proof@localhost)");

    let mut server = AppServer;
    server.run_on_address(config, addr).await?;

    Ok(())
}
