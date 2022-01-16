use std::cmp::Reverse;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use wordle::{report_stats, Filter};

#[derive(Debug)]
struct Guess {
	word: String,
	filter: Filter,
}

#[derive(Debug)]
struct Solution {
	guesses: Vec<Guess>,
}

impl Solution {
	fn find(words: &WordList, solution: &str) -> Option<Solution> {
		let mut c = words.all();
		let mut guesses = Vec::new();
		loop {
			let word = c.top();
			let filter = Filter::from_guess(word, solution);
			guesses.push(Guess {
				word: word.to_owned(),
				filter: filter.clone(),
			});
			if filter.all_green() {
				break;
			}
			if !c.filter(&filter) {
				return None;
			}
		}

		Some(Solution { guesses })
	}

	fn number_of_guesses(&self) -> usize {
		self.guesses.len()
	}

	fn emit(&self) {
		for guess in self.guesses.iter() {
			println!("{} {}", guess.word, guess.filter)
		}
	}
}

fn word_to_vec(w: &str) -> Vec<char> {
	w.chars().collect()
}

struct WordList {
	words: Vec<String>,
}

impl WordList {
	fn read<P: AsRef<Path>>(src: P) -> io::Result<WordList> {
		let words = BufReader::new(fs::File::open(src)?)
			.lines()
			.collect::<Result<Vec<_>, _>>()?;
		Ok(WordList { words })
	}

	// fn filter(self, f: &Filter) -> WordList {
	// 	let current = word_to_vec(&self.words[0]);
	// 	let words = self
	// 		.words
	// 		.into_iter()
	// 		.filter(|w| f.matches(&current, &word_to_vec(&w)))
	// 		.collect::<Vec<String>>();
	// 	WordList {
	// 		words: Ranker::rank(words),
	// 	}
	// }

	fn words(&self) -> &[String] {
		&self.words
	}

	fn at(&self, i: usize) -> &str {
		&self.words[i]
	}

	fn all(&self) -> Filtered {
		Filtered {
			words: self,
			indexes: (0..self.words.len()).collect::<Vec<usize>>(),
		}
	}
}

struct Filtered<'a> {
	words: &'a WordList,
	indexes: Vec<usize>,
}

impl<'a> Filtered<'a> {
	fn top(&self) -> &str {
		self.words.at(self.indexes[0])
	}

	fn filter(&mut self, f: &Filter) -> bool {
		let current = word_to_vec(self.top());
		let mut indexes = Vec::new();
		for ix in &self.indexes {
			let candidate = word_to_vec(self.words.at(*ix));
			if f.matches(&current, &candidate) {
				indexes.push(*ix);
			}
		}
		self.indexes = indexes;

		let mut r = Ranker::new();
		for ix in &self.indexes {
			r.insert(self.words.at(*ix));
		}

		self.indexes.sort_by_key(|ix| {
			let w = self.words.at(*ix);
			(Reverse(r.score(w)), w.to_owned())
		});
		!self.indexes.is_empty()
	}
}

struct Ranker {
	counts: [u32; 26 * 5],
}

impl Ranker {
	fn new() -> Ranker {
		Ranker {
			counts: [0; 26 * 5],
		}
	}

	fn insert(&mut self, w: &str) {
		for (i, c) in w.char_indices() {
			let ix = i * 26 + (c as usize - 'a' as usize);
			self.counts[ix] += 1;
		}
	}

	fn score(&self, w: &str) -> u32 {
		let mut tally = vec![0; 26];
		for (i, c) in w.char_indices() {
			let ord = c as usize - 'a' as usize;
			let ix = i * 26 + ord;
			tally[ord] = tally[ord].max(self.counts[ix]);
		}
		tally.iter().sum()
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-exp")
		.arg(
			clap::Arg::new("words")
				.long("words")
				.takes_value(true)
				.default_value("wordle.txt")
				.help("word list file"),
		)
		.arg(
			clap::Arg::new("verbose")
				.long("verbose")
				.short('v')
				.takes_value(false)
				.help("should verbose output be shown?"),
		)
		.arg(
			clap::Arg::new("solutions")
				.long("solution")
				.takes_value(true)
				.multiple_occurrences(true)
				.help("solutions to focus in on (enables verbose)"),
		)
		.get_matches();

	let to_show = match matches.values_of("solutions") {
		Some(vals) => vals.map(|v| v.to_owned()).collect::<HashSet<_>>(),
		None => HashSet::new(),
	};

	let verbose = matches.is_present("verbose") || !to_show.is_empty();
	let mut stats = Vec::new();
	let word_list = WordList::read(matches.value_of("words").unwrap())?;
	for solution in word_list.words() {
		let s = Solution::find(&word_list, &solution).unwrap();
		if verbose && (to_show.is_empty() || to_show.contains(solution))
			|| s.number_of_guesses() > 6
		{
			println!("{}", solution.to_uppercase());
			s.emit();
			println!();
		}
		stats.push(s.number_of_guesses());
	}
	report_stats(&mut stats);
	Ok(())
}
