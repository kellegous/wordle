use std::collections::HashMap;
use std::error::Error;
use wordle::{Feedback, Word, Words, WORD_SIZE};

const LENS: &[usize] = &[
	0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
];

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Difference {
	m: u8,
}

impl Difference {
	fn from(a: &Word, b: &Word) -> Difference {
		let m = (0..WORD_SIZE).fold(0, |m, i| {
			if a[i] == b[i] {
				let v = 1 << i;
				m | v
			} else {
				m
			}
		});
		Difference { m }
	}

	fn at(&self, i: usize) -> bool {
		self.m & (1 << i) != 0
	}

	fn number_different(&self) -> usize {
		5 - LENS[self.m as usize]
	}

	fn number_same(&self) -> usize {
		LENS[self.m as usize]
	}
}

impl std::fmt::Display for Difference {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let m = (0..WORD_SIZE)
			.map(|i| if self.at(i) { '*' } else { 'x' })
			.collect::<String>();
		write!(f, "{}", m)
	}
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

	let mut with_scores = Vec::new();

	for (i, a) in words.words().iter().enumerate() {
		let mut counts = HashMap::new();
		for (j, b) in words.words().iter().enumerate() {
			if i == j {
				continue;
			}

			let diff = Difference::from(a, b);
			if diff.number_different() > 1 {
				continue;
			}

			*counts.entry(diff).or_insert(0) += 1;
		}

		let max = *counts.values().max().unwrap_or(&0);
		with_scores.push((*a, max));
	}

	with_scores.sort_by(|(_, a), (_, b)| b.cmp(a));
	for (word, s) in with_scores.iter() {
		println!("{}: {}", word, s);
	}

	Ok(())
}

#[cfg(test)]
mod test {
	use super::{Difference, Word};

	#[test]
	fn compute_difference() {
		assert_eq!(
			Difference::from(
				&Word::from_str("aaaaa").unwrap(),
				&Word::from_str("aaaab").unwrap()
			),
			Difference { m: 0x0f }
		);
		assert_eq!(
			Difference::from(
				&Word::from_str("aaaaa").unwrap(),
				&Word::from_str("bbbbb").unwrap()
			),
			Difference { m: 0x0 }
		);
		assert_eq!(
			Difference::from(
				&Word::from_str("aaaaa").unwrap(),
				&Word::from_str("aaaaa").unwrap()
			),
			Difference { m: 0x1f }
		);
	}
}
