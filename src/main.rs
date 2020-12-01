use clap::App;
use clap::Arg;

mod common;
mod crate_info;
mod days;

use crate::crate_info::crate_author;
use crate::crate_info::crate_description;
use crate::crate_info::crate_name;
use crate::crate_info::crate_version;

use crate::common::day_input_filename;
use crate::common::get_file_lines;

fn main() -> Result<(), std::io::Error> {
	let cli = App::new(crate_name())
		.version(crate_version())
		.about(crate_description())
		.author(crate_author())
		.arg(
			Arg::with_name("day")
				.takes_value(true)
				.short("d")
				.long("day")
				.help(r#"Day number (1 - 25) to run. If omitted, no days are run."#)
		);

	let matches = cli.get_matches();

	if let Some(day) = matches.value_of("day") {
		run_day(
			day.parse::<u8>()
					.unwrap_or_else(|_| panic!(format!("Invalid day number: {}", day)))
		)
	}
	else {
		Ok(())
	}
}

fn run_day(day: u8) -> Result<(), std::io::Error> {
	println!();
	println!("=== Day {: >2} ===", day);

	let day_func = days::get_solver(day).unwrap_or_else(|| panic!(format!("Unknown day: {}", day)));
	let lines = get_file_lines(&day_input_filename(day)).unwrap();
	let solution = day_func(&lines);

	println!("A: {}", solution.0);
	println!("B: {}", solution.1);

	Ok(())
}
