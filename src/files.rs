use std::{
    fs::{self},
    io::Error,
    path::PathBuf,
};

use color_eyre::owo_colors::OwoColorize;
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Text},
};

const AUDIO_EXTENSIONS: [&str; 7] = ["aac", "alac", "flac", "mp3", "ogg", "opus", "wav"];

pub struct SourceProvider {
    pub path: PathBuf,
    pub playlists: Vec<Playlist>,
}

pub struct Playlist {
    pub name: String,
    pub path: PathBuf,
    pub songs: Vec<Song>,
}

pub struct Song {
    pub name: String,
    pub path: PathBuf,
}

impl SourceProvider {
    pub fn build(path: PathBuf) -> Result<Self, Error> {
        let children: Vec<Playlist> = fs::read_dir(&path)?
            .filter_map(|child| child.ok()) // Is able to read
            .filter_map(|child| {
                if child.file_type().ok()?.is_dir() {
                    // Is a directory
                    Playlist::build(child.file_name().into_string().unwrap(), child.path()).ok()
                } else {
                    // Is regular file
                    None
                }
            })
            .collect();

        Ok(Self {
            path: path.clone(),
            playlists: children,
        })
    }

    pub fn list_playlists(&self) -> Vec<Text> {
        let mut result = Vec::new();

        for i in &self.playlists {
            // Split the title so they can be individually styled
            let mut parts: Vec<&str> = i.name.split(" - ").collect();

            let artist = Line::styled(parts.pop().unwrap(), Style::new().bold());
            let title = Line::styled(
                parts
                    .pop()
                    .expect("Unexpected name format.\nDesired: [ARTIST] - [TITLE]"),
                Style::new().dim().italic(),
            );

            result.push(Text::from(vec![artist, title]));
        }

        return result;
    }
}

impl Playlist {
    pub fn build(name: String, path: PathBuf) -> Result<Self, Error> {
        let children: Vec<Song> = fs::read_dir(&path)?
            .filter_map(|child| child.ok()) // Is able to read
            .filter_map(|child| {
                if child.file_type().ok()?.is_file() {
                    // Is a file
                    Song::try_new(child.file_name().into_string().unwrap(), child.path())
                } else {
                    // Is dir
                    None
                }
            })
            .collect();

        Ok(Self {
            name,
            path,
            songs: children,
        })
    }
}

impl Song {
    /// Returns Some if codec is supported, otherwise returns none
    pub fn try_new(name: String, path: PathBuf) -> Option<Self> {
        let extension = path.extension()?.to_str()?;
        if AUDIO_EXTENSIONS.contains(&extension) {
            Some(Self { name, path })
        } else {
            None
        }
    }
}
