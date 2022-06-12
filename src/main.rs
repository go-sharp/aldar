// Copyright © 2022 The Aldar Authors
//
// Use of this source code is governed by an BSD-style
// license that can be found in the LICENSE file.

mod aldar;
mod fsutil;

use clap::Parser;
use colored::*;
use std::fs::File;
use std::process;

use crate::aldar::Aldar;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(short = 'a', long = "all", help = "List also hidden files")]
    all_files: bool,

    #[clap(short = 'd', long = "dirs-only", help = "List directories only")]
    dir_only: bool,

    #[clap(
        short = 'E',
        long = "exclude-pattern",
        help = "Do not list files that match the given pattern"
    )]
    exclude_pattern: Option<Vec<String>>,

    #[clap(
        short = 'I',
        long = "include-pattern",
        help = "List only those files that match the pattern given"
    )]
    include_pattern: Option<Vec<String>>,

    #[clap(
        short = 'i',
        long = "ignore-case",
        help = "Ignore case when pattern matching"
    )]
    ignore_case: bool,

    #[clap(
        short = 'L',
        long = "level",
        help = "Descend only level directories deep"
    )]
    level: Option<i32>,

    #[clap(
        short = 'f',
        long = "fullpath",
        help = "Print the full path prefix for each file"
    )]
    print_fullpath: bool,

    /// Print the size in a more human readable way
    #[clap(short = 'H', long)]
    human_readable: bool,

    /// Print the size in bytes of each file
    #[clap(short = 's', long)]
    size: bool,

    /// Print non-printable characters as '?'
    #[clap(short = 'q', long)]
    replace_nonprintable: bool,

    /// Output to file instead of stdout
    #[clap(short = 'o', long)]
    output: Option<String>,

    /// Print ASCII only indentation lines
    #[clap(short = 'A', long)]
    ascii: bool,

    /// Turn colorization off
    #[clap(short = 'n', long)]
    no_colors: bool,

    /// Working directory of this command (Default: Current directory)
    path: Option<String>,
}

fn main() {
    let args: Args = Args::parse();

    // Disable color if specified or a file is used as output
    if args.no_colors || args.output.is_some() {
        colored::control::set_override(false);
    }

    let mut a = Aldar::new();
    let aldar = a
        .use_path(args.path.unwrap_or_else(|| ".".to_string()))
        .show_hidden(args.all_files)
        .show_dirs_only(args.dir_only)
        .case_sensitive(args.ignore_case)
        .use_glyphset(match args.ascii {
            true => Box::new(aldar::ASCII_GLYPHSET),
            false => Box::new(aldar::UNICODE_GLYPHSET),
        })
        .use_max_level(args.level.unwrap_or_else(|| -1))
        .show_fullpath(args.print_fullpath)
        .show_size(args.size)
        .show_human_readable(args.human_readable)
        .do_replace_nonprintable_chars(args.replace_nonprintable);

    let error_str = "Error:".red();

    if let Some(output) = args.output {
        let result = File::create(output.clone());
        if result.is_err() {
            println!(
                "{} failed to open file {}: {}",
                error_str,
                output,
                result.unwrap_err()
            );
            process::exit(1);
        }

        aldar.use_writer(Box::new(result.unwrap()));
    }

    if let Some(pattern) = args.include_pattern {
        let v: Vec<&str> = pattern.iter().map(String::as_ref).collect();
        aldar.set_include_patterns(&v);
    }

    if let Some(pattern) = args.exclude_pattern {
        let v: Vec<&str> = pattern.iter().map(String::as_ref).collect();
        aldar.set_exclude_patterns(&v);
    }

    if let Err(e) = aldar.run() {
        println!("{} {}", error_str, e);
    }
}
