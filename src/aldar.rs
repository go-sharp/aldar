// Copyright © 2018 The Aldar Authors
//
// Use of this source code is governed by an BSD-style
// license that can be found in the LICENSE file.

use regex::{RegexSet, RegexSetBuilder};
use std::{
    env,
    path::PathBuf,
    io::{Write, stdout, self}
};

/// Represents a glyphset.
#[derive(Debug)]
pub struct GlyphSet(&'static str, &'static str, &'static str);

impl Glyphs for GlyphSet {
    fn pipe(&self) -> String {
        self.0.to_owned()
    }

    fn item(&self) -> String {
        self.2.to_owned()
    }

    fn last(&self) -> String {
        self.1.to_owned()
    }
}

/// Unicode glyphset uses unicode charachters.
pub const UNICODE_GLYPHSET: GlyphSet = GlyphSet("│", "└──", "├──");

/// Unicode glyphset uses unicode charachters.
pub const ASCII_GLYPHSET: GlyphSet = GlyphSet("|", "`--", "|--");

trait Glyphs {
    fn pipe(&self) -> String;
    fn item(&self) -> String;
    fn last(&self) -> String;
}


pub struct Aldar<'a> {
    show_hidden: bool,
    dir_only: bool,
    ignore_case: bool,
    match_dirs: bool,
    ascii_only: bool,
    level: i64,

    path: PathBuf,

    output: &'a (dyn Write + 'a),

    // Formatting options
    print_fullpath: bool,
    print_size: bool,
    human_readanle: bool,
    replace_nonprintables: bool,

    // Filter options
    exclude_pattern: Option<RegexSetBuilder>,
    include_pattern: Option<RegexSetBuilder>,

    exclude_matcher: Option<RegexSet>,
    include_matcher: Option<RegexSet>,
}

impl<'a> Aldar<'a> {
    pub fn new() -> Self {
        let mut default = PathBuf::new();
        default.push(".");

        let current_dir = env::current_dir().unwrap_or(default);     
        Aldar {
            show_hidden: false,
            dir_only: false,
            ignore_case: false,
            match_dirs: false,
            ascii_only: false,
            level: -1,
            path: current_dir,
            output: io::stdout().by_ref(),
            print_fullpath: false,
            print_size: false,
            human_readanle: false,
            replace_nonprintables: false,
            exclude_pattern: None,
            include_pattern: None,
            exclude_matcher: None,
            include_matcher: None,
        }
    }
}
