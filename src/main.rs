mod ui;
mod utils;

use log::{set_boxed_logger, set_max_level};
use std::{
    io,
    sync::mpsc::{channel, Receiver},
    time::Duration,
    vec,
};

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    DefaultTerminal, Frame,
};

use ui::{cert::Cert, info::Info, target::Target};
use utils::info_logger::InfoLogger;

fn main() -> io::Result<()> {
    let (tx, rx) = channel();
    set_boxed_logger(Box::new(InfoLogger { sender: tx })).unwrap();
    set_max_level(log::LevelFilter::Debug);

    let mut terminal = ratatui::init();
    let mut app = App::default();
    app.set_log_revicer(rx);
    let runner = app.run(&mut terminal);
    ratatui::restore();
    runner
}

#[derive(Debug)]
pub struct App {
    exit: bool,
    conter: u32,
    log_rx: Option<Receiver<String>>,
    target_ui: Target,
    info_ui: Info,
    cert_ui: Cert,
}
impl Default for App {
    fn default() -> Self {
        Self {
            conter: 0,
            exit: false,
            log_rx: None,
            target_ui: Target::default(),
            info_ui: Info::default(),
            cert_ui: Cert::default(),
        }
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if event::poll(Duration::from_millis(100))? {
                self.handle_event_global_key()?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        self.poll_logs();
        let main_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints(vec![
                Constraint::Max(3),
                Constraint::Min(0),
                Constraint::Max(7),
            ])
            .split(frame.area());
        let top_layout = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Min(0), Constraint::Max(8)])
            .split(main_layout[0]);

        frame.render_widget(&self.target_ui, top_layout[0]);
        frame.render_widget(&self.cert_ui, top_layout[1]);

        frame.render_widget(&self.info_ui, main_layout[2]);
    }
    fn handle_event_global_key(&mut self) -> io::Result<()> {
        match event::read()? {
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key(key_event);
                self.target_ui.handle_key(key_event);
            }

            _ => {}
        }
        Ok(())
    }
    fn handle_key(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('p') => {
                self.conter += 1;
                log::warn!("test press count {}", self.conter)
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true
    }
    fn set_log_revicer(&mut self, rx: Receiver<String>) {
        self.log_rx = Some(rx);
    }

    fn poll_logs(&mut self) {
        if let Some(rx) = &self.log_rx {
            while let Ok(msg) = rx.try_recv() {
                self.info_ui.push(msg);
            }
        }
    }
}
