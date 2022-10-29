pub use Sound::*;

#[ derive (Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd) ]
pub enum Sound {
	M, W, P, B, CH, J, Y, L, R, H, F, V, K, G, S, Z, T, D, TH, DH, SH, ZH, N, NG, E, I, O, U, A,
	EE, UU, II, OR, ER, EER, IER, OU, AR, EI, OI, AI, AU,
}

impl Sound {
	pub fn name (self) -> & 'static str {
		match self {
			M => "m", W => "w", P => "p", B => "b", CH => "ch", J => "j", Y => "y", L => "l",
			R => "r", H => "h", F => "f", V => "v", K => "k", G => "g", S => "s", Z => "z",
			T => "t", D => "d", TH => "th", DH => "dh", SH => "sh", ZH => "zh", N => "n",
			NG => "ng", E => "e", I => "i", O => "o", U => "u", A => "a", EE => "ee", UU => "uu",
			II => "ii", OR => "or", ER => "er", EER => "eer", IER => "ier", OU => "ou",
			AR => "ar", EI => "ei", OI => "oi", AI => "ai", AU => "au",
		}
	}
	pub fn is_cons (self) -> bool {
		matches! (self, M | W | P | B | CH | J | Y | L | R | H | F | V | K | G | S | Z | T | D
			| TH | DH | SH | ZH | N | NG)
	}
	pub fn is_vowel (self) -> bool {
		matches! (self, E | I | O | U | A | EE | UU | II | OR | ER | EER | IER | OU | AR | EI | OI
			| AI | AU)
	}
	pub fn match_strs (self) -> & 'static [& 'static str] {
		match self {
			M => & [ "m" ],
			W => & [ "w" ],
			P => & [ "p" ],
			B => & [ "b" ],
			CH => & [ "tʃ" ],
			J => & [ "dʒ" ],
			Y => & [ "y" ],
			L => & [ "l" ],
			R => & [ "r" ],
			H => & [ "h" ],
			F => & [ "f" ],
			V => & [ "v" ],
			K => & [ "k" ],
			G => & [ "g" ],
			S => & [ "s" ],
			Z => & [ "z" ],
			T => & [ "t" ],
			D => & [ "d" ],
			TH => & [ "θ" ],
			DH => & [ "ð" ],
			SH => & [ "ʃ" ],
			ZH => & [ "ʒ" ],
			N => & [ "n" ],
			NG => & [ "ŋ" ],
			E => & [ "ə", "ʌ" ],
			I => & [ "ɪ" ],
			O => & [ "ɔ", "ɒ" ],
			U => & [ "ʊ" ],
			A => & [ "a" ],
			EE => & [ "ε" ],
			UU => & [ "u" ],
			II => & [ "i" ],
			OR => & [ "ɔr", "ɒr" ],
			ER => & [ "ər" ],
			EER => & [ "εər" ],
			IER => & [ "ɪər" ],
			OU => & [ "oʊ" ],
			AR => & [ "ar", "ɑr" ],
			EI => & [ "eɪ" ],
			OI => & [ "ɔɪ" ],
			AI => & [ "aɪ" ],
			AU => & [ "aʊ" ],
		}
	}
	pub fn segments (self) -> u16 {
		match self {
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
	pub const ALL: [Sound; 42] = [
		M, W, P, B, CH, J, Y, L, R, H, F, V, K, G, S, Z, T, D, TH, DH, SH, ZH, N, NG, E, I, O, U,
		A, EE, UU, II, OR, ER, EER, IER, OU, AR, EI, OI, AI, AU,
	];
	/*
	pub const CONSNS: [Sound; 24] = [
		M, W, P, B, CH, J, Y, L, R, H, F, V, K, G, S, Z, T, D, TH, DH, SH, ZH, N, NG,
	];
	pub const VOWELS: [Sound; 18] = [
		E, I, O, U, A, EE, UU, II, OR, ER, EER, IER, OU, AR, EI, OI, AI, AU,
	];
	*/
}
