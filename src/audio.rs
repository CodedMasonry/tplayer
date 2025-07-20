use std::{fs, path::Path};

use anyhow::Error;
use rodio::{OutputStream, Sink};

pub struct AudioProvider {
    _stream_handle: OutputStream,
    sink: Sink,
}

impl AudioProvider {
    pub fn new() -> Self {
        let stream_handle = rodio::OutputStreamBuilder::open_default_stream()
            .expect("Failed to open default stream");
        let sink = rodio::Sink::connect_new(stream_handle.mixer());

        return Self {
            _stream_handle: stream_handle,
            sink,
        };
    }

    pub fn play_file(&self, path: &Path) -> Result<(), Error> {
        let file = fs::File::open(path)?;
        self.sink.append(rodio::Decoder::try_from(file)?);

        Ok(())
    }

    pub fn next(&self) {
        self.sink.skip_one();
    }

    pub fn sleep_until_end(&self) {
        self.sink.sleep_until_end();
    }
}
