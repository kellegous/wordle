use std::error::Error;
use std::fs;
use std::io::prelude::*;
use std::io::{self, BufReader};
use wordle::arg;
use wordle::{decision_tree, Feedback};

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-solve")
		.arg(arg::for_decision_tree_file())
		.get_matches();

	let tree: decision_tree::Node = serde_json::from_reader(BufReader::new(fs::File::open(
		matches.value_of(arg::DECISION_TREE_FILE).unwrap(),
	)?))?;

	let mut node = &tree;
	let stdin = io::stdin();
	let mut line = String::new();
	loop {
		line.clear();

		print!("{} > ", node.word().to_uppercase_string());
		io::stdout().flush()?;

		if stdin.lock().read_line(&mut line)? == 0 {
			break;
		}

		let feedback = match Feedback::from_str(&line.trim_end()) {
			Ok(f) => {
				if f.is_all_green() {
					break;
				} else {
					f
				}
			}
			Err(_) => continue,
		};

		node = node.next(&feedback).unwrap();
	}

	Ok(())
}
