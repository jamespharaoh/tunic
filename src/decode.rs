use super::*;

pub fn decode (src: & str) -> Option <Vec <Glyph>> {
	src.split ('/')
		.map (|part| decode_span (part))
		.collect::<Option <Vec <Glyph>>> ()
}

pub fn decode_span (src: & str) -> Option <Glyph> {
	match decode_sound (src) {
		Some ((first, "")) => {
			Some (Glyph::one (first))
		},
		Some ((first, rest)) => match decode_sound (rest) {
			Some ((second, "")) => Some (Glyph::two (first, second)),
			_ => None,
		},
		None => None,
	}
}

pub fn decode_sound (src: & str) -> Option <(Sound, & str)> {
	let mut found = None;
	for & sound in & Sound::ALL {
		for match_str in sound.match_strs () {
			if src.starts_with (match_str) {
				if found.map (|(found_len, _)| found_len < match_str.len ()).unwrap_or (true) {
					found = Some ((match_str.len (), sound));
				}
			}
		}
	}
	found.map (|(len, sound)| (sound, & src [len .. ]))
}
