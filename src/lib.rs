mod data;

pub use data::{EMOJI, VERSION};

pub struct Emoji {
    pub codepoints: &'static str,
    pub name: &'static str,
    flags: u8
}

impl Emoji {
    const FLAG_FLAG: u8 = 1;

    /// Returns true if this emoji represents a country flag
    pub fn is_flag(&self) -> bool {
        self.flags & Self::FLAG_FLAG == Self::FLAG_FLAG
    }
}
