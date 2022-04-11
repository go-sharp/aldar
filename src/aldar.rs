// Copyright © 2018 The Aldar Authors
//
// Use of this source code is governed by an BSD-style
// license that can be found in the LICENSE file.

use regex::{RegexSet, RegexSetBuilder};
use std::{
    env,
    path::PathBuf,
    io::{Write, self}
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

pub trait Glyphs {
    fn pipe(&self) -> String;
    fn item(&self) -> String;
    fn last(&self) -> String;
}


pub struct Aldar {
    show_hidden_files: bool,
    dir_only: bool,
    ignore_case: bool,    
    level: i64,

    path: PathBuf,

    output: Box<dyn Write>,
    glyphs: Box<dyn Glyphs>,

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

impl Aldar {
    /// Creates a new Aldar command.
    pub fn new() -> Self {
        let mut default = PathBuf::new();
        default.push(".");

        let current_dir = env::current_dir().unwrap_or(default);     
        Aldar {
            show_hidden_files: false,
            dir_only: false,
            ignore_case: false,            
            level: -1,
            path: current_dir,
            glyphs: Box::new(UNICODE_GLYPHSET),
            output: Box::new(io::stdout()),
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

    /// Configure whether or not hidden files should be printed.
    pub fn show_hidden<'a>(&'a mut self, show_hidden: bool) -> &'a mut Aldar {
        self.show_hidden_files = show_hidden;
        self
    }

    /// Configures whether or not only directories should be printed.
    pub fn show_dirs_only<'a>(&'a mut self, show_dirs_only: bool) -> &'a mut Aldar {
        self.dir_only = show_dirs_only;
        self
    }

    /// Configures whether or not to ignore case when pattern matching is used.
    pub fn case_sensitive<'a>(&'a mut self, ignore_case: bool) -> &'a mut Aldar {
        self.ignore_case = ignore_case;
        self
    }

    /// Configures whether or not to use asc
    pub fn use_glyphset<'a>(&'a mut self, glyphs: Box<dyn Glyphs>) -> &'a mut Aldar {
        self.glyphs = glyphs;
        self 
    }


}
