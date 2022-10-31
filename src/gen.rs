use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{ BufRead as _, BufReader, Write };

use super::*;

lazy_static! {
	static ref PLACEHOLDER_RE: Regex = Regex::new (r"\{([A-Z]+)\}").unwrap ();
}

pub fn gen_pages () {
	for name in [ "index", "names", "spoiler", "words" ] {
		let src = File::open (format! ("static/{name}.html")).unwrap ();
		let mut dst = File::create (format! ("target/site/{name}.html")).unwrap ();
		for line in BufReader::new (src).lines () {
			let line = line.unwrap ();
			let mut pos = 0;
			for cap in PLACEHOLDER_RE.captures_iter (& line) {
				let mat = cap.get (0).unwrap ();
				write! (dst, "{}", & line [pos .. mat.start ()]).unwrap ();
				let tag = & cap [1];
				if tag == "NAMES" { gen_words_table ("names", & mut dst).unwrap (); }
				else if tag == "WORDS" { gen_words_table ("words", & mut dst).unwrap (); }
				else {
					let mut found = false;
					for & sound in & Sound::ALL {
						if tag.eq_ignore_ascii_case (sound.name ()) {
							draw_svg_sound (& mut dst, sound).unwrap ();
							found = true;
							break;
						}
					}
					assert! (found);
				}
				pos = mat.end ();
			}
			write! (dst, "{}\n", & line [pos .. ]).unwrap ();
		}
		dst.flush ().unwrap ();
	}
}

fn gen_words_table (filename: & str, out: & mut dyn Write) -> io::Result <()> {
	for (latin, phonetic, notes) in load_words (filename) {
		let glyphs = match decode (& phonetic) {
			Some (glyphs) => glyphs,
			None => {
				eprintln! ("Unable to decode {latin}: {phonetic}");
				continue;
			},
		};
		write! (out,
			"\t\t<tr> \
			<td class=\"image\">") ?;
		draw_svg_word (out, & glyphs) ?;
		write! (out,
			"</td> \
			<td class=\"latin\">{latin}</td> \
			<td class=\"phonetic\">{phonetic}</td> \
			<td class=\"notes\">{notes}</td> \
			</tr>\n",
		) ?;
	}
	Ok (())
}
