use std::error::Error;
use std::fs;
use std::io::{BufReader, BufWriter};
use wordle::decision_tree;

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-strategy-to-json")
		.arg(
			clap::Arg::new("strategy-file")
				.takes_value(true)
				.required(true)
				.help("the strategy file to parse"),
		)
		.arg(
			clap::Arg::new("dst")
				.long("dst")
				.short('o')
				.takes_value(true)
				.default_value("tree.json")
				.help("the destination file"),
		)
		.get_matches();

	let root = decision_tree::from_stategy(BufReader::new(fs::File::open(
		matches.value_of("strategy-file").unwrap(),
	)?))?;

	serde_json::to_writer(
		BufWriter::new(fs::File::create(matches.value_of("dst").unwrap())?),
		&root,
	)?;

	Ok(())
}
