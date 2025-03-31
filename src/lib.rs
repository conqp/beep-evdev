//! A library using evdev to play notes on the PC speaker.

use std::time::Duration;

pub use beep::Beep;
pub use note::Note;

mod beep;
mod note;

pub const DEFAULT_DELAY: Duration = Duration::from_millis(100);
pub const DEFAULT_FREQ: u16 = 440;
pub const DEFAULT_LEN: Duration = Duration::from_millis(200);
pub const DEFAULT_REPEATS: u16 = 1;
pub const DEFAULT_FILE: &str = "/dev/input/by-path/platform-pcspkr-event-spkr";
