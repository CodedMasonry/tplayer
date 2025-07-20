use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Text,
    widgets::{Block, BorderType, List, ListState, StatefulWidget},
};

pub struct AlbumBar<'a> {
    list: Vec<Text<'a>>,
}

impl<'a> AlbumBar<'a> {
    pub fn new(list: Vec<Text<'a>>) -> Self {
        Self { list }
    }
}

impl StatefulWidget for AlbumBar<'_> {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let list = List::new(self.list)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .highlight_style(Style::new().reversed())
            .highlight_symbol("|")
            .repeat_highlight_symbol(true);

        list.render(area, buf, state);
    }
}
