use super::{Guess, Solution, Word, Words};

pub fn solve(words: &Words, solution: &Word) -> Option<Solution> {
	let mut guesses = Vec::new();

	let word = match words.first() {
		Some(w) => w,
		None => return None,
	};

	let guess = Guess::new(&word, solution);
	guesses.push(guess.clone());
	if guess.is_all_green() {
		return Some(Solution::new(guesses));
	}

	let mut candidates = words.filter(|w| guess.matches(w));

	loop {
		let word = match candidates.first() {
			Some(w) => w,
			None => return None,
		};

		let guess = Guess::new(&word, solution);
		guesses.push(guess.clone());
		if guess.is_all_green() {
			break;
		}

		candidates = candidates.filter_into(|w| guess.matches(w));
	}

	Some(Solution::new(guesses))
}
