use chrono::prelude::*;
use chrono::{Duration, NaiveDate};
use std::error::Error;
use std::fs;
use std::io::{BufRead, BufReader};
use wordle::arg;

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-list-solutions")
		.arg(arg::for_solutions_file())
		.arg(
			clap::Arg::new("n")
				.long("n")
				.takes_value(true)
				.default_value("100")
				.help("number of solutions"),
		)
		.get_matches();

	let words = BufReader::new(fs::File::open(
		matches.value_of(arg::SOLUTIONS_FILE).unwrap(),
	)?)
	.lines()
	.collect::<Result<Vec<_>, _>>()?;
	let n = matches.value_of("n").unwrap().parse::<usize>()?;

	let start = NaiveDate::from_ymd(2021, 6, 19);
	let today = Local::now().date().naive_local();
	for i in 0..n {
		let day = today + Duration::days(i as i64);
		let num = day.signed_duration_since(start).num_days();
		let word = words[num as usize % words.len()].to_uppercase();
		println!("{}\t#{}\t{}", day, num, word);
	}
	Ok(())
}
