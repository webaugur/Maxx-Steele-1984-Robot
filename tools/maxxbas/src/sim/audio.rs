//! Shared rodio output for speech and music — one device, multiple mixer sinks.

use rodio::{OutputStream, OutputStreamBuilder, Sink};

pub struct AudioOutput {
    stream: Option<OutputStream>,
}

impl AudioOutput {
    pub fn new() -> Self {
        Self { stream: None }
    }

    pub fn warm(&mut self) {
        self.ensure();
    }

    pub fn available(&self) -> bool {
        self.stream.is_some()
    }

    pub fn ensure(&mut self) {
        if self.stream.is_none() {
            match OutputStreamBuilder::open_default_stream() {
                Ok(stream) => self.stream = Some(stream),
                Err(e) => eprintln!("audio: open output: {e}"),
            }
        }
    }

    pub fn open_sink(&mut self) -> Option<Sink> {
        self.ensure();
        self.stream
            .as_ref()
            .map(|stream| Sink::connect_new(stream.mixer()))
    }
}

impl Default for AudioOutput {
    fn default() -> Self {
        Self::new()
    }
}