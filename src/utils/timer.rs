use std::time::{Duration, Instant};

#[derive(Debug, Default, Clone, Copy)]
pub struct Timer {
    start: Option<Instant>,
    end: Option<Instant>,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: None,
            end: None,
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
        self.end = None;
    }

    pub fn stop(&mut self) {
        self.end = Some(Instant::now());
    }

    pub fn elapsed(&self) -> Duration {
        match self.start {
            Some(s) => match self.end {
                Some(e) => e.saturating_duration_since(s),
                None => Instant::now().saturating_duration_since(s),
            },
            None => Duration::from_millis(0),
        }
    }

    pub fn elapsed_ms(&self) -> f64 {
        self.elapsed().as_secs_f64() * 1000.0
    }

    pub fn elapsed_ms_3dp(&self) -> String {
        format!("{:.3}", self.elapsed_ms())
    }
}
