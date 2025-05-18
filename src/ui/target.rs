use std::io;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::{Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Tabs, Widget},
};

#[derive(Debug, Default)]
pub struct Target {
    targes: Vec<String>,
    index: Option<usize>,
}
impl Target {
    fn set_index(&mut self, idx: Option<usize>) {
        self.index = idx;
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('1') => self.set_index(Some(0)),
            KeyCode::Char('2') => self.set_index(Some(1)),
            KeyCode::Char('3') => self.set_index(Some(2)),
            KeyCode::Char('4') => self.set_index(Some(3)),
            _ => {}
        }
    }
}

impl Widget for &Target {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title = Line::from("Target".bold());
        let block = Block::bordered().title(title).border_set(border::THICK);

        let tabs = Tabs::new(vec!["1", "2", "3", "4"])
            .block(block)
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .select(self.index)
            .divider("|")
            .padding(" ", " ");

        tabs.render(area, buf);
    }
}
