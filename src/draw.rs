use io::Write;
use raqote::*;

use super::*;

pub fn draw_svg (out: & mut dyn Write, glyphs: & [Glyph]) -> io::Result <()> {
	write! (out,
		"<svg viewBox=\"0 0 {view_width} 180\">\
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

pub fn draw_sound (target: & mut DrawTarget, pos: i32, sound: Sound) {
	let segs = sound.segments ();
	if segs & 0b_1000000_00000 != 0 { draw_segment (target, pos, UPPER_LEFT, UPPER_BOTTOM); }
	if segs & 0b_0100000_00000 != 0 { draw_segment (target, pos, UPPER_TOP, UPPER_BOTTOM); }
	if segs & 0b_0010000_00000 != 0 { draw_segment (target, pos, UPPER_RIGHT, UPPER_BOTTOM); }
	if segs & 0b_0001000_00000 != 0 { draw_segment (target, pos, UPPER_BOTTOM, LOWER_TOP); }
	if segs & 0b_0000100_00000 != 0 { draw_segment (target, pos, LOWER_TOP, LOWER_LEFT); }
	if segs & 0b_0000010_00000 != 0 { draw_segment (target, pos, LOWER_TOP, LOWER_BOTTOM); }
	if segs & 0b_0000001_00000 != 0 { draw_segment (target, pos, LOWER_TOP, LOWER_RIGHT); }
	if segs & 0b_0000000_10000 != 0 { draw_segment (target, pos, UPPER_RIGHT, UPPER_TOP); }
	if segs & 0b_0000000_01000 != 0 { draw_segment (target, pos, UPPER_TOP, UPPER_LEFT); }
	if segs & 0b_0000000_00100 != 0 { draw_segment (target, pos, UPPER_LEFT, LOWER_LEFT); }
	if segs & 0b_0000000_00010 != 0 { draw_segment (target, pos, LOWER_LEFT, LOWER_BOTTOM); }
	if segs & 0b_0000000_00001 != 0 { draw_segment (target, pos, LOWER_BOTTOM, LOWER_RIGHT); }
}

pub fn draw_middle (target: & mut DrawTarget, pos: i32) {
	draw_segment (target, pos, MIDDLE_LEFT, MIDDLE_RIGHT);
}

pub fn draw_flip (target: & mut DrawTarget, pos: i32) {
	let point = FLIP_MIDDLE.offset (pos);
	let mut path = PathBuilder::new ();
	path.arc (point.x, point.y, FLIP_RADIUS, 0.0, 360.0);
	let path = path.finish ();
	draw_path (target, path);
}

fn draw_segment (target: & mut DrawTarget, pos: i32, start: Point, end: Point) {
	let mut path = PathBuilder::new ();
	let start = start.offset (pos);
	let end = end.offset (pos);
	path.move_to (start.x, start.y);
	path.line_to (end.x, end.y);
	let path = path.finish ();
	draw_path (target, path);
}

fn draw_path (target: & mut DrawTarget, path: Path) {
	target.stroke (
		& path,
		& Source::Solid (SolidSource { r: 0, g: 0, b: 0, a: 255 }),
		& StrokeStyle {
			cap: LineCap::Round,
			join: LineJoin::Round,
			width: 5.0,
			miter_limit: 1.0,
			.. Default::default ()
		},
		& DrawOptions::new ());
}

#[ derive (Clone, Copy, PartialEq) ]
struct Point { x: f32, y: f32 }

impl Point {
	fn offset (self, pos: i32) -> Self {
		Point { x: self.x + 80.0 * pos as f32, y: self.y }
	}
}

pub const WIDTH: i32 = 90;
pub const HEIGHT: i32 = 180;

const X_LEFT: f32 = 5.0;
const X_MIDDLE: f32 = 45.0;
const X_RIGHT: f32 = 85.0;

const Y_UPPER_TOP: f32 = 5.0;
const Y_UPPER_MIDDLE: f32 = 30.0;
const Y_UPPER_BOTTOM: f32 = 55.0;
const Y_MIDDLE: f32 = 80.0;
const Y_LOWER_TOP: f32 = 105.0;
const Y_LOWER_MIDDLE: f32 = 130.0;
const Y_LOWER_BOTTOM: f32 = 155.0;
const Y_FLIP: f32 = 165.0;

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
const FLIP_MIDDLE: Point = Point { x: X_MIDDLE, y: Y_FLIP };
const FLIP_TOP: Point = Point { x: X_MIDDLE, y: Y_FLIP - FLIP_RADIUS };
const FLIP_LEFT: Point = Point { x: X_MIDDLE - FLIP_RADIUS, y: Y_FLIP };
