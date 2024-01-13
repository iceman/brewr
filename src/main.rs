//! # Brewr
//!
//! A command line utility to improve interaction with brew

mod diagnostics;
mod args;
use args::{Args, Parser};
use brewr::config;

fn main() {
    let args = Args::parse();
    config::insert("grid".to_string(), args.grid);
    
    run_diagnostic();
    
    if args.leaves {
        brewr::print_desc_for_leaves();
    } else if args.all {
        brewr::print_desc_for_all_installed();
    } else {
        brewr::print_output_with_new_item_desc();
    }
}

fn run_diagnostic() {
    diagnostics::error_if_not_in_path(&[
        "brew",
        "bash",
    ]);
}