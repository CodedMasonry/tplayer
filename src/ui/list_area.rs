use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, List, ListState, StatefulWidget},
};

pub struct ListArea<'a> {
    list: Vec<Text<'a>>,
    is_focused: bool,
}

impl<'a> ListArea<'a> {
    pub fn new(list: Vec<Text<'a>>, is_focused: bool) -> Self {
        Self { list, is_focused }
    }
}

impl StatefulWidget for ListArea<'_> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let list = List::new(self.list)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .border_style(match self.is_focused {
                        true => Style::default().fg(Color::Green),
                        false => Style::default().fg(Color::Green),
                    }),
            )
            .highlight_style(match self.is_focused {
                true => Style::default().reversed().fg(Color::Green).not_dim(),
                false => Style::default().fg(Color::Green).not_dim(),
            })
            .highlight_symbol("|")
            .repeat_highlight_symbol(true);

        list.render(area, buf, state);
    }
}
