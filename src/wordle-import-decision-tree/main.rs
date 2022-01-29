use std::error::Error;
use std::fs;
use wordle::decision_tree;

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-import-decision-tree")
		.arg(
			clap::Arg::new("decision-tree-file")
				.short('t')
				.long("decision-tree-file")
				.takes_value(true)
				.default_value("decision-tree.json")
				.help("where to write the file containing the decision tree"),
		)
		.arg(
			clap::Arg::new("strategy-url")
				.short('s')
				.long("strategy-url")
				.takes_value(true)
				.default_value(
					"http://sonorouschocolate.com/notes/images/0/0e/Optimaltree.hardmode5.txt",
				)
				.help("the URL to read the strategy from"),
		)
		.get_matches();

	let tree = decision_tree::from_reader(reqwest::blocking::get(
		matches.value_of("strategy-url").unwrap(),
	)?)?;

	serde_json::to_writer(
		fs::File::create(matches.value_of("decision-tree-file").unwrap())?,
		&tree,
	)?;

	Ok(())
}
