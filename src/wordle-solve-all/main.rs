use std::error::Error;
use std::fs;
use std::io::BufReader;
use wordle::{decision_tree, report_stats, words_from_file, Guess};

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-solve-all")
		.arg(
			clap::Arg::new("solutions-file")
				.short('s')
				.long("solutions-file")
				.takes_value(true)
				.default_value("solutions")
				.help("file containing list of all possible solutions"),
		)
		.arg(
			clap::Arg::new("decision-tree-file")
				.short('t')
				.long("decision-tree-file")
				.takes_value(true)
				.default_value("decision-tree.json")
				.help("json file containing the decision tree"),
		)
		.arg(
			clap::Arg::new("verbose")
				.long("verbose")
				.short('v')
				.takes_value(false)
				.help("should verbose output be shown?"),
		)
		.get_matches();

	let solutions = words_from_file(matches.value_of("solutions-file").unwrap())?;
	let tree: decision_tree::Node = serde_json::from_reader(BufReader::new(fs::File::open(
		matches.value_of("decision-tree-file").unwrap(),
	)?))?;
	let verbose = matches.is_present("verbose");

	let mut stats = Vec::with_capacity(solutions.len());
	for solution in &solutions {
		let mut guesses = Vec::new();
		let mut node = &tree;
		loop {
			let guess = Guess::new(node.word(), solution);
			guesses.push(guess.clone());
			if guess.word() == solution {
				break;
			}

			node = node.next(guess.feedback()).unwrap(); // TODO(knorton): no solution found error.
		}

		if verbose {
			println!("{}", solution);
			for guess in &guesses {
				println!("{}", guess);
			}
			println!();
		}

		stats.push(guesses.len());
	}

	report_stats(stats);
	Ok(())
}
