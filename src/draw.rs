use raqote::*;

use super::*;

pub fn draw_sound (target: & mut DrawTarget, pos: i32, sound: Sound) {
	let segs = sound_segments (sound);
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
	let point = FLIP_POS.offset (pos);
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

#[ derive (Clone, Copy) ]
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
const FLIP_POS: Point = Point { x: X_MIDDLE, y: Y_FLIP };

fn sound_segments (sound: Sound) -> u16 {
	match sound {
		M   => 0b_0000101_00000, W   => 0b_1010000_00000,
		P   => 0b_0011010_00000, B   => 0b_0101001_00000,
		CH  => 0b_1001010_00000, J   => 0b_0101100_00000,
		Y   => 0b_1101010_00000, L   => 0b_0101010_00000,
		R   => 0b_0111010_00000, H   => 0b_0101011_00000,
		F   => 0b_0011110_00000, V   => 0b_1101001_00000,
		K   => 0b_0111001_00000, G   => 0b_0011011_00000,
		S   => 0b_0111110_00000, Z   => 0b_1101011_00000,
		T   => 0b_1011010_00000, D   => 0b_0101101_00000,
		TH  => 0b_1111010_00000, DH  => 0b_0101111_00000,
		SH  => 0b_1011111_00000, ZH  => 0b_1111101_00000,
		N   => 0b_1000101_00000, NG  => 0b_1111111_00000,
		E   => 0b_0000000_11000, I   => 0b_0000000_00011,
		O   => 0b_0000000_01100, U   => 0b_0000000_00110,
		UU  => 0b_0000000_11110, II  => 0b_0000000_01111,
		A   => 0b_0000000_11100, EE  => 0b_0000000_00111,
		OR  => 0b_0000000_11101, ER  => 0b_0000000_10111,
		EER => 0b_0000000_00101, IER => 0b_0000000_01101,
		OU  => 0b_0000000_11111, AR  => 0b_0000000_11011,
		EI  => 0b_0000000_01000, OI  => 0b_0000000_00010,
		AI  => 0b_0000000_10000, AU  => 0b_0000000_00001,
	}
}
