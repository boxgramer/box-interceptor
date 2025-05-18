use ratatui::{
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct Info {
    logs: Vec<String>,
    scroll: u16,
}

impl Info {
    pub fn push(&mut self, msg: impl Into<String>) {
        self.logs.push(msg.into());

        self.scroll = self.logs.len().saturating_sub(5) as u16
    }
}
impl Widget for &Info {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let text: Vec<Line> = self.logs.iter().map(|l| Line::from(l.as_str())).collect();

        let paragraph = Paragraph::new(text)
            .block(Block::bordered().title("Info").border_set(border::THICK))
            .scroll((self.scroll, 0));

        paragraph.render(area, buf);
    }
}
