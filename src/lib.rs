use num_format::{Locale, ToFormattedString};
use serde::de::{self, Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, BufReader};
use std::path::Path;

pub mod decision_tree;

pub const WORD_SIZE: usize = 5;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Char {
	c: u8,
}

impl Char {
	pub fn from_char(c: char) -> Char {
		let o = c as u32;
		if o < 97 || o > 122 {
			panic!("invalid character: {}", c);
		}
		Char { c: (o as u8 - 97) }
	}

	pub fn from_u8(c: u8) -> Char {
		if c > 25 {
			panic!("invalid character: {}", c);
		}
		Char { c }
	}

	pub fn char(&self) -> char {
		(self.c + 97) as char
	}

	pub fn ord(&self) -> u8 {
		self.c
	}
}

impl Default for Char {
	fn default() -> Self {
		Char { c: 0 }
	}
}

struct WordVisitor;

impl<'de> Visitor<'de> for WordVisitor {
	type Value = Word;

	fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "5 character word")
	}

	fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		match Word::from_str(val) {
			Ok(word) => Ok(word),
			Err(e) => Err(E::custom(e)),
		}
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Word {
	chars: [Char; WORD_SIZE],
}

impl Word {
	pub fn from_str(s: &str) -> Result<Word, Box<dyn Error>> {
		if s.len() == WORD_SIZE {
			let mut chars = [Char::default(); WORD_SIZE];
			for (i, c) in s.char_indices() {
				chars[i] = Char::from_char(c);
			}
			Ok(Word { chars })
		} else {
			Err("a word has to be 5 chars".into())
		}
	}

	pub fn chars(&self) -> &[Char] {
		&self.chars
	}

	pub fn contains(&self, c: Char) -> bool {
		self.chars.contains(&c)
	}

	pub fn to_string(&self) -> String {
		self.chars().iter().map(|c| c.char()).collect::<String>()
	}
}

impl std::ops::Index<usize> for Word {
	type Output = Char;

	fn index(&self, ix: usize) -> &Self::Output {
		&self.chars[ix]
	}
}

impl std::fmt::Display for Word {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl Serialize for Word {
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		s.serialize_str(&self.to_string())
	}
}

impl<'de> Deserialize<'de> for Word {
	fn deserialize<D>(d: D) -> Result<Word, D::Error>
	where
		D: Deserializer<'de>,
	{
		d.deserialize_str(WordVisitor {})
	}
}

struct FeedbackVisitor;

impl<'de> Visitor<'de> for FeedbackVisitor {
	type Value = Feedback;

	fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "a 5 character feedback string")
	}

	fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
	where
		E: de::Error,
	{
		match Feedback::from_str(val) {
			Ok(f) => Ok(f),
			Err(e) => Err(E::custom(e)),
		}
	}
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Feedback {
	directives: [Directive; WORD_SIZE],
}

impl Feedback {
	pub fn from_str(s: &str) -> Result<Feedback, Box<dyn Error>> {
		if s.len() != WORD_SIZE {
			return Err(format!("feedback must be {} characters", WORD_SIZE).into());
		}
		let mut directives = [Directive::Green; WORD_SIZE];
		for (i, c) in s.char_indices() {
			directives[i] = match c {
				'g' | 'G' => Directive::Green,
				'y' | 'Y' => Directive::Yellow,
				'b' | 'B' => Directive::Black,
				_ => return Err(format!("invalid directive: {}", c).into()),
			};
		}
		Ok(Feedback { directives })
	}

	pub fn from_word(guess: &Word, solution: &Word) -> Feedback {
		// TODO(knorton): This is doggy doo.
		let mut directives = [Directive::Black; WORD_SIZE];
		let mut resolved = [false; WORD_SIZE];
		for (i, c) in guess.chars().iter().enumerate() {
			if solution[i] == *c {
				directives[i] = Directive::Green;
				resolved[i] = true;
			}
		}
		for (i, c) in guess.chars().iter().enumerate() {
			if directives[i] == Directive::Green {
				continue;
			}
			for (j, k) in solution.chars().iter().enumerate() {
				if !resolved[j] && *k == *c {
					directives[i] = Directive::Yellow;
					resolved[j] = true;
					break;
				}
			}
		}
		Feedback { directives }
	}

	pub fn to_string(&self) -> String {
		self.directives().iter().map(|d| d.to_char()).collect()
	}

	pub fn to_ascii_string(&self) -> String {
		self.directives()
			.iter()
			.map(|d| d.to_ascii_char())
			.collect()
	}

	fn directives(&self) -> &[Directive] {
		&self.directives
	}
}

impl std::ops::Index<usize> for Feedback {
	type Output = Directive;

