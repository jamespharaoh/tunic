#![ allow (confusable_idents) ]
#![ allow (mixed_script_confusables) ]
#![ allow (uncommon_codepoints) ]

use std::fs;
use std::io;

mod decode;
mod draw;
mod gen;
mod glyph;
#[ macro_use ]
mod sound;
mod words;

use decode::*;
use draw::*;
use gen::*;
use glyph::*;
use sound::*;
use words::*;

fn main () {

	fs::remove_dir_all ("target/site")
		.or_else (|err|
			if matches! (err.kind (), io::ErrorKind::NotFound) { Ok (()) }
			else { Err (err) })
		.unwrap ();

	fs::create_dir_all ("target/site").unwrap ();

	gen_page ();
	gen_sounds ();
	gen_words ();

	fs::copy ("static/styles.css", "target/site/styles.css").unwrap ();

}
