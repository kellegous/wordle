use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
struct WordList {
	words: Vec<String>,
}

impl WordList {
	fn read<P: AsRef<Path>>(src: P) -> io::Result<WordList> {
		let words = BufReader::new(fs::File::open(src)?)
			.lines()
			.collect::<Result<Vec<String>, _>>()?;
		Ok(WordList { words })
	}

	fn first(&self) -> Cursor {
		Cursor { words: &self.words }
	}
}

struct Cursor<'a> {
	words: &'a [String],
}

impl<'a> Cursor<'a> {
	fn word(&self) -> &str {
		&self.words[0]
	}

	fn apply(&mut self, f: &Filter) -> bool {
		let current = self.word().chars().collect::<Vec<char>>();
		for (i, w) in self.words.iter().enumerate() {
			let candidate = w.chars().collect::<Vec<char>>();
			if f.matches(&current, &candidate) {
				self.words = &self.words[i..];
				return true;
			}
		}
		false
	}
}

#[derive(Copy, Clone)]
enum Directive {
	Green,
	Yellow,
	Gray,
}

impl Directive {
	fn matches(&self, i: usize, current: &[char], candidate: &[char]) -> bool {
		match self {
			Directive::Green => current[i] == candidate[i],
			Directive::Yellow => current[i] != candidate[i] && candidate.contains(&current[i]),
			Directive::Gray => !candidate.contains(&current[i]),
		}
	}
}

// type Filter = [Directive; 5];
struct Filter {
	directives: [Directive; 5],
}

impl Filter {
	fn from_str(s: &str) -> Result<Filter, Box<dyn Error>> {
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

	fn matches(&self, cur: &[char], candidate: &[char]) -> bool {
		(0..5).all(|i| self.directives[i].matches(i, cur, candidate))
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-solve")
		.arg(
			clap::Arg::new("words")
				.long("words")
				.takes_value(true)
				.default_value("wordle.txt")
				.help("word list file"),
		)
		.arg(
			clap::Arg::new("filters")
				.takes_value(true)
				.multiple_occurrences(true)
				.help("filters to apply"),
		)
		.get_matches();

	let wl = WordList::read(matches.value_of("words").unwrap())?;
	let filters = match matches.values_of("filters") {
		Some(vals) => vals
			.map(|s| Filter::from_str(s))
			.collect::<Result<Vec<_>, _>>()?,
		None => Vec::new(),
	};

	let mut c = wl.first();
	for filter in filters {
		c.apply(&filter);
	}
	println!("{}", c.word());
	Ok(())
}
