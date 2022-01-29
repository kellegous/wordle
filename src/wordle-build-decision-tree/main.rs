use std::error::Error;
use wordle::arg;
use wordle::words_from_file;

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-build-decision-tree")
		.arg(arg::for_solutions_file())
		.arg(arg::for_guesses_file())
		.arg(arg::for_decision_tree_file())
		.get_matches();

	let solutions = words_from_file(matches.value_of(arg::SOLUTIONS_FILE).unwrap())?;
	let guesses = words_from_file(matches.value_of(arg::GUESSES_FILE).unwrap())?;
	println!("guesses: {}, solutions: {}", guesses.len(), solutions.len());
	Ok(())
}
