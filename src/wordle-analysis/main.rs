use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Write;
use wordle::{Feedback, Word, Words};

fn score(word: &Word, words: &Words) -> f64 {
	let mut counts = HashMap::new();
	for w in words.words() {
		if w == word {
			continue;
		}
		let feedback = Feedback::from_word(w, word);
		*counts.entry(feedback).or_insert(0) += 1;
	}
	counts.values().sum::<usize>() as f64 / counts.len() as f64
}

fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-analysis")
		.arg(
			clap::Arg::new("words")
				.long("words")
				.takes_value(true)
				.default_value("words")
				.help("word list file"),
		)
		.get_matches();

	let words = Words::from_file(matches.value_of("words").unwrap())?;
	let mut scores = words
		.words()
		.iter()
		.map(|w| (w, score(w, &words)))
		.collect::<Vec<_>>();
	scores.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap());

	let mut w = fs::File::create("wordle.txt")?;
	for (word, _) in scores.iter() {
		writeln!(&mut w, "{}", word.to_string())?;
	}

	Ok(())
}
