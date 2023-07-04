use evdev::{Device, EventType, InputEvent, SoundType};

const FILE: &str = "/dev/input/by-path/platform-pcspkr-event-spkr";

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