	fn index(&self, ix: usize) -> &Self::Output {
		&self.directives[ix]
	}
}

impl std::fmt::Display for Feedback {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(f, "{}", self.to_string())
	}
}

impl Serialize for Feedback {
	fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		s.serialize_str(&self.to_ascii_string())
	}
}

impl<'de> Deserialize<'de> for Feedback {
	fn deserialize<D>(d: D) -> Result<Feedback, D::Error>
	where
		D: Deserializer<'de>,
	{
		d.deserialize_str(FeedbackVisitor {})
	}
}

#[derive(Clone, Debug)]
pub struct Guess {
	feedback: Feedback,
	word: Word,
}

// TODO(knorton): Fix the names of methods here.
impl Guess {
	pub fn new(word: &Word, solution: &Word) -> Guess {
		Guess {
			word: *word,
			feedback: Feedback::from_word(word, solution),
		}
	}

	pub fn from_feedback_and_word(feedback: Feedback, word: Word) -> Guess {
		Guess {
			feedback: feedback,
			word: word,
		}
	}

	pub fn feedback(&self) -> &Feedback {
		&self.feedback
	}

	pub fn word(&self) -> &Word {
		&self.word
	}

	pub fn matches(&self, candidate: &Word) -> bool {
		(0..WORD_SIZE).all(|i| self.feedback[i].matches(i, &self.word, candidate))
	}

	pub fn is_all_green(&self) -> bool {
		self.feedback
			.directives()
			.iter()
			.all(|d| *d == Directive::Green)
	}

	pub fn iter(&self) -> impl Iterator<Item = (Directive, Char)> + '_ {
		(0..WORD_SIZE).map(|i| (self.feedback[i], self.word[i]))
	}
}

impl std::fmt::Display for Guess {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{} {}",
			self.word().to_string().to_uppercase(),
			self.feedback.to_string()
		)
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
#[repr(u8)]
pub enum Directive {
	Green,
	Yellow,
	Black,
}

impl Directive {
	fn matches(&self, i: usize, guess: &Word, candidate: &Word) -> bool {
		match self {
			Directive::Green => guess[i] == candidate[i],
			Directive::Yellow => guess[i] != candidate[i] && candidate.contains(guess[i]),
			Directive::Black => !candidate.contains(guess[i]),
		}
	}

	fn to_ascii_char(&self) -> char {
		match self {
			Directive::Green => 'g',
			Directive::Yellow => 'y',
			Directive::Black => 'b',
		}
	}

	fn to_char(&self) -> char {
		match self {
			Directive::Green => '🟩',
			Directive::Yellow => '🟨',
			Directive::Black => '⬛',
		}
	}
}

pub fn report_stats(mut num_guesses: Vec<usize>) {
	if num_guesses.is_empty() {
		return;
	}

	num_guesses.sort();

	let n = num_guesses.len();

	let max = num_guesses[n - 1];
	let mut hist = HashMap::new();
	for n in num_guesses.iter() {
		*hist.entry(n).or_insert(0) += 1;
	}
	println!("Total:           {}", n.to_formatted_string(&Locale::en));
	println!(
		"Median Guesses:  {}",
		num_guesses[n / 2].to_formatted_string(&Locale::en)
	);
	println!("Max Guesses:     {}", max.to_formatted_string(&Locale::en));
	println!(
		"Avg Guesses:     {:0.1}",
		num_guesses.iter().sum::<usize>() as f64 / n as f64
	);
	let failed = num_guesses.iter().filter(|g| **g > 6).count();
	println!(
		"Percent Failed:  {:0.1}% ({})",
		100.0 * failed as f64 / n as f64,
		failed.to_formatted_string(&Locale::en)
	);
	println!();
	println!("Guesses Histogram");
	let dw = 60.0 / *hist.values().max().unwrap() as f64;
	for i in 1..=max {
		let v = *hist.get(&i).unwrap_or(&0);
		let w = v as f64 * dw;
		let bar = std::iter::repeat("░").take(w as usize).collect::<String>();
		println!(
			"{:2}: ░{} {} ({:0.1}%)",
			i,
			bar,
			v.to_formatted_string(&Locale::en),
			100.0 * v as f64 / n as f64
		);
	}
}

pub fn words_from_reader<R: io::BufRead>(r: R) -> Result<Vec<Word>, Box<dyn Error>> {
	let mut words = Vec::new();
	for line in r.lines() {
		words.push(Word::from_str(&line?)?);
	}
	Ok(words)
}

pub fn words_from_file<P: AsRef<Path>>(src: P) -> Result<Vec<Word>, Box<dyn Error>> {
	words_from_reader(BufReader::new(fs::File::open(src)?))
}
