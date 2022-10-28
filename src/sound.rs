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
			O => & [ "ɔ" ],
			U => & [ "ʊ" ],
			A => & [ "a" ],
			EE => & [ "ε" ],
			UU => & [ "u" ],
			II => & [ "i" ],
			OR => & [ "ɔr" ],
			ER => & [ "ər" ],
			EER => & [ "εər" ],
			IER => & [ "ɪər" ],
			OU => & [ "oʊ" ],
			AR => & [ "ɑr" ],
			EI => & [ "eɪ" ],
			OI => & [ "ɔɪ" ],
			AI => & [ "aɪ" ],
			AU => & [ "aʊ" ],
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
