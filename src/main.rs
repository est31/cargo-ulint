extern crate structopt;
#[macro_use]
extern crate structopt_derive;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(about = "unused lint checker")]
struct Options {
	/// Override for the ulint toml
	#[structopt(help = "Path to ulint.toml")]
	toml: Option<String>,
}

fn main() {
	let Options = Options::from_args();
	// TODO
}
