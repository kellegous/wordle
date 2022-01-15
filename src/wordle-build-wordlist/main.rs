use std::cmp::Reverse;
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn read_words_from<P: AsRef<Path>>(src: P, n: usize) -> io::Result<HashSet<String>> {
	let mut words = HashSet::new();
	for line in BufReader::new(fs::File::open(src)?).lines() {
		let line = line?;
		if line.len() == n {
			words.insert(line.to_lowercase());
		}
	}
	Ok(words)
}

fn rank(words: &HashSet<String>) -> Vec<(&String, u32)> {
	let mut f = vec![0; 26];
	for word in words {
		for c in word.chars() {
			f[c as usize - 'a' as usize] += 1;
		}
	}

	let mut ranked = words
		.iter()
		.map(|w| {
			let s = w
				.chars()
				.collect::<HashSet<_>>()
				.iter()
				.fold(0, |s, c| s + f[*c as usize - 'a' as usize]);
			(w, s)
		})
		.collect::<Vec<_>>();
	ranked.sort_by_key(|(w, s)| (Reverse(*s), *w));
	ranked
}
fn main() -> Result<(), Box<dyn Error>> {
	let matches = clap::App::new("wordle-build-wordlist")
		.arg(
			clap::Arg::new("src")
				.long("src")
				.takes_value(true)
				.default_value("/usr/share/dict/words")
				.help("source dictionary"),
		)
		.arg(
			clap::Arg::new("dst")
				.long("dst")
				.takes_value(true)
				.default_value("wordle.txt")
				.help("destination for ranked wordlist"),
		)
		.get_matches();

	let words = read_words_from(matches.value_of("src").unwrap(), 5)?;
	let words = rank(&words);
	let mut w = fs::File::create(matches.value_of("dst").unwrap())?;
	for (word, _) in &words {
		writeln!(w, "{}", word)?;
	}
	Ok(())
}
