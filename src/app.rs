use crate::{
    audio::AudioHandler,
    event::{AppEvent, Event, EventHandler},
    files::{Playlist, SourceHandler, Track},
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::ListState,
};

#[derive(PartialEq)]
pub enum CurrentList {
    Playlists,
    Tracks,
}

/// Application.
pub struct App {
    /// Quit
    pub quit: bool,

    /// Handlers & Handlers
    pub source: SourceHandler,
    pub audio: AudioHandler,
    pub events: EventHandler,

    /// State Handling
    pub current_list: CurrentList,
    pub album_list_state: ListState,
    pub track_list_state: ListState,
}

impl App {
    /*
     * Primary Functions
     */

    /// Constructs a new instance of [`App`].
    pub fn new(source: SourceHandler, audio: AudioHandler) -> Self {
        // Init Lists
        let mut album_list_state = ListState::default();
        album_list_state.select_first();

        let mut track_list_state = ListState::default();
        track_list_state.select_first();

        Self {
            quit: false,

            source,
            audio,
            events: EventHandler::new(),

            current_list: CurrentList::Playlists,
            album_list_state,
            track_list_state,
        }
    }

    /// Run the application's main loop.
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while !self.quit {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /*
     * Handlers
     */

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            // Tick
            Event::Tick => self.tick(),
            // Terminal
            Event::Crossterm(event) => match event {
                ratatui::crossterm::event::Event::Key(key_event) => {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            },
            // Custom Events
            Event::App(app_event) => match app_event {
                // Quit
                AppEvent::Quit => self.quit(),

                // List
                AppEvent::ListUp => self.handle_list_events(AppEvent::ListUp),
                AppEvent::ListDown => self.handle_list_events(AppEvent::ListDown),
                AppEvent::ListQueue => self.handle_list_events(AppEvent::ListQueue),
                AppEvent::ListSelect => self.handle_list_events(AppEvent::ListSelect),
                AppEvent::ListBack => self.handle_list_events(AppEvent::ListBack),

                // Playback
                AppEvent::PlayTogle => self.audio.toggle_playing(),
                AppEvent::PlayNext => self.audio.next(),
                AppEvent::PlayPrevious => self.previous(),
                AppEvent::PlaySeekForward => self.audio.seek_forward(),
                AppEvent::PlaySeekBack => self.audio.seek_back(),

                // Volume
                AppEvent::VolumeUp => self.audio.raise_volume(0.05),
                AppEvent::VolumeDown => self.audio.lower_volume(0.05),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            // Quit
            KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }

            // Volume
            KeyCode::Up if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::VolumeUp);
            }
            KeyCode::Down if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::VolumeDown);
            }

            // List
            KeyCode::Up => self.events.send(AppEvent::ListUp),
            KeyCode::Down => self.events.send(AppEvent::ListDown),
            KeyCode::Tab => self.events.send(AppEvent::ListQueue),
            KeyCode::Enter => self.events.send(AppEvent::ListSelect),
            KeyCode::Esc => self.events.send(AppEvent::ListBack),

            // Playback
            KeyCode::Char(' ') => self.events.send(AppEvent::PlayTogle),
            KeyCode::Right if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::PlayNext);
            }
            KeyCode::Left if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::PlayPrevious);
            }
            KeyCode::Right => self.events.send(AppEvent::PlaySeekForward),
            KeyCode::Left => self.events.send(AppEvent::PlaySeekBack),
            _ => {}
        }
        Ok(())
    }

    /// Handles events related to [`CurrentList`].
    pub fn handle_list_events(&mut self, event: AppEvent) {
        // Get context
        let (current_list, list_length) = match self.current_list {
            CurrentList::Playlists => {
                // Cleanup track list to make UI transition look cleaner
                self.track_list_state.select_first();
                // list & length
                (&mut self.album_list_state, self.source.playlists.len())
            }
            CurrentList::Tracks => (
                // List
                &mut self.track_list_state,
                // Length
                self.source
                    .tracks_in_playlists(self.album_list_state.selected().unwrap()),
            ),
        };

        // Handle in context
        match event {
            // Up
            AppEvent::ListUp => match current_list.selected().unwrap() <= 0 {
                true => current_list.select(Some(list_length - 1)),
                false => current_list.select_previous(),
            },
            // Down
            AppEvent::ListDown => match current_list.selected().unwrap() >= list_length - 1 {
                true => current_list.select(Some(0)),
                false => current_list.select_next(),
            },
            // Queue
            AppEvent::ListQueue => match self.current_list {
                // Only works on tracks, can't queue playlist
                CurrentList::Playlists => {}
                CurrentList::Tracks => {
                    let track = self.selected_track().clone();
                    self.audio
                        .queue_track(&track)
                        .expect("Failed to play track")
                }
            },
            // Select
            AppEvent::ListSelect => match self.current_list {
                CurrentList::Playlists => {
                    self.current_list = CurrentList::Tracks;
                }
                CurrentList::Tracks => {
                    let track = self.selected_track().clone();
                    self.audio
                        .play_track(&track, true)
                        .expect("Failed to play track")
                }
            },
            // Back
            AppEvent::ListBack => self.current_list = CurrentList::Playlists,
            // Only want list events
            _ => {}
        };
    }

    /// Handles trying to play previous song
    fn previous(&mut self) {
        if let Some(primary_track) = &self.audio.primary_track {
            if primary_track.number > 1 {
                self.audio
                    .play_track(
                        &self.previous_in_playlist(&primary_track).unwrap().clone(),
                        true,
                    )
                    .unwrap();
            }
        }
    }

    /*
     * Fetchers
     */

    pub fn selected_playlist(&self) -> &Playlist {
        self.source
            .playlists
            .get(self.album_list_state.selected().unwrap())
            .unwrap()
    }

    pub fn selected_track(&self) -> &Track {
        let playlist = self.selected_playlist();
        playlist
            .tracks
            .get(self.track_list_state.selected().unwrap())
            .unwrap()
    }

    pub fn track_to_playlist(&self, track: &Track) -> &Playlist {
        self.source.playlists.get(track.playlist_index).unwrap()
    }

    /*
     * Audio functions that require higher context
     */

    pub fn next_in_playlist(&self, track: &Track) -> Option<&Track> {
        let playlist = self.track_to_playlist(track);
        // The number starts at 1 instead of 0 (how albums number), so just use it as the index for next
        playlist.tracks.get(track.number)
    }

    pub fn previous_in_playlist(&self, track: &Track) -> Option<&Track> {
        let playlist = self.track_to_playlist(track);
        // The number starts at 1 instead of 0 (how albums number), so subtract 2 to get previous
        playlist.tracks.get(track.number - 2)
    }

    /*
     *  Tick
     */

    /// Handles the tick event of the terminal
    pub fn tick(&mut self) {
        self.tick_audio();
    }

    pub fn tick_audio(&mut self) {
        let primary_track = &self.audio.primary_track;

        // track has finished
        if self.audio.sink.empty() {
            // There is a queue
            if self.audio.queue.len() > 0 {
                let next = &self.audio.pop_queue().unwrap();
                self.audio.play_track(next, false).unwrap();
            }
            // Play next in playlist if nothing in queue
            else if primary_track.is_some() {
                let primary_track = primary_track.clone().unwrap();
                if primary_track.number < self.track_to_playlist(&primary_track).tracks.len() {
                    self.audio
                        .play_track(
                            &self.next_in_playlist(&primary_track).unwrap().clone(),
                            true,
                        )
                        .unwrap();
                }
            }
        }

        // Tick track progress
        if !self.audio.sink.is_paused() && self.audio.current_track.is_some() {
            let current_track = self.audio.current_track.as_mut().unwrap();
            current_track.elapsed_duration = self.audio.sink.get_pos();
        }
    }

    /*
     * Quit
     */

    pub fn quit(&mut self) {
        self.quit = true;
    }
}
