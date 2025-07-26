use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

pub struct StatusInfo {
    pub volume: f32,
    pub queue_len: usize,
}

pub struct Status {}

impl Status {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for Status {
    type State = StatusInfo;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut StatusInfo) {
        let vol = (state.volume * 100.0).floor();
        let queue = state.queue_len;

        let volume_line = Line::from(vec![
            Span::styled("V: ", Style::default().dim()),
            Span::styled(format!("{}%", vol), Style::default().bold()),
        ]);
        let queue_line = Line::from(vec![
            Span::styled("Q: ", Style::default().dim()),
            Span::styled(format!("{}", queue), Style::default().bold()),
        ]);

        let text = Paragraph::new(vec![volume_line, queue_line]).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::new().fg(Color::Green)),
        );

        text.render(area, buf);
    }
}
