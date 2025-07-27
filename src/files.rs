use std::{
    fs::{self},
    io::Error,
    num::NonZeroUsize,
    path::PathBuf,
    sync::{LazyLock, Mutex},
    time::Duration,
};

use anyhow::Context;
use hashbrown::HashMap;
use lofty::{
    file::{AudioFile, TaggedFileExt},
    tag::Accessor,
};
use lru::LruCache;
use ratatui::{
    style::{Style, Stylize},
    text::{Line, Text},
};

/*
 * Globals
 */

/// Supported audio formats
const AUDIO_EXTENSIONS: [&str; 7] = ["aac", "alac", "flac", "mp3", "ogg", "opus", "wav"];
/// Cache of 5 most recent track lists
static TRACK_CACHE: LazyLock<Mutex<LruCache<usize, Vec<Track>>>> =
    LazyLock::new(|| Mutex::new(LruCache::new(NonZeroUsize::new(5).unwrap())));

/*
 * Structs
 */

pub struct SourceHandler {
    /// Path where playlists are located
    pub path: PathBuf,
    /// The playlists
    pub playlists: HashMap<usize, Playlist>,
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

/*
 * Functions
 */

impl SourceHandler {
    pub fn build(path: PathBuf) -> Result<Self, Error> {
        let mut playlists = HashMap::new();
        // Use indexes so tracks can be backtraced to playlist
        let mut id = 0;

        fs::read_dir(&path)?
            .filter_map(|child| child.ok()) // Is able to read
            .filter_map(|child| {
                if child.file_type().ok()?.is_dir() {
                    // Is a directory
                    let playlist =
                        Playlist::build(child.file_name().into_string().unwrap(), child.path(), id);

                    if let Ok(v) = playlist {
                        id += 1;
                        Some(v)
                    } else {
                        None
                    }
                } else {
                    // Is regular file
                    None
                }
            })
            .for_each(|child| {
                playlists.insert(child.id, child);
            });

        Ok(Self {
            path: path.clone(),
            playlists: playlists,
        })
    }

    /// Lists out playlists to be displayed
    pub fn list_playlists(&self) -> Vec<Text> {
        let mut result = Vec::new();

        for (_, playlist) in &self.playlists {
            let title = Line::styled(&playlist.title, Style::new().bold());
            let artists = Line::styled(&playlist.artists, Style::new().dim().italic());

            result.push(Text::from(vec![title, artists]));
        }

        return result;
    }

    /// Number of tracks in a playlist at index
    pub fn num_tracks_in_playlists(&self, id: usize) -> usize {
        match self.playlists.get(&id) {
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

        Ok(Self {
            title,
            artists,
            path,
            id,
        })
    }

    /// fetches tracks
    pub fn tracks<'a>(&self) -> Vec<Track> {
        // If we get a cache hit, return it
        let mut cache_lock = TRACK_CACHE.lock().unwrap();
        if let Some(tracks) = cache_lock.get(&self.id) {
            return tracks.clone();
        }

        // Fetch tracks if no cache hit
        let children: Vec<Track> = fs::read_dir(&self.path)
            .expect("Failed to read playlist")
            .filter_map(|child| child.ok()) // Is able to read
            .filter_map(|child| {
                if child.file_type().ok()?.is_file() {
                    // Is a file
                    Track::try_new(child.path(), self.id)
                } else {
                    // Is dir
                    None
                }
            })
            .collect();

        // Save to cache
        cache_lock.put(self.id, children.clone());

        return children;
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
