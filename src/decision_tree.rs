use super::{Feedback, Guess, Word};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
	word: Word,
	#[serde(skip_serializing_if = "Option::is_none")]
	next: Option<HashMap<Feedback, Node>>,
}

impl Node {
	fn new(word: &Word) -> Node {
		Node {
			word: *word,
			next: None,
		}
	}

	fn add_guesses(&mut self, guesses: &[Guess]) {
		if guesses.is_empty() {
			return;
		}

		let guess = &guesses[0];
		self.next
			.get_or_insert_with(|| HashMap::new())
			.entry(*guess.feedback())
			.or_insert_with(|| Node::new(guess.word()))
			.add_guesses(&guesses[1..]);
	}

	pub fn next(&self, feedback: &Feedback) -> Option<&Node> {
		self.next.as_ref().and_then(|next| next.get(feedback))
	}

	pub fn word(&self) -> &Word {
		&self.word
	}
}

pub fn from_json_reader<R: io::Read>(r: R) -> Result<Node, serde_json::Error> {
	serde_json::from_reader(r)
}

fn parse_guesses(s: &str) -> Result<Vec<Guess>, Box<dyn Error>> {
	let n = (s.len() + 1) / 13;
	let mut path = Vec::with_capacity(n);
	for i in 0..n {
		let i = 12 * i + i;
		path.push(Guess::from_feedback_and_word(
			Feedback::from_str(&s[i..i + 5])?,
			Word::from_str(&s[i + 7..i + 7 + 5])?,
		));
	}
	Ok(path)
}

fn combine_line(prev: &str, curr: &str) -> String {
	let n = curr
		.char_indices()
		.find(|(_, c)| !c.is_ascii_whitespace())
		.map(|(i, _)| i)
		.unwrap_or(prev.len());
	format!("{}{}", &prev[0..n], &curr[n..])
}

pub fn from_reader<R: io::Read>(r: R) -> Result<Node, Box<dyn Error>> {
	let r = BufReader::new(r);
	let mut lines = r.lines();
	let (mut root, mut prev) = match lines.next() {
		Some(line) => {
			let line = line?;
			let mut node = Node::new(&Word::from_str(&line[..5])?);
			node.add_guesses(&parse_guesses(&line[6..line.len() - 7])?);
			(node, line)
		}
		None => return Err("empty file".into()),
	};

	for line in lines {
		let line = combine_line(&prev, &line?);
		root.add_guesses(&parse_guesses(&line[6..line.len() - 7])?);
		prev = line;
	}

	Ok(root)
}

pub fn from_strategy<R: io::BufRead>(r: R) -> Result<Node, Box<dyn Error>> {
	let mut lines = r.lines();

	let (mut root, mut prev) = match lines.next() {
		Some(line) => {
			let line = line?;
			let mut node = Node::new(&Word::from_str(&line[..5])?);
			node.add_guesses(&parse_guesses(&line[6..line.len() - 7])?);
			(node, line)
		}
		None => return Err("empty file".into()),
	};

	for line in lines {
		let line = combine_line(&prev, &line?);
		root.add_guesses(&parse_guesses(&line[6..line.len() - 7])?);
		prev = line;
	}

	Ok(root)
}
