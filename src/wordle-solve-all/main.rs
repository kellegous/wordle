use std::collections::HashSet;
use std::error::Error;
use wordle::{solve_all, Strategy, Word, Words};

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
		.arg(
			clap::Arg::new("strategy")
				.long("strategy")
				.short('s')
				.takes_value(true)
				.default_value("a")
				.help("strategy to use to solve"),
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
	let strategy = Strategy::from_str(matches.value_of("strategy").unwrap())?;
	let stats = solve_all(
		&words,
		|words, word| strategy.solve(words, word),
		|word, solution| {
			verbose && (to_show.is_empty() || to_show.contains(word))
				|| solution.number_of_guesses() > 6
		},
	)?;

	stats.report();
	Ok(())
}
