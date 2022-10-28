use raqote::DrawTarget;
use std::collections::HashSet;
use std::fs::File;
use std::io::{ BufRead as _, BufReader, Write };

use super::*;

pub fn gen_sounds () {
	for & sound in & Sound::ALL {
		let prefix = if sound.is_cons () { "cons" } else { "vowel" };
		let name = sound.name ();
		let mut target = DrawTarget::new (WIDTH, HEIGHT);
		draw_sound (& mut target, 0, sound);
		target.write_png (format! ("target/site/{prefix}-{name}.png")).unwrap ();
	}
}

pub fn gen_words () {
	let mut names = HashSet::new ();
	for (_, phonetic, _) in load_words () {
		let glyphs = decode (& phonetic).unwrap ();
		let name = gen_name (& glyphs);
		if ! names.insert (name.clone ()) { continue }
		let mut target = DrawTarget::new (10 + 80 * glyphs.len () as i32, HEIGHT);
		for (pos, & glyph) in glyphs.iter ().enumerate () {
			let pos = pos as i32;
			match glyph {
				One (sound) => {
					draw_sound (& mut target, pos, sound);
				},
				Two (sound_0, sound_1) => {
					draw_sound (& mut target, pos, sound_0);
					draw_sound (& mut target, pos, sound_1);
					if sound_0.is_vowel () {
						draw_flip (& mut target, pos);
					}
				},
			}
			draw_middle (& mut target, pos);
		}
		target.write_png (format! ("target/site/word-{name}.png")).unwrap ();
	}
}

pub fn gen_words_table (out: & mut dyn Write) -> io::Result <()> {
	for (latin, phonetic, notes) in load_words () {
		let glyphs = decode (& phonetic).unwrap ();
		let name = gen_name (& glyphs);
		write! (out,
			concat! (
				"\t\t<tr> ",
				"<td class=\"image\"><img src=\"word-{name}.png\"></td> ",
				"<td class=\"latin\">{latin}</td> ",
				"<td class=\"phonetic\">{phonetic}</td>" ,
				"<td class=\"notes\">{notes}</td> ",
				"</tr>\n",
			),
			name = name,
			latin = latin,
			phonetic = phonetic,
			notes = notes,
		) ?;
	}
	Ok (())
}

pub fn gen_page () {
	let src = File::open ("static/index.html").unwrap ();
	let mut dst = File::create ("target/site/index.html").unwrap ();
	for line in BufReader::new (src).lines () {
		let line = line.unwrap ();
		if line.trim () == "{WORDS}" {
			gen_words_table (& mut dst).unwrap ();
		} else {
			write! (& mut dst, "{line}\n").unwrap ();
		}
	}
	dst.flush ().unwrap ();
}

fn gen_name (glyphs: & [Glyph]) -> String {
	let mut name = String::new ();
	for & glyph in glyphs {
		if ! name.is_empty () {
			name += "-";
		}
		match glyph {
			One (sound) => {
				name += sound.name ();
			},
			Two (sound_0, sound_1) => {
				name += sound_0.name ();
				name += sound_1.name ();
			}
		}
	}
	name
}