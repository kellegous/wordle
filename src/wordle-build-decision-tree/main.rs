use num_format::{Locale, ToFormattedString};
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use wordle::arg;
use wordle::{decision_tree, words_from_file, Word};

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-build-decision-tree")
		.arg(arg::for_solutions_file())
		.arg(arg::for_guesses_file())
		.arg(arg::for_decision_tree_file())
		.get_matches();

	let solutions = words_from_file(matches.value_of(arg::SOLUTIONS_FILE).unwrap())?
		.into_iter()
		.collect::<HashSet<Word>>();
	let mut guesses = words_from_file(matches.value_of(arg::GUESSES_FILE).unwrap())?;
	println!(
		"guesses: {}, solutions: {}",
		guesses.len().to_formatted_string(&Locale::en),
		solutions.len().to_formatted_string(&Locale::en)
	);

	let node = decision_tree::Node::build(&mut guesses, &solutions, 0);

	serde_json::to_writer(
		fs::File::create(matches.value_of(arg::DECISION_TREE_FILE).unwrap())?,
		&node.unwrap(),
	)?;
	Ok(())
}
