use std::error::Error;
use wordle::{Filter, WordList};

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-solve")
		.arg(
			clap::Arg::new("words")
				.long("words")
				.takes_value(true)
				.default_value("wordle.txt")
				.help("word list file"),
		)
		.arg(
			clap::Arg::new("filters")
				.takes_value(true)
				.multiple_occurrences(true)
				.help("filters to apply"),
		)
		.get_matches();

	let wl = WordList::read(matches.value_of("words").unwrap())?;
	let filters = match matches.values_of("filters") {
		Some(vals) => vals
			.map(|s| Filter::from_str(s))
			.collect::<Result<Vec<_>, _>>()?,
		None => Vec::new(),
	};

	let mut c = wl.first();
	for filter in filters {
		if !c.apply(&filter) {
			break;
		}
	}
	println!("{}", c.word());
	Ok(())
}
