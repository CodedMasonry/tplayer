use std::time::Duration;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Block, BorderType, Gauge, StatefulWidget, Widget},
};

use crate::audio::CurrentTrack;

pub struct Progress {}

impl Progress {
    pub fn new() -> Self {
        Self {}
    }
}

impl StatefulWidget for Progress {
    type State = Option<CurrentTrack>;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Option<CurrentTrack>) {
        let (elapsed, total) = match state {
            Some(v) => (v.elapsed_duration, v.total_duration),
            None => (Duration::default(), Duration::default()),
        };
        let mut percent = match state {
            Some(v) => v.elapsed_duration.as_secs_f64() / v.total_duration.as_secs_f64(),
            None => 0.0,
        };

        // Out of bounds somehow happened
        if percent > 1.0 {
            percent = 1.0;
        }
        if percent < 0.0 {
            percent = 0.0;
        }

        let line = Gauge::default()
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().green()),
            )
            .gauge_style(Style::default().green().italic())
            .label(format!(
                "{} / {}",
                format_duration(elapsed),
                format_duration(total)
            ))
            .use_unicode(true)
            .ratio(percent);

        line.render(area, buf);
    }
}

fn format_duration(duration: Duration) -> String {
    let total_secs = duration.as_secs();
    let hours = total_secs / 3600;
    let minutes = (total_secs % 3600) / 60;
    let seconds = total_secs % 60;

    if hours > 0 {
        format!("{}:{:02}:{:02}", hours, minutes, seconds)
    } else {
        format!("{}:{:02}", minutes, seconds)
    }
}
