use std::error::Error;
use std::fs;
use std::io::BufReader;
use wordle::{decision_tree, Feedback};

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-solve")
		.arg(
			clap::Arg::new("decision-tree-file")
				.short('t')
				.long("decision-tree-file")
				.takes_value(true)
				.default_value("decision-tree.json")
				.help("json file containing the decision tree"),
		)
		.arg(
			clap::Arg::new("feedback")
				.takes_value(true)
				.multiple_occurrences(true)
				.help("feedback received on previous guesses"),
		)
		.get_matches();

	let tree: decision_tree::Node = serde_json::from_reader(BufReader::new(fs::File::open(
		matches.value_of("decision-tree-file").unwrap(),
	)?))?;

	let feedback = matches
		.values_of("feedback")
		.map(|vals| {
			vals.map(|s| Feedback::from_str(s))
				.collect::<Result<Vec<_>, _>>()
		})
		.unwrap_or_else(|| Ok(Vec::new()))?;

	let mut node = &tree;
	for f in &feedback {
		node = node.next(f).unwrap(); // TODO(knorton): no solution found error.
	}
	println!("{}", node.word());
	Ok(())
}
