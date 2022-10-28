use super::*;

pub use Glyph::*;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum Glyph {
	One (Sound),
	Two (Sound, Sound),
}

impl Glyph {
	pub fn one (sound: Sound) -> Self {
		One (sound)
	}
	pub fn two (sound_0: Sound, sound_1: Sound) -> Self {
		if sound_0.is_vowel () == sound_1.is_vowel () { panic! () }
		Two (sound_0, sound_1)
	}
}
