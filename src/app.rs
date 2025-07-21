use crate::{
    audio::AudioProvider,
    event::{AppEvent, Event, EventHandler},
    files::{Playlist, Song, SourceProvider},
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::ListState,
};

#[derive(PartialEq)]
pub enum CurrentList {
    Playlists,
    Songs,
}

/// Application.
pub struct App {
    /// Quit
    pub quit: bool,

    /// Providers & Handlers
    pub source: SourceProvider,
    pub audio: AudioProvider,
    pub events: EventHandler,

    /// State Handling
    pub current_list: CurrentList,
    pub album_list_state: ListState,
    pub song_list_state: ListState,
}

impl App {
    /*
     * Primary Functions
     */

    /// Constructs a new instance of [`App`].
    pub fn new(source: SourceProvider, audio: AudioProvider) -> Self {
        // Init Lists
        let mut album_list_state = ListState::default();
        album_list_state.select_first();

        let mut song_list_state = ListState::default();
        song_list_state.select_first();

        Self {
            quit: false,

            source,
            audio,
            events: EventHandler::new(),

            current_list: CurrentList::Playlists,
            album_list_state,
            song_list_state,
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
                AppEvent::ListSelect => self.handle_list_events(AppEvent::ListSelect),
                AppEvent::ListBack => self.handle_list_events(AppEvent::ListBack),

                // Playback
                AppEvent::PlayNext => todo!(),
                AppEvent::PlayPrevious => todo!(),
                AppEvent::PlaySeekForward => todo!(),
                AppEvent::PlaySeekBack => todo!(),
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

            // List
            KeyCode::Up => self.events.send(AppEvent::ListUp),
            KeyCode::Down => self.events.send(AppEvent::ListDown),
            KeyCode::Enter => self.events.send(AppEvent::ListSelect),
            KeyCode::Esc => self.events.send(AppEvent::ListBack),
            _ => {}
        }
        Ok(())
    }

    /// Handles events related to [`CurrentList`].
    pub fn handle_list_events(&mut self, event: AppEvent) {
        // Get context
        let (current_list, list_length) = match self.current_list {
            CurrentList::Playlists => {
                // Cleanup song list to make UI transition look cleaner
                self.song_list_state.select_first();
                // list & length
                (&mut self.album_list_state, self.source.playlists.len())
            }
            CurrentList::Songs => (
                // List
                &mut self.song_list_state,
                // Length
                self.source
                    .songs_in_playlists(self.album_list_state.selected().unwrap()),
            ),
        };

        // Handle in context
        match event {
            AppEvent::ListUp => match current_list.selected().unwrap() <= 0 {
                true => current_list.select(Some(list_length - 1)),
                false => current_list.select_previous(),
            },
            AppEvent::ListDown => match current_list.selected().unwrap() >= list_length - 1 {
                true => current_list.select(Some(0)),
                false => current_list.select_next(),
            },
            AppEvent::ListSelect => match self.current_list {
                CurrentList::Playlists => {
                    self.current_list = CurrentList::Songs;
                }
                CurrentList::Songs => self
                    .audio
                    .play_song(self.selected_song())
                    .expect("Failed to play song"),
            },
            AppEvent::ListBack => self.current_list = CurrentList::Playlists,

            _ => {}
        };
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

    pub fn selected_song(&self) -> &Song {
        let playlist = self
            .source
            .playlists
            .get(self.album_list_state.selected().unwrap())
            .unwrap();
        playlist
            .songs
            .get(self.song_list_state.selected().unwrap())
            .unwrap()
    }

    /*
     *  Tick
     */

    /// Handles the tick event of the terminal
    pub fn tick(&self) {}

    /*
     * Quit
     */

    pub fn quit(&mut self) {
        self.quit = true;
    }
}
