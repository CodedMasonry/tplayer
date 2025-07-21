pub mod list_area;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::{
    app::{App, CurrentList},
    ui::list_area::ListArea,
};

impl App {
    /// Renders the user interface widgets.
    pub fn render(&mut self, frame: &mut Frame) {
        // Required Data
        let current_playlist = self.album_list_state.selected().unwrap();

        // Layout
        let horizontal =
            Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Fill(1)]).split(frame.area());

        // Rendering
        frame.render_stateful_widget(
            ListArea::new(
                self.source.list_playlists(),
                self.current_list == CurrentList::Playlists,
            ),
            horizontal[0],
            &mut self.album_list_state,
        );
        frame.render_stateful_widget(
            ListArea::new(
                self.source.list_songs_from_playlists(current_playlist),
                self.current_list == CurrentList::Songs,
            ),
            horizontal[1],
            &mut self.song_list_state,
        );
    }
}
