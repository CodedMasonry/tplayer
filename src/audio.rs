use std::{
    fs::{self},
    time::Duration,
};

use color_eyre::eyre::Error;
use rodio::{OutputStream, Sink, Source};

use crate::{config::Config, files::Track};

pub struct AudioHandler {
    /// Player
    _stream_handle: OutputStream,
    pub sink: Sink,

    // The most recent forced played track
    pub primary_track: Option<Track>,
    // Current track
    pub current_track: Option<CurrentTrack>,

    /// Queue
    pub queue: Vec<Track>,
}

pub struct CurrentTrack {
    pub track: Track,
    pub elapsed_duration: Duration,
    pub total_duration: Duration,
}

impl AudioHandler {
    pub fn new() -> Self {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("Failed to open default stream");
        let sink = rodio::Sink::connect_new(stream_handle.mixer());

        sink.pause();

        return Self {
            _stream_handle: stream_handle,
            sink,

            primary_track: None,
            current_track: None,

            queue: Vec::new(),
        };
    }

    /*
     * Tracks & Queue
     */

    /// Forced played tracks
    pub fn play_track(&mut self, track: &Track, set_primary: bool) -> Result<(), Error> {
        let file = fs::File::open(&track.path)?;
        let decoder = rodio::Decoder::try_from(file)?;
        let total_duration = decoder.total_duration().unwrap();

        // Clean up sink so it plays immediately
        self.sink.clear();
        self.sink.append(decoder);
        self.sink.play();

        // Allows rest of album to auto play
        if set_primary {
            self.primary_track = Some(track.clone());
        }

        // Save current track
        self.current_track = Some(CurrentTrack {
            track: track.clone(),
            elapsed_duration: Duration::default(),
            total_duration: total_duration,
        });

        Ok(())
    }

    /// tracks intentionally added to queue OR automatically added
    pub fn queue_track(&mut self, track: &Track) -> Result<(), Error> {
        self.queue.push(track.clone());
        Ok(())
    }

    pub fn pop_queue(&mut self) -> Option<Track> {
        self.queue.pop()
    }

    /*
     * Playback
     */

    pub fn toggle_playing(&self) {
        if self.sink.is_paused() {
            self.sink.play();
        } else {
            self.sink.pause();
        }
    }

    pub fn next(&self) {
        self.sink.clear();
    }

    pub fn seek_forward(&self) {
        if self.current_track.is_none() {
            return;
        }

        // If seeking past the end
        let current = self.current_track.as_ref().unwrap();
        let adjusted_elapsed = current.elapsed_duration.checked_sub(Duration::from_secs(5));
        if adjusted_elapsed.is_some() && adjusted_elapsed.unwrap() > current.total_duration {
            return;
        }

        let time = self.sink.get_pos() + Duration::from_secs(5);
        self.sink.try_seek(time).unwrap()
    }

    pub fn seek_back(&self) {
        // If seeking before song starts
        if self.current_track.is_none() || self.sink.get_pos().as_secs() < 5 {
            return;
        }

        let time = self.sink.get_pos() - Duration::from_secs(5);
        self.sink.try_seek(time).unwrap()
    }

    /*
     * Volume
     */
    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }

    pub fn lower_volume(&self, amount: f32, config: &mut Config) {
        let volume = self.volume();

        if volume - amount <= 0.0 {
            self.sink.set_volume(0.0);
        } else {
            self.sink.set_volume(round_vol(volume - amount))
        }

        // Save to config
        config.set_volume(self.volume());
    }

    pub fn raise_volume(&self, amount: f32, config: &mut Config) {
        let volume = self.volume();

        if volume + amount >= 1.0 {
            self.sink.set_volume(1.0);
        } else {
            self.sink.set_volume(round_vol(volume + amount))
        }

        // Save to config
        config.set_volume(self.volume());
    }
}

fn round_vol(input: f32) -> f32 {
    (input * 100.0).round() / 100.0
}
