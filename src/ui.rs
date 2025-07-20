pub mod album_bar;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::{app::App, ui::album_bar::AlbumBar};

impl App {
    /// Renders the user interface widgets.
    pub fn render(&mut self, frame: &mut Frame) {
        let horizontal =
            Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Fill(1)]).split(frame.area());

        self.album_list_state.select_first();

        frame.render_stateful_widget(
            AlbumBar::new(self.source.list_playlists()),
            horizontal[0],
            &mut self.album_list_state,
        );
    }
}
