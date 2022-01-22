use num_format::{Locale, ToFormattedString};
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use wordle::{Guess, Word, Words};

struct Stats {
	num_guesses: Vec<usize>,
	finished: bool,
}

impl Stats {
	fn new(n: usize) -> Stats {
		Stats {
			num_guesses: Vec::with_capacity(n),
			finished: false,
		}
	}

	fn record(&mut self, solution: &Solution) {
		self.num_guesses.push(solution.number_of_guesses());
	}

	fn finish(&mut self) {
		self.num_guesses.sort();
		self.finished = true;
	}

	fn report(&self) {
		// TODO(knorton): This is dumb.
		if !self.finished {
			panic!("Stats must be finished");
		}

		if self.num_guesses.is_empty() {
			return;
		}

		let n = self.num_guesses.len();

		let max = self.num_guesses[n - 1];
		let mut hist = HashMap::new();
		for n in self.num_guesses.iter() {
			*hist.entry(n).or_insert(0) += 1;
		}
		println!("Total:           {}", n.to_formatted_string(&Locale::en));
		println!(
			"Median Guesses:  {}",
			self.num_guesses[n / 2].to_formatted_string(&Locale::en)
		);
		println!("Max Guesses:     {}", max.to_formatted_string(&Locale::en));
		println!(
			"Avg Guesses:     {:0.1}",
			self.num_guesses.iter().sum::<usize>() as f64 / n as f64
		);
		let failed = self.num_guesses.iter().filter(|g| **g > 6).count();
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
			let bar = std::iter::repeat("#").take(w as usize).collect::<String>();
			println!(
				"{:2}: #{} {} ({:0.1}%)",
				i,
				bar,
				v.to_formatted_string(&Locale::en),
				100.0 * v as f64 / n as f64
			);
		}
	}
}

pub struct Solution {
	guesses: Vec<Guess>,
}

impl Solution {
	fn new(guesses: Vec<Guess>) -> Solution {
		Solution { guesses }
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

mod a {
	use super::{Guess, Solution, Word, Words};
	pub fn solve(words: &Words, solution: &Word) -> Option<Solution> {
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

		Some(Solution::new(guesses))
	}
}

fn solve_all<S, P>(words: &Words, solve_fn: S, print_fn: P) -> Result<Stats, Box<dyn Error>>
where
	S: Fn(&Words, &Word) -> Option<Solution>,
	P: Fn(&Word, &Solution) -> bool,
{
	let mut stats = Stats::new(words.words().len());
	for solution in words.words() {
		let solved = solve_fn(words, &solution).ok_or("no solution")?;
		if print_fn(&solution, &solved) {
			println!("{}", solution.to_string().to_uppercase());
			solved.emit();
			println!();
		}
		stats.record(&solved);
	}
	stats.finish();
	Ok(stats)
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
	let words = Words::from_file(matches.value_of("words").unwrap())?;

	let stats = solve_all(&words, a::solve, |word, solution| {
		verbose && (to_show.is_empty() || to_show.contains(word))
			|| solution.number_of_guesses() > 6
	})?;

	stats.report();
	Ok(())
}
