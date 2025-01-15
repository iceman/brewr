//! # Brewr
//!
//! A command line utility to improve interaction with brew

mod command_line;
mod diagnostics;
use brewr::*;

fn main() {
	let args = command_line::Args::get();
	config::insert(config::GRID, args.grid);

	run_diagnostic();

	if args.leaves {
		print_desc_for_leaves();
	} else if args.all {
		print_desc_for_all_installed();
	} else {
		print_output_with_new_item_desc();
	}
}

fn run_diagnostic() {
	diagnostics::error_if_not_in_path(&["brew", "bash"]);
}
