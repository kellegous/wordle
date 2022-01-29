pub const SOLUTIONS_FILE: &str = "solutions-file";

pub const GUESSES_FILE: &str = "guesses-file";

pub const DECISION_TREE_FILE: &str = "decision-tree-file";

pub const VERBOSE: &str = "verbose";

pub fn for_solutions_file<'a>() -> clap::Arg<'a> {
	clap::Arg::new(SOLUTIONS_FILE)
		.short('s')
		.long(SOLUTIONS_FILE)
		.takes_value(true)
		.default_value("solutions")
		.help("file containing all possible solution words")
}

pub fn for_guesses_file<'a>() -> clap::Arg<'a> {
	clap::Arg::new("guesses-file")
		.short('g')
		.long("guesses-file")
		.takes_value(true)
		.default_value("words")
		.help("file containing all words that can be used as a guess")
}

pub fn for_decision_tree_file<'a>() -> clap::Arg<'a> {
	clap::Arg::new("decision-tree-file")
		.short('t')
		.long("decision-tree-file")
		.takes_value(true)
		.default_value("decision-tree.json")
		.help("json file containing the decision tre")
}

pub fn for_verbose<'a>() -> clap::Arg<'a> {
	clap::Arg::new("verbose")
		.long("verbose")
		.short('v')
		.takes_value(false)
		.help("should verbose output be shown?")
}
