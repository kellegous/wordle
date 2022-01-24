use super::{Char, Directive, Guess, Solution, Word, Words};
use std::cmp::Ordering;

const LENS: &[usize] = &[
	0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2, 3, 2, 3, 3, 4, 1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5,
	1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
	1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
	2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
	1, 2, 2, 3, 2, 3, 3, 4, 2, 3, 3, 4, 3, 4, 4, 5, 2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6,
	2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
	2, 3, 3, 4, 3, 4, 4, 5, 3, 4, 4, 5, 4, 5, 5, 6, 3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7,
	3, 4, 4, 5, 4, 5, 5, 6, 4, 5, 5, 6, 5, 6, 6, 7, 4, 5, 5, 6, 5, 6, 6, 7, 5, 6, 6, 7, 6, 7, 7, 8,
];

#[derive(Copy, Clone)]
struct CharSet {
	chars: u32,
}

impl CharSet {
	fn empty() -> CharSet {
		CharSet { chars: 0 }
	}

	fn with_all() -> CharSet {
		let mut s = CharSet::empty();
		for i in 0..26 {
			s.insert(Char::from_u8(i));
		}
		s
	}

	#[allow(dead_code)]
	fn len(&self) -> usize {
		let c = self.chars as usize;
		LENS[c & 0xff] + LENS[c >> 8 & 0xff] + LENS[c >> 16 & 0xff] + LENS[c >> 24 & 0xff]
	}

	fn insert(&mut self, c: Char) -> bool {
		let m = 1 << c.ord();
		let chars = self.chars;
		self.chars |= m;
		chars != self.chars
	}

	fn remove(&mut self, c: Char) -> bool {
		let m = 1 << c.ord();
		let chars = self.chars;
		self.chars &= !m;
		chars != self.chars
	}

	fn contains(&self, c: Char) -> bool {
		let m = 1 << c.ord();
		self.chars & m != 0
	}

	fn clear(&mut self) {
		self.chars = 0;
	}

	fn iter(&self) -> impl Iterator<Item = Char> + '_ {
		(0..26)
			.map(|i| Char::from_u8(i as u8))
			.filter(|c| self.contains(*c))
	}
}

struct Entry<'a, V> {
	v: &'a mut Option<V>,
}

impl<'a, V> Entry<'a, V> {
	fn or_insert(&mut self, default: V) -> &mut V {
		if let None = self.v {
			*self.v = Some(default);
		}
		self.v.as_mut().unwrap()
	}
}

#[derive(Debug)]
struct CharMap<T> {
	values: [Option<T>; 26],
}

impl<T> CharMap<T> {
	fn new() -> CharMap<T> {
		CharMap {
			values: Default::default(),
		}
	}

	fn get_mut(&mut self, key: Char) -> &mut Option<T> {
		&mut self.values[key.ord() as usize]
	}

	fn get(&self, key: Char) -> Option<&T> {
		self.values[key.ord() as usize].as_ref()
	}

	fn entry(&mut self, key: Char) -> Entry<'_, T> {
		Entry {
			v: self.get_mut(key),
		}
	}
}

struct Constraints {
	positions: [CharSet; 5],
	must_have: CharSet,
}

impl Constraints {
	fn new() -> Constraints {
		Constraints {
			positions: [CharSet::with_all(); 5],
			must_have: CharSet::empty(),
		}
	}

	fn add(&mut self, guess: &Guess) {
		for (i, (d, c)) in guess.iter().enumerate() {
			match d {
				Directive::Green => {
					self.positions[i].clear();
					self.positions[i].insert(c);
				}
				Directive::Yellow => {
					self.positions[i].remove(c);
					self.must_have.insert(c);
				}
				Directive::Gray => {
					for pos in &mut self.positions {
						pos.remove(c);
					}
				}
			}
		}
	}

	fn is_satisfied_by(&self, word: &Word) -> bool {
		for (i, c) in word.chars().iter().enumerate() {
			if !self.positions[i].contains(*c) {
				return false;
			}
		}

		for c in self.must_have.iter() {
			if !word.contains(c) {
				return false;
			}
		}

		true
	}

	fn unique_chars(&self) -> CharSet {
		let (max_index, _) = self
			.positions
			.iter()
			.map(|s| s.len())
			.enumerate()
			.max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
			.unwrap();
		self.positions[max_index].clone()
	}
}

impl std::fmt::Display for Constraints {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let pos = self
			.positions
			.iter()
			.map(|s| s.iter().map(|c| c.char()).collect::<String>())
			.collect::<Vec<String>>();
		write!(f, "[{}]", pos.join(", "))
	}
}

fn score(word: &Word, freq: &CharMap<usize>, s: &CharSet) -> f64 {
	let mut chars = CharSet::empty();
	for c in word.chars() {
		chars.insert(*c);
	}

	chars
		.iter()
		.map(|c| {
			if s.contains(c) {
				// freq.get(c).unwrap_or(&0)
				// *freq.get(c).unwrap_or(&1) as f64 / 2315.0
				1.0
			} else {
				0.0
			}
		})
		.sum()
}

fn rank(words: &mut Words, freq: &CharMap<usize>, constraints: &Constraints) {
	let chars = constraints.unique_chars();
	words.rank(|w| score(w, freq, &chars));
}

pub fn solve(words: &Words, solution: &Word) -> Option<Solution> {
	let mut counts = CharMap::new();
	for word in words.words() {
		for c in word.chars() {
			*counts.entry(*c).or_insert(0) += 1;
		}
	}

	let mut guesses = Vec::new();
	let mut constraints = Constraints::new();

	let mut candidates = words.clone();

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

		constraints.add(&guess);
		rank(&mut candidates, &counts, &constraints);
		candidates = candidates.filter_into(|w| constraints.is_satisfied_by(w));
	}

	Some(Solution::new(guesses))
}

#[cfg(test)]
mod tests {
	use super::{Char, CharSet};

	#[test]
	fn test_charset_len() {
		let mut s = CharSet::empty();
		for i in 0..26 {
			assert_eq!(s.len(), i);
			s.insert(Char::from_u8(i as u8));
			assert_eq!(s.len(), i + 1);
		}
	}
}
