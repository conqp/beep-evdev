use crate::{Melody, Note, FILE};
use evdev::{Device, EventType, InputEvent, SoundType};
use std::thread::sleep;

pub struct Pcspkr {
    device: Device,
}

impl Pcspkr {
    #[must_use]
    pub const fn new(device: Device) -> Self {
        Self { device }
    }

    /// Beeps the PC speaker at the given frequency.
    ///
    /// # Examples
    /// ```
    /// use beep_evdev::Pcspkr;
    /// use std::{thread, time};
    ///
    /// let mut pcspkr = Pcspkr::default();
    /// pcspkr.beep(440).expect("could not beep");
    /// thread::sleep(time::Duration::from_millis(500));
    /// pcspkr.beep(880).expect("could not beep");
    /// thread::sleep(time::Duration::from_millis(500));
    /// pcspkr.beep(0).expect("could not beep");
    /// ```
    ///
    /// # Errors
    /// Returns [`std::io::Error`] on I/O errors
    pub fn beep(&mut self, hertz: u16) -> std::io::Result<()> {
        self.device.send_events(&[InputEvent::new(
            EventType::SOUND,
            SoundType::SND_TONE.0,
            i32::from(hertz),
        )])
    }

    /// Plays a note.
    ///
    /// # Examples
    /// ```
    /// use beep_evdev::{Note, Pcspkr};
    ///
    /// Pcspkr::default().note(&Note::default()).expect("could not play melody :-(");
    /// ```
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on I/O errors
    pub fn note(&mut self, note: &Note) -> Result<(), std::io::Error> {
        if note.repeats() > 0 {
            self.beep(note.frequency())?;
            sleep(note.length());
            self.beep(0)?;
        }

        for _ in 1..note.repeats() {
            sleep(note.delay());
            self.beep(note.frequency())?;
            sleep(note.length());
            self.beep(0)?;
        }

        Ok(())
    }

    /// Plays a melody.
    ///
    /// # Examples
    /// ```
    /// use beep_evdev::{Melody, Pcspkr};
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
    /// Pcspkr::default().play(&melody).expect("could not play melody :-(");
    /// ```
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on I/O errors
    pub fn play(&mut self, melody: &Melody) -> Result<(), std::io::Error> {
        for note in melody.as_ref() {
            self.note(note)?;
        }

        Ok(())
    }
}

impl Default for Pcspkr {
    fn default() -> Self {
        Self::new(Device::open(FILE).expect("failed to open device"))
    }
}
