use std::{
    fs::{self},
    io::Error,
    path::PathBuf,
};

use anyhow::Context;
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Text},
};

const AUDIO_EXTENSIONS: [&str; 7] = ["aac", "alac", "flac", "mp3", "ogg", "opus", "wav"];

pub struct SourceProvider {
    pub path: PathBuf,
    pub playlists: Vec<Playlist>,
}

#[derive(Default)]
pub struct Playlist {
    pub title: String,
    pub artists: String,
    pub path: PathBuf,
    pub songs: Vec<Song>,
}

pub struct Song {
    pub index: usize,
    pub title: String,
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
            let artist = Line::styled(&i.title, Style::new().bold());
            let title = Line::styled(&i.artists, Style::new().dim().italic());

            result.push(Text::from(vec![artist, title]));
        }

        return result;
    }

    pub fn list_songs_from_playlists(&self, index: usize) -> Vec<Text> {
        let mut result = Vec::new();
        let playlist = self.playlists.get(index);

        // Doesn't exist, so nothing in list
        if let None = playlist {
            return result;
        }

        for i in &playlist.unwrap().songs {
            result.push(Text::from(i.title.as_str()));
        }

        return result;
    }

    pub fn songs_in_playlists(&self, index: usize) -> usize {
        match self.playlists.get(index) {
            Some(v) => v.songs.len(),
            None => 0,
        }
    }
}

impl Playlist {
    pub fn build(name: String, path: PathBuf) -> Result<Self, Error> {
        // Title & Artist(s)
        let mut name_parts = name.split(" - ");
        let artists = name_parts.next().unwrap().to_string();
        let title = name_parts.next().context(name.clone()).expect(
            "Unexpected name format. Desired: [INDEX] [ARTIST] - [TITLE]\nNo Artist / Index found",
        ).to_string();

        // Songs
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
            title,
            artists,
            path,
            songs: children,
        })
    }
}

impl Song {
    /// Returns Some if codec is supported, otherwise returns none
    pub fn try_new(title: String, path: PathBuf) -> Option<Self> {
        let extension = path.extension()?.to_str()?;
        if AUDIO_EXTENSIONS.contains(&extension) {
            // Parses for index
            let index = title[0..2].parse::<usize>().context(title.clone()).expect(
                "Unexpected name format. Desired: [INDEX] [ARTIST] - [TITLE]\nNo Index found",
            );
            // Parse for title
            let offset_extension = title.len() - (extension.len() + 1);
            let title = title[2..offset_extension].to_string();

            Some(Self { index, title, path })
        } else {
            None
        }
    }
}
