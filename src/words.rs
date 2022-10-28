use std::fs::File;
use std::io::{ BufRead as _, BufReader };
use std::iter;

pub fn load_words () -> impl Iterator <Item = (String, String, String)> {
	let file = File::open ("words").unwrap ();
	let mut lines_iter = BufReader::new (file).lines ();
	iter::from_fn (move || {
		let line = lines_iter.next () ?.unwrap ();
		let latin = line.chars ().take (12).collect::<String> ().trim_end ().to_owned ();
		let phonetic = line.chars ().skip (12).take (16).collect::<String> ().trim_end ().to_owned ();
		let notes = line.chars ().skip (28).collect ();
		Some ((latin, phonetic, notes))
	})
}
