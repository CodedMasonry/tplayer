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
}
