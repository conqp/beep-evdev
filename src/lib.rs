//! A library using evdev to play notes on the PC speaker.

use std::time::Duration;

pub use beep::Beep;
pub use note::Note;

mod beep;
mod note;

/// Default duration of a note.
pub const DEFAULT_DELAY: Duration = Duration::from_millis(100);

/// Default frequency of a note in Hertz.
pub const DEFAULT_FREQ: u16 = 440;

/// Default length of a note.
pub const DEFAULT_LEN: Duration = Duration::from_millis(200);

/// Default repeats of a note.
pub const DEFAULT_REPEATS: u16 = 1;

/// Default pcspkr device path.
pub const DEFAULT_FILE: &str = "/dev/input/by-path/platform-pcspkr-event-spkr";
