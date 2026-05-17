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
    time::Duration,
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
}

impl AppSession {
    fn new() -> Self {
        Self {
            handle: None,
            channel_id: None,
            term_cols: 220,
            term_rows: 50,
            input_tx: None,
        }
    }
}

impl Handler for AppSession {
    type Error = anyhow::Error;

    async fn auth_publickey(
        &mut self,
        _user: &str,
        _key: &russh::keys::PublicKey,
    ) -> Result<Auth, Self::Error> {
        Ok(Auth::Accept)
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

        tokio::task::spawn_blocking(move || run_tui(terminal, input_rx));

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
) {
    {
        let backend = terminal.backend_mut();
        let _ = execute!(backend, EnterAlternateScreen, cursor::Hide);
    }

    loop {
        let draw_result = terminal.draw(|f| {
            let area = f.area();

            let outer = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan))
                .title(" PROOFREADER — Leapfrog 2030 ")
                .title_alignment(Alignment::Center);
            f.render_widget(outer, area);

            let inner = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Fill(1),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .margin(1)
                .split(area);

            let greeting = Paragraph::new(
                "Session 1 — SSH + ratatui spike\n\nPress  q  to quit",
            )
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::White));
            f.render_widget(greeting, inner[0]);

            let status = Paragraph::new("[ Tier A (OLMo 3 7B) │ Ready │ 127.0.0.1:9092 ]")
                .alignment(Alignment::Center)
                .style(
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::BOLD),
                );
            f.render_widget(status, inner[1]);
        });

        if draw_result.is_err() {
            break;
        }

        match input_rx.recv_timeout(Duration::from_millis(50)) {
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
