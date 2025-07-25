use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Paragraph, StatefulWidget, Widget},
};

use crate::audio::CurrentTrack;

pub struct CurrentPlaying {}

impl CurrentPlaying {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for CurrentPlaying {
    type State = Option<CurrentTrack>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Option<CurrentTrack>) {
        let title = match state {
            Some(v) => v.track.title.clone(),
            None => "Nothing Playing".to_string(),
        };
        let artist = match state {
            Some(v) => v.track.artists.to_string(),
            None => "Nothing Playing".to_string(),
        };

        let title = Line::styled(title, Style::new().bold());
        let artist = Line::styled(artist, Style::new().italic().dim());

        let text = Paragraph::new(vec![title, artist]).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .border_style(Style::new().green()),
        );

        text.render(area, buf);
    }
}
