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

pub struct SourceHandler {
    pub path: PathBuf,
    pub playlists: Vec<Playlist>,
}

#[derive(Default)]
pub struct Playlist {
    /// Second half of folder name
    pub title: String,
    /// First half of folder name
    pub artists: String,
    /// Path to folder
    pub path: PathBuf,
    /// tracks in the playlist
    pub tracks: Vec<Track>,
}

#[derive(Clone)]
pub struct Track {
    /// Assigned by file (sorting)
    pub number: usize,
    /// Reference to `Playlist` in source handler
    pub playlist_index: usize,
    /// Title of track
    pub title: String,
    /// Artists derived from `Playlist` file name
    pub artists: String,
    /// Path to the file
    pub path: PathBuf,
}

impl SourceHandler {
    pub fn build(path: PathBuf) -> Result<Self, Error> {
        // Use indexes so tracks can be backtraced to playlist
        let mut playlist_index = 0;

        let children: Vec<Playlist> = fs::read_dir(&path)?
            .filter_map(|child| child.ok()) // Is able to read
            .filter_map(|child| {
                if child.file_type().ok()?.is_dir() {
                    // Is a directory
                    let playlist = Playlist::build(
                        child.file_name().into_string().unwrap(),
                        child.path(),
                        playlist_index,
                    );
                    playlist_index += 1;

                    playlist.ok()
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

    /// Lists out playlists to be displayed
    pub fn list_playlists(&self) -> Vec<Text> {
        let mut result = Vec::new();

        for i in &self.playlists {
            let title = Line::styled(&i.title, Style::new().bold());
            let artists = Line::styled(&i.artists, Style::new().dim().italic());

            result.push(Text::from(vec![title, artists]));
        }

        return result;
    }

    /// Lists out tracks in a playlist to be displayed
    pub fn list_tracks_from_playlists(&self, index: usize) -> Vec<Text> {
        let mut result = Vec::new();
        let playlist = self.playlists.get(index);

        // Doesn't exist, so nothing in list
        if let None = playlist {
            return result;
        }

        // Variable wnumberth for index
        let max_width = &playlist
            .unwrap()
            .tracks
            .iter()
            .map(|n| n.number.to_string().len())
            .max()
            .unwrap_or(1);

        // Format the names
        for i in &playlist.unwrap().tracks {
            result.push(Text::from(format!(
                "{:width$} {}",
                i.number,
                i.title,
                width = max_width
            )));
        }

        return result;
    }

    /// Number of tracks in a playlist at index
    pub fn tracks_in_playlists(&self, index: usize) -> usize {
        match self.playlists.get(index) {
            Some(v) => v.tracks.len(),
            None => 0,
        }
    }
}

impl Playlist {
    pub fn build(name: String, path: PathBuf, playlist_index: usize) -> Result<Self, Error> {
        // Title & Artist(s)
        let mut name_parts = name.split(" - ");
        let artists = name_parts.next().unwrap().trim().to_string();
        let title = name_parts.next().context(name.clone()).expect(
            "Unexpected name format. Desired: [INDEX] [ARTIST] - [TITLE]\nNo Artist / Index found",
        ).trim().to_string();

        // tracks
        let children: Vec<Track> = fs::read_dir(&path)?
            .filter_map(|child| child.ok()) // Is able to read
            .filter_map(|child| {
                if child.file_type().ok()?.is_file() {
                    // Is a file
                    Track::try_new(
                        child.file_name().into_string().unwrap(),
                        artists.clone(),
                        child.path(),
                        playlist_index,
                    )
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
            tracks: children,
        })
    }
}

impl Track {
    /// Returns Some if codec is supported, otherwise returns none
    pub fn try_new(
        title: String,
        artists: String,
        path: PathBuf,
        playlist_index: usize,
    ) -> Option<Self> {
        let extension = path.extension()?.to_str()?;
        if AUDIO_EXTENSIONS.contains(&extension) {
            // Parses for number
            let number = title[0..2].parse::<usize>().context(title.clone()).expect(
                "Unexpected name format. Desired: [number] [ARTIST] - [TITLE]\nNo number found",
            );

            // Parse for title
            let offset_extension = title.len() - (extension.len() + 1);
            let title = title[2..offset_extension].trim().to_string();

            Some(Self {
                number,
                title,
                path,
                playlist_index,
                artists,
            })
        } else {
            None
        }
    }
}
