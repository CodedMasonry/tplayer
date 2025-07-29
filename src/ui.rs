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

        // Split lists & status
        let vertical_main =
            Layout::vertical([Constraint::Fill(1), Constraint::Length(4)]).split(frame.area());
        // Split Album & Track
        let horizontal_lists = Layout::horizontal([Constraint::Ratio(1, 3), Constraint::Fill(1)])
            .split(vertical_main[0]);
        // Split status into Playing, Progress, Status
        let horizontal_status = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(3),
            Constraint::Fill(1),
        ])
        .split(vertical_main[1]);

        // Album List
        frame.render_stateful_widget(
            ListArea::new(
                self.source.display_playlists(),
                self.current_list == CurrentList::Playlists,
            ),
            horizontal_lists[0],
            &mut self.album_list_state,
        );
        // Track List
        frame.render_stateful_widget(
            ListArea::new(
                self.source
                    .playlists
                    .get(&current_playlist)
                    .unwrap()
                    .display(),
                self.current_list == CurrentList::Tracks,
            ),
            horizontal_lists[1],
            &mut self.track_list_state,
        );

        // Currently Playing
        frame.render_stateful_widget(
            CurrentPlaying::new(),
            horizontal_status[0],
            &mut self.audio.current_track,
        );
        // Progress
        frame.render_stateful_widget(
            Progress::new(),
            horizontal_status[1],
            &mut (
                self.audio.current_track.as_mut(),
                self.audio.sink.is_paused(),
            ),
        );
        // Status
        frame.render_stateful_widget(
            Status::new(),
            horizontal_status[2],
            &mut StatusInfo {
                volume: self.audio.volume(),
                queue_len: self.audio.queue.len(),
            },
        );
    }
}
