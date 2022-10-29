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
		if sound_0.is_vowel () && sound_1.is_vowel () {
			panic! ("Tried to form glyph from two vowels: {sound_0:?} {sound_1:?}");
		}
		if sound_0.is_cons () && sound_1.is_cons () {
			panic! ("Tried to form glyph from two consonants: {sound_0:?} {sound_1:?}");
		}
		Two (sound_0, sound_1)
	}
	pub fn segments (self) -> u16 {
		match self {
			One (sound) => sound.segments (),
			Two (sound_0, sound_1) => sound_0.segments () | sound_1.segments ()
				| if sound_0.is_vowel () { 0b_01_0000000_00000 } else { 0 },
		}
	}
}
