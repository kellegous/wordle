use std::process::Command;
fn main() {
	let output = Command::new("git")
		.args(&["rev-parse", "HEAD"])
		.output()
		.unwrap();
	let sha = String::from_utf8(output.stdout).unwrap();
	println!("cargo:rustc-env=BUILD_SHA={}", sha);
}
