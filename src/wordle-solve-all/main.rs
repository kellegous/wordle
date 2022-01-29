use std::error::Error;
use std::fs;
use std::io::BufReader;
use wordle::arg;
use wordle::{decision_tree, report_stats, words_from_file, Guess};

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-solve-all")
		.arg(arg::for_solutions_file())
		.arg(arg::for_decision_tree_file())
		.arg(arg::for_verbose())
		.get_matches();

	let solutions = words_from_file(matches.value_of(arg::SOLUTIONS_FILE).unwrap())?;
	let tree: decision_tree::Node = serde_json::from_reader(BufReader::new(fs::File::open(
		matches.value_of(arg::DECISION_TREE_FILE).unwrap(),
	)?))?;
	let verbose = matches.is_present(arg::VERBOSE);

	let mut stats = Vec::with_capacity(solutions.len());
	for solution in &solutions {
		let mut guesses = Vec::new();
		let mut node = &tree;
		loop {
			let guess = Guess::from_word(node.word(), solution);
			guesses.push(guess.clone());
			if guess.word() == solution {
				break;
			}

			node = node.next(guess.feedback()).unwrap(); // TODO(knorton): no solution found error.
		}

		if verbose {
			println!("{}", solution.to_uppercase_string());
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
