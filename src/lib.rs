use evdev::{Device, EventType, InputEvent, SoundType};
use std::thread::sleep;
use std::time::Duration;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const DEFAULT_DELAY: Duration = Duration::from_millis(100);
pub const DEFAULT_FREQ: u16 = 440;
pub const DEFAULT_LEN: Duration = Duration::from_millis(200);
pub const DEFAULT_REPEATS: u16 = 1;
const FILE: &str = "/dev/input/by-path/platform-pcspkr-event-spkr";

/// A sequence of notes
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Melody(Vec<Note>);

impl Melody {
    #[must_use]
    pub fn new(notes: Vec<Note>) -> Self {
        Self(notes)
    }

    /// Plays the melody
    ///
    /// # Examples
    /// ```
    /// use beep_evdev::Melody;
    ///
    /// let melody: Melody = vec![
    ///     (659, 120).into(),
    ///     (622, 120).into(),
    ///     (659, 120).into(),
    ///     (622, 120).into(),
    ///     (659, 120).into(),
    ///     (94, 120).into(),
    ///     (587, 120).into(),
    ///     (523, 120).into(),
    ///     (440, 120).into(),
    ///     (262, 120).into(),
    ///     (330, 120).into(),
    ///     (440, 120).into(),
    ///     (494, 120).into(),
    ///     (330, 120).into(),
    ///     (415, 120).into(),
    ///     (494, 120).into(),
    ///     (523, 120).into(),
    ///     (330, 120).into(),
    ///     (659, 120).into(),
    ///     (622, 120).into(),
    ///     (659, 120).into(),
    ///     (622, 120).into(),
    ///     (659, 120).into(),
    ///     (494, 120).into(),
    ///     (587, 120).into(),
    ///     (523, 120).into(),
    ///     (440, 120).into(),
    ///     (262, 120).into(),
    ///     (330, 120).into(),
    ///     (440, 120).into(),
    ///     (494, 120).into(),
    ///     (330, 120).into(),
    ///     (523, 120).into(),
    ///     (494, 120).into(),
    ///     (440, 120).into(),
    /// ]
    /// .into();
    /// melody.play().expect("could not play melody :-(");
    /// ```
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on I/O errors
    pub fn play(&self) -> Result<(), std::io::Error> {
        for note in &self.0 {
            if note.repeats > 0 {
                beep(note.frequency)?;
                sleep(note.length);
                beep(0)?;
            }

            for _ in 1..note.repeats {
                sleep(note.delay);
                beep(note.frequency)?;
                sleep(note.length);
                beep(0)?;
            }
        }

        Ok(())
    }
}

impl Default for Melody {
    fn default() -> Self {
        Self::new(vec![Note::default()])
    }
}

impl From<Vec<Note>> for Melody {
    fn from(notes: Vec<Note>) -> Self {
        Self::new(notes)
    }
}

impl From<&[Note]> for Melody {
    fn from(notes: &[Note]) -> Self {
        Self::new(Vec::from(notes))
    }
}

/// A note of a certain frequency and duration
/// that may be repeated with a delay
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Note {
    frequency: u16,
    length: Duration,
    repeats: u16,
    delay: Duration,
}

impl Note {
    /// Creates a new note
    /// # Attributes
    /// * frequency - The frequency in Hertz
    /// * length -  The length in milliseconds
    /// * repeats - The amount of repeats
    /// * delay - The delay in seconds when repeating
    #[must_use]
    pub const fn new(frequency: u16, length: Duration, repeats: u16, delay: Duration) -> Self {
        Self {
            frequency,
            length,
            repeats,
            delay,
        }
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

/// Beeps the PC speaker at the given frequency
///
/// # Examples
/// ```
/// use beep_evdev::beep;
/// use std::{thread, time};
///
/// beep(440).expect("could not beep");
/// thread::sleep(time::Duration::from_millis(500));
/// beep(880).expect("could not beep");
/// thread::sleep(time::Duration::from_millis(500));
/// beep(0).expect("could not beep");
/// ```
///
/// # Errors
/// Returns [`std::io::Error`] on I/O errors
pub fn beep(hertz: u16) -> std::io::Result<()> {
    Device::open(FILE)?.send_events(&[InputEvent::new(
        EventType::SOUND,
        SoundType::SND_TONE.0,
        i32::from(hertz),
    )])
}
