use std::{
    fs::{self},
    io::Error,
    path::PathBuf,
    time::Duration,
};

use anyhow::Context;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    tag::Accessor,
};
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
    /// Used to by tracks to reference playlist
    pub id: usize,
    /// Second half of folder name
    pub title: String,
    /// First half of folder name
    pub artists: String,
    /// Path to folder
    pub path: PathBuf,
    /// Tracks
    tracks: Vec<Track>,
}

#[derive(Clone)]
pub struct Track {
    /// Reference to `Playlist` in source handler
    pub playlist_index: usize,
    /// Path to the file
    pub path: PathBuf,
    /// Track Metadata
    pub metadata: TrackMetadata,
}

#[derive(Clone)]
pub struct TrackMetadata {
    pub number: u32,
    pub title: String,
    pub artists: String,
    pub year: u32,
    pub total_duration: Duration,
    pub bit_rate: u32,
    pub sample_rate: u32,
}

impl SourceHandler {
    pub fn build(path: PathBuf) -> Result<Self, Error> {
        // Use indexes so tracks can be backtraced to playlist
        let mut id = 0;

        let children: Vec<Playlist> = fs::read_dir(&path)?
            .filter_map(|child| child.ok()) // Is able to read
            .filter_map(|child| {
                if child.file_type().ok()?.is_dir() {
                    // Is a directory
                    let playlist =
                        Playlist::build(child.file_name().into_string().unwrap(), child.path(), id);
                    id += 1;

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

    /// Number of tracks in a playlist at index
    pub fn num_tracks_in_playlists(&self, index: usize) -> usize {
        match self.playlists.get(index) {
            Some(v) => v.tracks().len(),
            None => 0,
        }
    }
}

impl Playlist {
    pub fn build(name: String, path: PathBuf, id: usize) -> Result<Self, Error> {
        // Title & Artist(s)
        let mut name_parts = name.split(" - ");
        let artists = name_parts.next().unwrap().trim().to_string();
        let title = name_parts.next().context(name.clone()).expect(
            "Unexpected name format. Desired: [INDEX] [ARTIST] - [TITLE]\nNo Artist / Index found",
        ).trim().to_string();

        let children: Vec<Track> = fs::read_dir(&path)
            .expect("Failed to read playlist")
            .filter_map(|child| child.ok()) // Is able to read
            .filter_map(|child| {
                if child.file_type().ok()?.is_file() {
                    // Is a file
                    Track::try_new(child.path(), id)
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
            id,
            tracks: children,
        })
    }

    /// fetches tracks
    pub fn tracks(&self) -> &Vec<Track> {
        return &self.tracks;
    }

    /// Gets specific track based off number
    pub fn get(&self, number: u32) -> Option<Track> {
        let tracks = self.tracks();

        for i in tracks {
            if i.metadata.number == number {
                return Some(i.clone());
            }
        }

        return None;
    }

    /// Lists out tracks in a playlist to be displayed
    pub fn display(&self) -> Vec<Text> {
        let mut result = Vec::new();

        // Format the names
        for i in self.tracks() {
            result.push(Text::from(format!(
                "{:2} {}",
                i.metadata.number, i.metadata.title,
            )));
        }

        return result;
    }
}

impl Track {
    /// Returns Some if codec is supported, otherwise returns none
    pub fn try_new(path: PathBuf, playlist_index: usize) -> Option<Self> {
        let extension = path.extension()?.to_str()?;
        if AUDIO_EXTENSIONS.contains(&extension) {
            Some(Self {
                path: path.clone(),
                playlist_index,
                metadata: read_track_metadata(path),
            })
        } else {
            None
        }
    }
}

fn read_track_metadata(path: PathBuf) -> TrackMetadata {
    let tagged_file = lofty::read_from_path(path).unwrap();
    let tag = tagged_file.primary_tag().unwrap();
    let properties = tagged_file.properties();

    TrackMetadata {
        number: tag.track().unwrap(),
        title: tag.title().unwrap().to_string(),
        artists: tag.artist().unwrap().to_string(),
        year: tag.year().unwrap(),
        total_duration: properties.duration(),
        bit_rate: properties.overall_bitrate().unwrap(),
        sample_rate: properties.sample_rate().unwrap(),
    }
}
