use crate::Note;

/// A sequence of notes
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Melody(Box<[Note]>);

impl Melody {
    #[must_use]
    pub fn new(notes: Box<[Note]>) -> Self {
        Self(notes)
    }
}

impl AsRef<[Note]> for Melody {
    fn as_ref(&self) -> &[Note] {
        &self.0
    }
}

impl Default for Melody {
    fn default() -> Self {
        Self::new(Box::new([Note::default()]))
    }
}

impl From<Vec<Note>> for Melody {
    fn from(notes: Vec<Note>) -> Self {
        Self::new(notes.into_boxed_slice())
    }
}

impl From<&[Note]> for Melody {
    fn from(notes: &[Note]) -> Self {
        Self::new(notes.into())
    }
}
