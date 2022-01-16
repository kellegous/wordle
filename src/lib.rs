use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct WordList {
	words: Vec<String>,
}

impl WordList {
	pub fn read<P: AsRef<Path>>(src: P) -> io::Result<WordList> {
		let words = BufReader::new(fs::File::open(src)?)
			.lines()
			.collect::<Result<Vec<String>, _>>()?;
		Ok(WordList { words })
	}

	pub fn first(&self) -> Cursor {
		Cursor {
			words: &self.words,
			guesses: Vec::new(),
		}
	}

	pub fn words(&self) -> &[String] {
		&self.words
	}
}

pub struct Cursor<'a> {
	words: &'a [String],
	guesses: Vec<(Vec<char>, Filter)>,
}

impl<'a> Cursor<'a> {
	pub fn word(&self) -> &str {
		&self.words[0]
	}

	pub fn apply(&mut self, f: &Filter) -> bool {
		let current = self.word().chars().collect::<Vec<char>>();
		for (i, w) in self.words.iter().enumerate() {
			let candidate = w.chars().collect::<Vec<char>>();
			let ok = self.guesses.iter().all(|(g, f)| f.matches(&g, &candidate));
			if !ok {
				continue;
			}
			if f.matches(&current, &candidate) {
				self.words = &self.words[i..];
				self.guesses.push((current, f.clone()));
				return true;
			}
		}
		false
	}
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
}

#[derive(Debug, Clone)]
pub struct Filter {
	directives: [Directive; 5],
}

impl std::fmt::Display for Filter {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		write!(
			f,
			"{}{}{}{}{}",
			self.directives[0],
			self.directives[1],
			self.directives[2],
			self.directives[3],
			self.directives[4]
		)
	}
}

impl Filter {
	pub fn from_str(s: &str) -> Result<Filter, Box<dyn Error>> {
		let mut directives = [Directive::Gray; 5];
		for (i, c) in s.chars().enumerate() {
			directives[i] = match c {
				'g' => Directive::Green,
				'y' => Directive::Yellow,
				'x' => Directive::Gray,
				_ => return Err(format!("invalid directive: {}", c).into()),
			}
		}
		Ok(Filter { directives })
	}

	pub fn from_guess(guess: &str, solution: &str) -> Filter {
		let mut directives = [Directive::Gray; 5];
		let gc = guess.chars().collect::<Vec<_>>();
		let sc = solution.chars().collect::<Vec<_>>();
		for (i, c) in gc.iter().enumerate() {
			directives[i] = if *c == sc[i] {
				Directive::Green
			} else if sc.contains(c) {
				Directive::Yellow
			} else {
				Directive::Gray
			};
		}
		Filter { directives }
	}

	pub fn matches(&self, cur: &[char], candidate: &[char]) -> bool {
		(0..5).all(|i| self.directives[i].matches(i, cur, candidate))
	}

	pub fn all_green(&self) -> bool {
		self.directives[0] == Directive::Green
			&& self.directives[1] == Directive::Green
			&& self.directives[2] == Directive::Green
			&& self.directives[3] == Directive::Green
			&& self.directives[4] == Directive::Green
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
