use std::collections::HashSet;
use std::error::Error;
use wordle::{report_stats, Guess, Word, Words};

struct Solution {
	guesses: Vec<Guess>,
}

impl Solution {
	fn find(words: &Words, solution: &Word) -> Option<Solution> {
		let mut guesses = Vec::new();

		let word = match words.first() {
			Some(w) => w,
			None => return None,
		};

		let guess = Guess::new(&word, solution);
		guesses.push(guess.clone());
		if guess.is_all_green() {
			return Some(Solution { guesses });
		}

		let mut candidates = words.filter(|w| guess.matches(w));

		loop {
			let word = match candidates.first() {
				Some(w) => w,
				None => return None,
			};

			let guess = Guess::new(&word, solution);
			guesses.push(guess.clone());
			if guess.is_all_green() {
				break;
			}

			candidates = candidates.filter_into(|w| guess.matches(w));
		}

		Some(Solution { guesses })
	}

	fn number_of_guesses(&self) -> usize {
		self.guesses.len()
	}

	fn emit(&self) {
		for guess in self.guesses.iter() {
			println!("{}", guess)
		}
	}
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-solve-all")
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
		Some(vals) => vals
			.map(|v| Word::from_str(v))
			.collect::<Result<HashSet<_>, _>>()?,
		None => HashSet::new(),
	};

	let verbose = matches.is_present("verbose") || !to_show.is_empty();
	let mut stats = Vec::new();
	let words = Words::from_file(matches.value_of("words").unwrap())?;
	for solution in words.words() {
		let s = Solution::find(&words, &solution).unwrap();
		if verbose && (to_show.is_empty() || to_show.contains(solution))
			|| s.number_of_guesses() > 6
		{
			println!("{}", solution.to_string());
			s.emit();
			println!();
		}

		stats.push(s.number_of_guesses());
	}

	report_stats(&mut stats);
	Ok(())
}
