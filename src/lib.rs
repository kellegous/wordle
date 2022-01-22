use num_format::{Locale, ToFormattedString};
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, BufReader};
use std::path::Path;

const WORD_SIZE: usize = 5;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

	pub fn char(&self) -> char {
		(self.c + 97) as char
	}
}

impl Default for Char {
	fn default() -> Self {
		Char { c: 0 }
	}
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
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

#[derive(Clone, Copy)]
pub struct Feedback {
	directives: [Directive; WORD_SIZE],
}

impl Feedback {
	fn from_word(guess: &Word, solution: &Word) -> Feedback {
		let mut directives = [Directive::Green; WORD_SIZE];
		for (i, c) in guess.chars().iter().enumerate() {
			directives[i] = if *c == solution[i] {
				Directive::Green
			} else if solution.contains(*c) {
				Directive::Yellow
			} else {
				Directive::Gray
			};
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

#[derive(Clone)]
pub struct Guess {
	feedback: Feedback,
	word: Word,
}

impl Guess {
	pub fn new(word: &Word, solution: &Word) -> Guess {
		Guess {
			word: *word,
			feedback: Feedback::from_word(word, solution),
		}
	}

	pub fn feedback(&self) -> &Feedback {
		&self.feedback
	}

	pub fn word(&self) -> &Word {
		&self.word
	}

	pub fn matches(&self, candidate: &Word) -> bool {
		(0..WORD_SIZE).all(|i| self.feedback[i].matches2(i, &self.word, candidate))
	}

	pub fn is_all_green(&self) -> bool {
		self.feedback
			.directives()
			.iter()
			.all(|d| *d == Directive::Green)
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

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct Score {
	v: f64,
}

impl std::cmp::Ord for Score {
	fn cmp(&self, b: &Score) -> Ordering {
		match self.partial_cmp(b) {
			Some(o) => o,
			None => std::cmp::Ordering::Equal,
		}
	}
}

impl std::cmp::Eq for Score {}

impl From<f64> for Score {
	fn from(v: f64) -> Score {
		Score { v }
	}
}

pub struct Words {
	words: Vec<Word>,
}

impl Words {
	pub fn from_reader<R: io::BufRead>(r: R) -> Result<Words, Box<dyn Error>> {
		let mut words = Vec::new();
		for line in r.lines() {
			words.push(Word::from_str(&line?)?);
		}
		Ok(Words { words })
	}

	pub fn from_file<P: AsRef<Path>>(src: P) -> Result<Words, Box<dyn Error>> {
		Words::from_reader(&mut BufReader::new(fs::File::open(src)?))
	}

	pub fn first(&self) -> Option<Word> {
		if self.words.is_empty() {
			None
		} else {
			Some(self.words[0])
		}
	}

	pub fn words(&self) -> &[Word] {
		&self.words
	}

	pub fn filter<F>(&self, f: F) -> Words
	where
		F: FnMut(&&Word) -> bool,
	{
		Words {
			words: self.words.iter().filter(f).map(|w| *w).collect(),
		}
	}

	pub fn filter_into<F>(self, f: F) -> Words
	where
		F: FnMut(&Word) -> bool,
	{
		Words {
			words: self.words.into_iter().filter(f).collect(),
		}
	}

	pub fn rank<F>(&mut self, f: F)
	where
		F: Fn(&Word) -> f64,
	{
		self.words.sort_by_key(|w| Reverse(Score::from(f(w))));
	}
}

impl std::ops::Index<usize> for Words {
	type Output = Word;

	fn index(&self, ix: usize) -> &Self::Output {
		&self.words[ix]
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Directive {
	Green,
	Yellow,
	Gray,
}

impl std::fmt::Display for Directive {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Directive::Green => "🟩",
				Directive::Yellow => "🟨",
				Directive::Gray => "⬜",
			}
		)
	}
}

impl Directive {
	pub fn matches(&self, i: usize, current: &[char], candidate: &[char]) -> bool {
		match self {
			Directive::Green => current[i] == candidate[i],
			Directive::Yellow => current[i] != candidate[i] && candidate.contains(&current[i]),
			Directive::Gray => !candidate.contains(&current[i]),
		}
	}

	fn matches2(&self, i: usize, guess: &Word, candidate: &Word) -> bool {
		match self {
			Directive::Green => guess[i] == candidate[i],
			Directive::Yellow => guess[i] != candidate[i] && candidate.contains(guess[i]),
			Directive::Gray => !candidate.contains(guess[i]),
		}
	}

	fn to_ascii_char(&self) -> char {
		match self {
			Directive::Green => 'g',
			Directive::Yellow => 'y',
			Directive::Gray => 'x',
		}
	}

	fn to_char(&self) -> char {
		match self {
			Directive::Green => '🟩',
			Directive::Yellow => '🟨',
			Directive::Gray => '⬜',
		}
	}
}

pub fn report_stats(stats: &mut [usize]) {
	stats.sort();

	let max = stats[stats.len() - 1];

	let mut hist = HashMap::new();
	for n in stats.iter() {
		*hist.entry(n).or_insert(0) += 1;
	}

	println!(
		"Total:           {}",
		stats.len().to_formatted_string(&Locale::en)
	);
	println!(
		"Median Guesses:  {}",
		stats[stats.len() / 2].to_formatted_string(&Locale::en)
	);
	println!("Max Guesses:     {}", max.to_formatted_string(&Locale::en));
	println!(
		"Avg Guesses:     {:0.1}",
		stats.iter().sum::<usize>() as f64 / stats.len() as f64
	);

	let failed = stats.iter().filter(|g| **g > 6).count();
	println!(
		"Percent Failed:  {:0.1}% ({})",
		100.0 * failed as f64 / stats.len() as f64,
		failed.to_formatted_string(&Locale::en)
	);

	println!();

	println!("Guesses Histogram");
	let dw = 60.0 / *hist.values().max().unwrap() as f64;
	for i in 1..=max {
		let v = *hist.get(&i).unwrap_or(&0);
		let w = v as f64 * dw;
		let bar = std::iter::repeat("#").take(w as usize).collect::<String>();
		println!(
			"{:2}: #{} {} ({:0.1}%)",
			i,
			bar,
			v.to_formatted_string(&Locale::en),
			100.0 * v as f64 / stats.len() as f64
		);
	}
}
