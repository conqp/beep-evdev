use beep_evdev::{Beep, Melody, DEFAULT_FILE};
use evdev::Device;

fn main() {
    Device::open(DEFAULT_FILE)
        .unwrap()
        .play(&Melody::default())
        .unwrap()
}
