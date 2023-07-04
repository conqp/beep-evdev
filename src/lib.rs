use evdev::{Device, EventType, InputEvent, SoundType};
use std::thread::sleep;
use std::time::Duration;

pub const DEFAULT_DELAY: u64 = 100;
pub const DEFAULT_FREQ: u16 = 440;
pub const DEFAULT_LEN: u64 = 200;
pub const DEFAULT_REPEATS: u64 = 1;
const FILE: &str = "/dev/input/by-path/platform-pcspkr-event-spkr";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Melody(Vec<Note>);

/// A sequence of notes
impl Melody {
    #[must_use]
    pub fn new(notes: &[Note]) -> Self {
        Self(Vec::from(notes))
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
                sleep(Duration::from_millis(note.length));
                beep(0)?;
            }

            for _ in 1..note.repeats {
                sleep(Duration::from_millis(note.delay));
                beep(note.frequency)?;
                sleep(Duration::from_millis(note.length));
                beep(0)?;
            }
        }

        Ok(())
    }
}

impl From<Vec<Note>> for Melody {
    fn from(notes: Vec<Note>) -> Self {
        Self(notes)
    }
}

impl From<&[Note]> for Melody {
    fn from(notes: &[Note]) -> Self {
        Self::new(notes)
    }
}

/// A note of a certain frequency and duration
/// May be repeated with a delay
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Note {
    frequency: u16,
    length: u64,
    repeats: u64,
    delay: u64,
}

impl Note {
    /// Creates a new note
    /// # Attributes
    /// * frequency - The frequency in Hertz
    /// * length -  The length in milliseconds
    /// * repeats - The amount of repeats
    /// * delay - The delay in seconds when repeating
    #[must_use]
    pub const fn new(frequency: u16, length: u64, repeats: u64, delay: u64) -> Self {
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
    fn from((frequency, length): (u16, u64)) -> Self {
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
