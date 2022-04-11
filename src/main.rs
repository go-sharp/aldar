// Copyright © 2018 The Aldar Authors
//
// Use of this source code is governed by an BSD-style
// license that can be found in the LICENSE file.

mod aldar;

use clap::Parser;
use colored::*;
use std::{fs::read_dir, io, io::Write};



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
    // colored::control::set_override(false);
    println!("{} Hello, world!", "Error:".red());
    let args = Args::parse();
    println!("{:?}", args);

    
    print!("{:?} -> {:?}", aldar::ASCII_GLYPHSET, aldar::UNICODE_GLYPHSET);

    let iter = read_dir(".").unwrap();

    for item in iter {
        println!("-> {:?}, {:?}", item.as_ref().unwrap().path(), item.unwrap().file_type().unwrap().is_dir())
    }

    let mut stdout = io::stdout();
    write!(stdout, "Hello World");

    let mut myvec: Vec<String> = Vec::new();
    myvec.push("value".to_owned());

    println!(">>>>> {:?}", myvec);


}
