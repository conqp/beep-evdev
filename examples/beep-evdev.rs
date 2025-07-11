//! Beep the default PC speaker.

use beep_evdev::{Beep, Note, DEFAULT_FILE};
use evdev::Device;

fn main() {
    Device::open(DEFAULT_FILE)
        .unwrap()
        .play([Note::default()])
        .unwrap();
}
