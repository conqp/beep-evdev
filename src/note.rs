use crate::{DEFAULT_DELAY, DEFAULT_FREQ, DEFAULT_LEN, DEFAULT_REPEATS};
use std::time::Duration;

/// A note of a certain frequency and duration
/// that may be repeated with a delay
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Note {
    frequency: u16,
    length: Duration,
    repeats: u16,
    delay: Duration,
}

impl Note {
    /// Create a new note.
    #[must_use]
    pub const fn new(frequency: u16, length: Duration, repeats: u16, delay: Duration) -> Self {
        Self {
            frequency,
            length,
            repeats,
            delay,
        }
    }

    #[must_use]
    pub const fn frequency(&self) -> u16 {
        self.frequency
    }

    #[must_use]
    pub const fn length(&self) -> Duration {
        self.length
    }

    #[must_use]
    pub const fn repeats(&self) -> u16 {
        self.repeats
    }

    #[must_use]
    pub const fn delay(&self) -> Duration {
        self.delay
    }
}

impl Default for Note {
    fn default() -> Self {
        Self::new(DEFAULT_FREQ, DEFAULT_LEN, DEFAULT_REPEATS, DEFAULT_DELAY)
    }
}

impl From<(u16, u64)> for Note {
    /// Creates a note from a `(frequency, length)` tuple with default repeats and delay
    fn from((frequency, length): (u16, u64)) -> Self {
        Self::from((frequency, Duration::from_millis(length)))
    }
}

impl From<(u16, Duration)> for Note {
    /// Creates a note from a `(frequency, length)` tuple with default repeats and delay
    fn from((frequency, length): (u16, Duration)) -> Self {
        Self::new(frequency, length, DEFAULT_REPEATS, DEFAULT_DELAY)
    }
}
