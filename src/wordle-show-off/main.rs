use chrono::{Duration, NaiveDate};
use std::error::Error;
use std::fs;
use std::io::BufReader;
use wordle::{arg, decision_tree, words_from_file, Guess, Word};

fn solve(tree: &decision_tree::Node, solution: &Word) -> Result<Vec<Guess>, Box<dyn Error>> {
	let mut node = tree;
	let mut guesses = Vec::new();
	loop {
		let guess = Guess::from_word(node.word(), solution);
		guesses.push(guess.clone());
		if guess.word() == solution {
			break;
		}

		node = match node.next(guess.feedback()) {
			Some(n) => n,
			None => return Err(format!("no solution for {}", solution).into()),
		};
	}

	Ok(guesses)
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("world-show-off")
		.arg(arg::for_decision_tree_file())
		.arg(arg::for_solutions_file().default_value("nyt.solutions"))
		.get_matches();

	let solutions = words_from_file(matches.value_of(arg::SOLUTIONS_FILE).unwrap())?;
	let tree: decision_tree::Node = serde_json::from_reader(BufReader::new(fs::File::open(
		matches.value_of(arg::DECISION_TREE_FILE).unwrap(),
	)?))?;

	let start = NaiveDate::from_ymd(2021, 6, 19);
	for (i, solution) in solutions.iter().enumerate() {
		// if i < 264 {
		// 	continue;
		// }
		let day = start + Duration::days(i as i64);
		let guesses = solve(&tree, &solution)?;
		println!("{}", day.format("%m/%d/%Y"));
		println!("Wordle {} {}/6*", i, guesses.len());
		for guess in &guesses {
			println!("{}", guess.feedback());
		}
		println!();
	}

	Ok(())
}
