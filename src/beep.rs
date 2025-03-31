use std::io::Result;
use std::thread::sleep;

use evdev::{Device, EventType, InputEvent, SoundCode};

use crate::Note;

pub trait Beep {
    /// Beep the pcspkr at the given frequency.
    fn beep(&mut self, hertz: u16) -> Result<()>;

    /// Play the given note on the pcspkr.
    fn note(&mut self, note: &Note) -> Result<()>;

    /// Play the given melody on the pcspkr.
    fn play<T>(&mut self, melody: T) -> Result<()>
    where
        T: AsRef<[Note]>;
}

impl Beep for Device {
    /// Beeps the PC speaker at the given frequency.
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
    /// ```
    ///
    /// # Errors
    /// Returns [`std::io::Error`] on I/O errors.
    fn beep(&mut self, hertz: u16) -> Result<()> {
        self.send_events(&[InputEvent::new(
            EventType::SOUND.0,
            SoundCode::SND_TONE.0,
            i32::from(hertz),
        )])
    }

    /// Plays a note.
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
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on I/O errors.
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

    /// Plays a melody.
    ///
    /// # Examples
    /// ```
    /// use evdev::Device;
    /// use beep_evdev::{Beep, Note, DEFAULT_FILE};
    ///
    /// let melody: Vec<Note> = vec![
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
    ///
    /// Device::open(DEFAULT_FILE)
    ///     .expect("could not open pcspkr device")
    ///     .play(&melody)
    ///     .expect("could not play melody :-(");
    /// ```
    ///
    /// # Errors
    /// Returns an [`std::io::Error`] on I/O errors.
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
