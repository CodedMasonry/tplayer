pub mod current_playing;
pub mod list_area;
pub mod progress;
pub mod status;

use ratatui::{
    Frame,
    layout::{Constraint, Layout},
};

use crate::{
    app::{App, CurrentList},
    ui::{
        current_playing::CurrentPlaying,
        list_area::ListArea,
        progress::Progress,
        status::{Status, StatusInfo},
    },
};

impl App {
    /// Renders the user interface widgets.
    pub fn render(&mut self, frame: &mut Frame) {
        // Required Data
        let current_playlist = self.album_list_state.selected().unwrap();

        /* Layout:
         * ┌─────────┐┌────────────────┐
         * │Playlists││      tracks     │
         * │         ││                │
         * │         ││                │
         * │         ││                │
         * └─────────┘└────────────────┘
         * │         Playing           │
         * └───────────────────────────┘
         */
        let vertical =
            Layout::vertical([Constraint::Fill(1), Constraint::Max(4)]).split(frame.area());
        let horizontal_top =
            Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Fill(1)]).split(vertical[0]);
        let horizontal_bottom = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(2),
            Constraint::Fill(1),
        ])
        .split(vertical[1]);

        // Render Top
        frame.render_stateful_widget(
            ListArea::new(
                self.source.display_playlists(),
                self.current_list == CurrentList::Playlists,
            ),
            horizontal_top[0],
            &mut self.album_list_state,
        );
        frame.render_stateful_widget(
            ListArea::new(
                self.source
                    .playlists
                    .get(&current_playlist)
                    .unwrap()
                    .display(),
                self.current_list == CurrentList::Tracks,
            ),
            horizontal_top[1],
            &mut self.track_list_state,
        );

        // Render Bottom
        frame.render_stateful_widget(
            CurrentPlaying::new(),
            horizontal_bottom[0],
            &mut self.audio.current_track,
        );
        frame.render_stateful_widget(
            Progress::new(),
            horizontal_bottom[1],
            &mut (
                self.audio.current_track.as_mut(),
                self.audio.sink.is_paused(),
            ),
        );
        frame.render_stateful_widget(
            Status::new(),
            horizontal_bottom[2],
            &mut StatusInfo {
                volume: self.audio.volume(),
                queue_len: self.audio.queue.len(),
            },
        );
    }
}
