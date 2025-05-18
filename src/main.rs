use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    DefaultTerminal, Frame,
};

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app = App::default().run(&mut terminal);
    ratatui::restore();
    app
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_event_global_key()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {}
    fn handle_event_global_key(&mut self) -> io::Result<()> {
        match event::read()? {
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true
    }
}
