use crate::{
    audio::AudioProvider,
    event::{AppEvent, Event, EventHandler},
    files::SourceProvider,
};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    widgets::ListState,
};

/// Application.
pub struct App {
    // Is Running
    pub running: bool,
    // File management
    pub source: SourceProvider,
    // Audio interface
    pub audio: AudioProvider,
    /// Event handler
    pub events: EventHandler,

    pub album_list_state: ListState,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(source: SourceProvider, audio: AudioProvider) -> Self {
        Self {
            running: true,
            source,
            audio,
            events: EventHandler::new(),

            album_list_state: ListState::default(),
        }
    }

    /// Run the application's main loop.
    pub fn run(&mut self, mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn handle_events(&mut self) -> color_eyre::Result<()> {
        match self.events.next()? {
            Event::Tick => self.tick(),
            Event::Crossterm(event) => match event {
                ratatui::crossterm::event::Event::Key(key_event) => {
                    self.handle_key_event(key_event)?
                }
                _ => {}
            },
            Event::App(app_event) => match app_event {
                AppEvent::Quit => self.quit(),
            },
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> color_eyre::Result<()> {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
            KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                self.events.send(AppEvent::Quit)
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles the tick event of the terminal
    pub fn tick(&self) {}

    /// Set running to false to quit the application
    pub fn quit(&mut self) {
        self.running = false;
    }
}
