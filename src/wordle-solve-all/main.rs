use std::collections::HashSet;
use std::error::Error;
use wordle::{report_stats, Filter, WordList};

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
		let mut c = words.first();
		let mut guesses = Vec::new();
		loop {
			let word = c.word();
			let filter = Filter::from_guess(word, solution);
			guesses.push(Guess {
				word: word.to_owned(),
				filter: filter.clone(),
			});
			if filter.all_green() {
				break;
			}
			if !c.apply(&filter) {
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
