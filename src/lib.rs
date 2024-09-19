mod beep;
mod melody;
mod note;

pub use beep::Beep;
pub use melody::Melody;
pub use note::Note;
use std::time::Duration;

pub const DEFAULT_DELAY: Duration = Duration::from_millis(100);
pub const DEFAULT_FREQ: u16 = 440;
pub const DEFAULT_LEN: Duration = Duration::from_millis(200);
pub const DEFAULT_REPEATS: u16 = 1;
pub const DEFAULT_FILE: &str = "/dev/input/by-path/platform-pcspkr-event-spkr";
