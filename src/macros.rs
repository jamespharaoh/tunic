macro_rules! words {
	($($name:ident $($label:literal)?: $($($comp:ident)*),* $($notes:literal)?);* $(;)?) => {
		words! (@gen_words_fn $($name $($label)?: $($($comp)*),* $($notes)?);*);
		words! (@gen_words_table_fn $($name $($label)?: $($($comp)*),* $($notes)?);*);
	};
	(@gen_words_fn $($name:ident $($label:literal)?: $($($comp:ident)*),* $($notes:literal)?);*) => {
		pub fn gen_words () {
			let mut word_names = HashSet::new ();
			$(
				if ! word_names.insert (stringify! ($name)) {
					panic! ("Duplicated word: {}", stringify! ($name));
				}
				gen_word (
					concat! ("target/site/word-", stringify! ($name), ".png"),
					& [ $(words! (@glyph $($comp)*)),* ]);
			)*
		}
	};
	(@gen_words_table_fn $($name:ident $($label:literal)?: $($($comp:ident)*),* $($notes:literal)?);*) => {
		pub fn gen_words_table (out: & mut dyn Write) -> io::Result <()> {
			let mut phonetic = String::new ();
			$(
				#[ allow (unused_variables) ]
				let notes = "";
				$( let notes = $notes; )?
				#[ allow (unused_variables) ]
				let label = stringify! ($name);
				$( let label = $label; )?
				phonetic.clear ();
				$(
					if ! phonetic.is_empty () { phonetic += "/"; }
					$( phonetic += stringify! ($comp); )*
				)*
				write! (out,
					concat! (
						"\t\t<tr> ",
						"<td><img src=\"word-{name}.png\"></td> ",
						"<td>{label}</td> ",
						"<td>{phonetic}</td>" ,
						"<td>{notes}</td> ",
						"</tr>\n",
					),
					name = stringify! ($name),
					label = label,
					phonetic = phonetic,
					notes = notes,
				) ?;
			)*
			Ok (())
		}
	};

	// vowel a

	(@glyph a) => { Vowel (VOWEL_AA) };
	(@glyph $cons:ident a) => { ConsVowel (words! (@cons $cons), VOWEL_AA) };
	(@glyph a $cons:ident) => { VowelCons (VOWEL_AA, words! (@cons $cons)) };

	// vowel aɪ

	(@glyph aɪ) => { Vowel (VOWEL_AI) };
	(@glyph $cons:ident aɪ) => { ConsVowel (words! (@cons $cons), VOWEL_AI) };
	(@glyph aɪ $cons:ident) => { VowelCons (VOWEL_AI, words! (@cons $cons)) };

	// vowel aʊ

	(@glyph aʊ) => { Vowel (VOWEL_OU) };
	(@glyph $cons:ident aʊ) => { ConsVowel (words! (@cons $cons), VOWEL_OU) };
	(@glyph aʊ $cons:ident) => { VowelCons (VOWEL_OU, words! (@cons $cons)) };

	// vowel ɑr

	(@glyph ɑr) => { Vowel (VOWEL_AR) };
	(@glyph $cons:ident ɑr) => { ConsVowel (words! (@cons $cons), VOWEL_AR) };
	(@glyph ɑr $cons:ident) => { VowelCons (VOWEL_AR, words! (@cons $cons)) };

	// vowel eɪ

	(@glyph eɪ) => { Vowel (VOWEL_EI) };
	(@glyph $cons:ident eɪ) => { ConsVowel (words! (@cons $cons), VOWEL_EI) };
	(@glyph eɪ $cons:ident) => { VowelCons (VOWEL_EI, words! (@cons $cons)) };

	// vowel ə/ʌ

	(@glyph ə) => { Vowel (VOWEL_A) };
	(@glyph ʌ) => { Vowel (VOWEL_A) };
	(@glyph $cons:ident ə) => { ConsVowel (words! (@cons $cons), VOWEL_A) };
	(@glyph $cons:ident ʌ) => { ConsVowel (words! (@cons $cons), VOWEL_A) };
	(@glyph ə $cons:ident) => { VowelCons (VOWEL_A, words! (@cons $cons)) };
	(@glyph ʌ $cons:ident) => { VowelCons (VOWEL_A, words! (@cons $cons)) };

	// vowel ər

	(@glyph ər) => { Vowel (VOWEL_SHWAR) };
	(@glyph $cons:ident ər) => { ConsVowel (words! (@cons $cons), VOWEL_SHWAR) };
	(@glyph ər $cons:ident) => { VowelCons (VOWEL_SHWAR, words! (@cons $cons)) };

	// vowel ε

	(@glyph ε) => { Vowel (VOWEL_SHWA) };
	(@glyph $cons:ident ε) => { ConsVowel (words! (@cons $cons), VOWEL_SHWA) };
	(@glyph ε $cons:ident) => { VowelCons (VOWEL_SHWA, words! (@cons $cons)) };

	// vowel εər

	(@glyph εər) => { Vowel (VOWEL_ER) };
	(@glyph $cons:ident εər) => { ConsVowel (words! (@cons $cons), VOWEL_ER) };
	(@glyph εər $cons:ident) => { VowelCons (VOWEL_ER, words! (@cons $cons)) };

	// vowel i

	(@glyph i) => { Vowel (VOWEL_II) };
	(@glyph $cons:ident i) => { ConsVowel (words! (@cons $cons), VOWEL_II) };
	(@glyph i $cons:ident) => { VowelCons (VOWEL_II, words! (@cons $cons)) };

	// vowel ɪ

	(@glyph ɪ) => { Vowel (VOWEL_I) };
	(@glyph $cons:ident ɪ) => { ConsVowel (words! (@cons $cons), VOWEL_I) };
	(@glyph ɪ $cons:ident) => { VowelCons (VOWEL_I, words! (@cons $cons)) };

	// vowel ɪər

	(@glyph ɪər) => { Vowel (VOWEL_IR) };
	(@glyph $cons:ident ɪər) => { ConsVowel (words! (@cons $cons), VOWEL_IR) };
	(@glyph ɪər $cons:ident) => { VowelCons (VOWEL_IR, words! (@cons $cons)) };

	// vowel oʊ

	(@glyph oʊ) => { Vowel (VOWEL_O) };
	(@glyph $cons:ident oʊ) => { ConsVowel (words! (@cons $cons), VOWEL_O) };
	(@glyph oʊ $cons:ident) => { VowelCons (VOWEL_O, words! (@cons $cons)) };

	// vowel ɔ

	(@glyph ɔ) => { Vowel (VOWEL_OO) };
	(@glyph $cons:ident ɔ) => { ConsVowel (words! (@cons $cons), VOWEL_OO) };
	(@glyph ɔ $cons:ident) => { VowelCons (VOWEL_OO, words! (@cons $cons)) };

	// vowel ɔi

	(@glyph ɔi) => { Vowel (VOWEL_OI) };
	(@glyph $cons:ident ɔi) => { ConsVowel (words! (@cons $cons), VOWEL_OI) };
	(@glyph ɔi $cons:ident) => { VowelCons (VOWEL_OI, words! (@cons $cons)) };

	// vowel ɔr

	(@glyph ɔr) => { Vowel (VOWEL_OR) };
	(@glyph $cons:ident ɔr) => { ConsVowel (words! (@cons $cons), VOWEL_OR) };
	(@glyph ɔr $cons:ident) => { VowelCons (VOWEL_OR, words! (@cons $cons)) };

	// vowel u

	(@glyph u) => { Vowel (VOWEL_UU) };
	(@glyph $cons:ident u) => { ConsVowel (words! (@cons $cons), VOWEL_UU) };
	(@glyph u $cons:ident) => { VowelCons (VOWEL_UU, words! (@cons $cons)) };

	// vowel ʊ

	(@glyph ʊ) => { Vowel (VOWEL_UPSILON) };
	(@glyph $cons:ident ʊ) => { ConsVowel (words! (@cons $cons), VOWEL_UPSILON) };
	(@glyph ʊ $cons:ident) => { VowelCons (VOWEL_UPSILON, words! (@cons $cons)) };

	// consonant

	(@glyph $cons:ident) => { Cons (words! (@cons $cons)) };

	(@cons b) => { CONS_B };
	(@cons tʃ) => { CONS_CH };
	(@cons d) => { CONS_D };
	(@cons f) => { CONS_F };
	(@cons g) => { CONS_G };
	(@cons h) => { CONS_H };
	(@cons dʒ) => { CONS_J };
	(@cons k) => { CONS_K };
	(@cons l) => { CONS_L };
	(@cons m) => { CONS_M };
	(@cons n) => { CONS_N };
	(@cons ŋ) => { CONS_NG };
	(@cons p) => { CONS_P };
	(@cons r) => { CONS_R };
	(@cons s) => { CONS_S };
	(@cons ʃ) => { CONS_SH };
	(@cons t) => { CONS_T };
	(@cons θ) => { CONS_TH0 };
	(@cons ð) => { CONS_TH1 };
	(@cons v) => { CONS_V };
	(@cons w) => { CONS_W };
	(@cons y) => { CONS_Y };
	(@cons z) => { CONS_Z };
	(@cons ʒ) => { CONS_ZH };
}
