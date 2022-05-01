// Copyright © 2018 The Aldar Authors
//
// Use of this source code is governed by an BSD-style
// license that can be found in the LICENSE file.

use regex::{RegexSet, RegexSetBuilder};
use std::{
    env,
    io::{self, Write},
    path::PathBuf, error::Error,
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


pub struct Aldar<'a> {
    show_hidden_files: bool,
    dir_only: bool,
    ignore_case: bool,
    level: i32,

    path: PathBuf,

    output: Box<dyn Write>,
    glyphs: &'a dyn Glyphs,

    // Formatting options
    print_fullpath: bool,
    print_size: bool,
    human_readable: bool,
    replace_nonprintables: bool,

    // Filter options
    exclude_pattern: Option<RegexSetBuilder>,
    include_pattern: Option<RegexSetBuilder>,

    exclude_matcher: Option<RegexSet>,
    include_matcher: Option<RegexSet>,

    // Statistics
    proc_dirs: u64,
    proc_files: u64,
}

impl<'a> Aldar<'a>{
    /// Creates a new Aldar command.
    pub fn new() -> Aldar<'a> {
        let mut default = PathBuf::new();
        default.push(".");

        let current_dir = env::current_dir().unwrap_or(default);
        Aldar {
            show_hidden_files: false,
            dir_only: false,
            ignore_case: false,
            level: -1,
            path: current_dir,
            glyphs: &UNICODE_GLYPHSET,
            output: Box::new(io::stdout()),
            print_fullpath: false,
            print_size: false,
            human_readable: false,
            replace_nonprintables: false,
            exclude_pattern: None,
            include_pattern: None,
            exclude_matcher: None,
            include_matcher: None,
            proc_dirs: 0,
            proc_files: 0,
        }
    }

    // Configures on which path aldar should operate.
    pub fn use_path(&'a mut self, path: &str) -> &'a mut Aldar {
        self.path.clear();
        self.path.push(path.to_owned());
        return self
    }

    /// Configure whether or not hidden files should be printed.
    pub fn show_hidden(&'a mut self, show_hidden: bool) -> &'a mut Aldar {
        self.show_hidden_files = show_hidden;
        self
    }

    /// Configures whether or not only directories should be printed.
    pub fn show_dirs_only(&'a mut self, show_dirs_only: bool) -> &'a mut Aldar {
        self.dir_only = show_dirs_only;
        self
    }

    /// Configures whether or not to ignore case when pattern matching is used.
    pub fn case_sensitive(&'a mut self, ignore_case: bool) -> &'a mut Aldar {
        self.ignore_case = ignore_case;
        self
    }

    /// Configures which glyphset to use.
    pub fn use_glyphset<T: Glyphs>(&'a mut self, glyphs: &'a T) -> &'a mut Aldar {
        self.glyphs = glyphs;
        self
    }

    /// Configures how deep directory recursion should go (default: -1 unrestricted).
    pub fn use_max_level(&'a mut self, lvl: i32) -> &'a mut Aldar {
        self.level = lvl;
        self
    }

    /// Configures whether to show full path for items or not.
    pub fn show_fullpath(&'a mut self, show_fullpath: bool) -> &'a mut Aldar {
        self.print_fullpath = show_fullpath;
        self
    }

    /// Configures whether to show size for items or not.
    pub fn show_size(&'a mut self, show_size: bool) -> &'a mut Aldar {
        self.print_size = show_size;
        self
    }

    /// Configures whether to show size in a human readable manner for items or not.
    pub fn show_human_readable(&'a mut self, show_human_readable: bool) -> &'a mut Aldar {
        self.human_readable = show_human_readable;
        self
    }

    /// Configures whether to replace non printables characters with a ?.
    pub fn do_replace_nonprintable_chars(
        &'a mut self,
        replace_nonprintables: bool,
    ) -> &'a mut Aldar {
        self.replace_nonprintables = replace_nonprintables;
        self
    }

    /// Configures aldar to use given strings as include patterns.
    pub fn set_include_patterns(&'a mut self, patterns: &[&'a str]) -> &'a mut Aldar {
        self.include_pattern = Some(RegexSetBuilder::new(patterns));
        self
    }

    /// Configures aldar to use given strings as exclude patterns.
    pub fn set_exclude_patterns(&'a mut self, patterns: &[&'a str]) -> &'a mut Aldar {
        self.exclude_pattern = Some(RegexSetBuilder::new(patterns));
        self
    }

    pub fn run(&'a mut self) -> Result<(), Box<dyn Error>> {

        if let Err(e) = writeln!(self.output.as_mut(), "Hello World! Bye") {
            return Err(Box::new(e));
        }
        
        Ok(())
    }
}
