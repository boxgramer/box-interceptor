use std::io;

use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    style::{Style, Stylize},
    symbols::border,
    widgets::{Block, Paragraph, Tabs, Widget},
};

#[derive(Debug, Default)]
pub struct Cert {}
impl Cert {
    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            _ => {}
        }
    }
}

impl Widget for &Cert {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().border_set(border::THICK);
        let content = Paragraph::new("CERT")
            .block(block)
            .alignment(ratatui::layout::Alignment::Center);

        content.render(area, buf);
    }
}
