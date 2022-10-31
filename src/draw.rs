use io::Write;

use super::*;

pub fn draw_svg_sound (out: & mut dyn Write, sound: Sound) -> io::Result <()> {
	write! (out, "<svg viewBox=\"0 0 90 {GLYPH_HEIGHT_PADDED}\"><path d=\"") ?;
	draw_svg_segments (out, 0, sound.segments () ) ?;
	write! (out, "\"/></svg>") ?;
	Ok (())
}

pub fn draw_svg_word (out: & mut dyn Write, glyphs: & [Glyph]) -> io::Result <()> {
	write! (out,
		"<svg viewBox=\"0 0 {view_width} {GLYPH_HEIGHT_PADDED}\">\
		<path d=\"",
		view_width = 80 * glyphs.len () + 10) ?;
	let mut first = true;
	for (pos, & glyph) in glyphs.iter ().enumerate () {
		let pos = pos as i32;
		if ! first { write! (out, " ") ?; }
		draw_svg_segments (out, pos, glyph.segments () | 0b_10_0000000_00000) ?;
		first = false;
	}
	write! (out, "\"/></svg>") ?;
	Ok (())
}

pub fn draw_svg_segments (out: & mut dyn Write, pos: i32, segs: u16) -> io::Result <()> {
	let mut first = true;
	if segs & 0b_01_0000000_00000 != 0 {
		let flip_top = FLIP_TOP.offset (pos);
		let flip_left = FLIP_LEFT.offset (pos);
		write! (out,
			"M {} {} \
			A {FLIP_RADIUS} {FLIP_RADIUS} 0 0 0 {} {} \
			A {FLIP_RADIUS} {FLIP_RADIUS} 0 1 0 {} {}",
			flip_top.x, flip_top.y,
			flip_left.x, flip_left.y,
			flip_top.x, flip_top.y,
		) ?;
		first = false;
	}
	let mut prev = Point { x: f32::MIN, y: f32::MIN };
	for (bit, start, end) in [
		(0b_10_0000000_00000, MIDDLE_LEFT, MIDDLE_RIGHT),
		(0b_00_1000000_00000, UPPER_LEFT, UPPER_BOTTOM),
		(0b_00_0100000_00000, UPPER_TOP, UPPER_BOTTOM),
		(0b_00_0010000_00000, UPPER_RIGHT, UPPER_BOTTOM),
		(0b_00_0001000_00000, UPPER_BOTTOM, LOWER_TOP),
		(0b_00_0000100_00000, LOWER_TOP, LOWER_LEFT),
		(0b_00_0000010_00000, LOWER_TOP, LOWER_BOTTOM),
		(0b_00_0000001_00000, LOWER_TOP, LOWER_RIGHT),
		(0b_00_0000000_10000, UPPER_RIGHT, UPPER_TOP),
		(0b_00_0000000_01000, UPPER_TOP, UPPER_LEFT),
		(0b_00_0000000_00100, UPPER_LEFT, LOWER_LEFT),
		(0b_00_0000000_00010, LOWER_LEFT, LOWER_BOTTOM),
		(0b_00_0000000_00001, LOWER_BOTTOM, LOWER_RIGHT),
	] {
		if segs & bit == 0 { continue }
		let start = start.offset (pos);
		let end = end.offset (pos);
		if ! first { write! (out, " ") ?; }
		if start != prev {
			write! (out, "M {} {} L {} {}", start.x, start.y, end.x, end.y) ?;
		} else {
			write! (out, "{} {}", end.x, end.y) ?;
		}
		first = false;
		prev = end;
	}
	Ok (())
}

#[ derive (Clone, Copy, PartialEq) ]
struct Point { x: f32, y: f32 }

impl Point {
	fn offset (self, pos: i32) -> Self {
		Point { x: self.x + GLYPH_WIDTH as f32 * pos as f32, y: self.y }
	}
}

pub const GLYPH_WIDTH: i32 = 80;
pub const GLYPH_HEIGHT: i32 = 190;
pub const GLYPH_PAD: i32 = 5;

pub const GLYPH_HEIGHT_PADDED: i32 = GLYPH_HEIGHT + GLYPH_PAD * 2;

const X_LEFT: f32 = 5.0;
const X_MIDDLE: f32 = 45.0;
const X_RIGHT: f32 = 85.0;

const Y_UPPER_TOP: f32 = 25.0;
const Y_UPPER_MIDDLE: f32 = 50.0;
const Y_UPPER_BOTTOM: f32 = 75.0;
const Y_MIDDLE: f32 = 100.0;
const Y_LOWER_TOP: f32 = 125.0;
const Y_LOWER_MIDDLE: f32 = 150.0;
const Y_LOWER_BOTTOM: f32 = 175.0;
const Y_FLIP: f32 = 185.0;

const FLIP_RADIUS: f32 = 10.0;

const UPPER_TOP: Point = Point { x: X_MIDDLE, y: Y_UPPER_TOP };
const UPPER_LEFT: Point = Point { x: X_LEFT, y: Y_UPPER_MIDDLE };
const UPPER_RIGHT: Point = Point { x: X_RIGHT, y: Y_UPPER_MIDDLE };
const UPPER_BOTTOM: Point = Point { x: X_MIDDLE, y: Y_UPPER_BOTTOM };
const MIDDLE_LEFT: Point = Point { x: X_LEFT, y: Y_MIDDLE };
const MIDDLE_RIGHT: Point = Point { x: X_RIGHT, y: Y_MIDDLE };
const LOWER_TOP: Point = Point { x: X_MIDDLE, y: Y_LOWER_TOP };
const LOWER_LEFT: Point = Point { x: X_LEFT, y: Y_LOWER_MIDDLE };
const LOWER_RIGHT: Point = Point { x: X_RIGHT, y: Y_LOWER_MIDDLE };
const LOWER_BOTTOM: Point = Point { x: X_MIDDLE, y: Y_LOWER_BOTTOM };
const FLIP_TOP: Point = Point { x: X_MIDDLE, y: Y_FLIP - FLIP_RADIUS };
const FLIP_LEFT: Point = Point { x: X_MIDDLE - FLIP_RADIUS, y: Y_FLIP };
