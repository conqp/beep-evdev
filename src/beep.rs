use std::io::Result;
use std::thread::sleep;

use evdev::{Device, EventType, InputEvent, SoundCode};

use crate::Note;

/// Allows to beep the PC speaker.
pub trait Beep {
    /// Beep the PC speaker at the given frequency.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if beeping the PC speaker fails.
    ///
    /// # Examples
    /// ```
    /// use beep_evdev::{Beep, DEFAULT_FILE};
    /// use std::{thread, time};
    /// use evdev::Device;
    ///
    /// let mut pcspkr = Device::open(DEFAULT_FILE).unwrap();
    /// pcspkr.beep(440).expect("could not beep");
    /// thread::sleep(time::Duration::from_millis(500));
    /// pcspkr.beep(880).expect("could not beep");
    /// thread::sleep(time::Duration::from_millis(500));
    /// pcspkr.beep(0).expect("could not beep");
    fn beep(&mut self, hertz: u16) -> Result<()>;

    /// Play the given note on the PC speaker.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if beeping the PC speaker fails.
    ///
    /// # Examples
    /// ```
    /// use evdev::Device;
    /// use beep_evdev::{Note, Beep, DEFAULT_FILE};
    ///
    /// let mut pcspkr = Device::open(DEFAULT_FILE)
    ///     .unwrap()
    ///     .note(&Note::default())
    ///     .expect("could not play melody :-(");
    /// ```
    fn note(&mut self, note: &Note) -> Result<()> {
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

    /// Play the given melody on the PC speaker.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if beeping the PC speaker fails.
    ///
    /// # Examples
    /// ```
    /// use evdev::Device;
    /// use beep_evdev::{Beep, Note, DEFAULT_FILE};
    ///
    /// let melody = vec![
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
    /// ];
    ///
    /// Device::open(DEFAULT_FILE)
    ///     .expect("could not open pcspkr device")
    ///     .play(&melody)
    ///     .expect("could not play melody :-(");
    /// ```
    fn play<T>(&mut self, melody: T) -> Result<()>
    where
        T: AsRef<[Note]>,
    {
        for note in melody.as_ref() {
            self.note(note)?;
        }

        Ok(())
    }
}

impl Beep for Device {
    fn beep(&mut self, hertz: u16) -> Result<()> {
        self.send_events(&[InputEvent::new(
            EventType::SOUND.0,
            SoundCode::SND_TONE.0,
            i32::from(hertz),
        )])
    }
}
