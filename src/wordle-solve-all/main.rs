use num_format::{Locale, ToFormattedString};
use std::collections::HashSet;
use std::error::Error;
use wordle::{Filter, WordList};

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
	fn find(words: &WordList, solution: &str) -> Solution {
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
			c.apply(&filter);
		}
		Solution { guesses }
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

fn report_stats(stats: &mut [usize]) {
	stats.sort();
	println!(
		"Total:           {}",
		stats.len().to_formatted_string(&Locale::en)
	);
	println!(
		"Median Guesses:  {}",
		stats[stats.len() / 2].to_formatted_string(&Locale::en)
	);
	println!(
		"Max Guesses:     {}",
		stats[stats.len() - 1].to_formatted_string(&Locale::en)
	);
	println!(
		"Avg Guesses:     {:0.1}",
		stats.iter().sum::<usize>() as f64 / stats.len() as f64
	);
	println!(
		"Percent Success: {:0.1}%",
		100.0 * stats.iter().filter(|g| **g <= 6).count() as f64 / stats.len() as f64
	);
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
		let s = Solution::find(&word_list, &solution);
		if verbose && (to_show.is_empty() || to_show.contains(solution)) {
			println!("{}", solution.to_uppercase());
			s.emit();
			println!();
		}
		stats.push(s.number_of_guesses());
	}

	report_stats(&mut stats);
	Ok(())
}
