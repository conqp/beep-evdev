use beep_evdev::{Melody, Pcspkr};

fn main() {
    Pcspkr::default().play(&Melody::default()).unwrap()
}
