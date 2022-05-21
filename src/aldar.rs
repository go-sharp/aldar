// Copyright © 2022 The Aldar Authors
//
// Use of this source code is governed by an BSD-style
// license that can be found in the LICENSE file.

use colored::*;
use regex::{RegexSet, RegexSetBuilder};
use simple_error::SimpleError;
use std::{
    cmp::Ordering,
    env,
    error::Error,
    fs::{self, DirEntry},
    io::{self, Write},
    path::PathBuf,
};

use crate::fsutil::AldarExt;

const KB_SIZE: u64 = 1 << 10;
const MB_SIZE: u64 = 1 << 20;
const GB_SIZE: u64 = 1 << 30;
const TB_SIZE: u64 = 1 << 40;
const EB_SIZE: u64 = 1 << 50;
const PB_SIZE: u64 = 1 << 60;

/// Represents a glyphset.
#[derive(Debug)]
pub struct GlyphSet(&'static str, &'static str, &'static str);

impl Glyphs for GlyphSet {
    fn pipe(&self) -> String {
        self.0.to_owned()
    }

    fn last(&self) -> String {
        self.1.to_owned()
    }

    fn item(&self) -> String {
        self.2.to_owned()
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
    level: i32,

    path: PathBuf,

    output: Box<dyn Write>,
    glyphs: Box<dyn Glyphs>,

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

    indent: Vec<String>,
    sz_last: usize,
    sz_item: usize,
}

impl Aldar {
    /// Creates a new Aldar command.
    pub fn new() -> Self {
        let mut default = PathBuf::new();
        default.push(".");

        let current_dir = env::current_dir().unwrap_or(default);
        Self {
            show_hidden_files: false,
            dir_only: false,
            ignore_case: false,
            level: -1,
            path: current_dir,
            glyphs: Box::new(UNICODE_GLYPHSET),
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
            indent: vec![],
            sz_item: UNICODE_GLYPHSET.item().chars().count(),
            sz_last: UNICODE_GLYPHSET.last().chars().count() + 1,
        }
    }

    // Configures to use given writer.
    pub fn use_writer(&mut self, writer: Box<dyn Write>) -> &mut Aldar {
        self.output = writer;
        self
    }

    // Configures on which path aldar should operate.
    pub fn use_path(&mut self, path: String) -> &mut Aldar {
        self.path.clear();
        self.path.push(path.to_owned());
        self
    }

    /// Configure whether or not hidden files should be printed.
    pub fn show_hidden(&mut self, show_hidden: bool) -> &mut Aldar {
        self.show_hidden_files = show_hidden;
        self
    }

    /// Configures whether or not only directories should be printed.
    pub fn show_dirs_only(&mut self, show_dirs_only: bool) -> &mut Aldar {
        self.dir_only = show_dirs_only;
        self
    }

    /// Configures whether or not to ignore case when pattern matching is used.
    pub fn case_sensitive(&mut self, ignore_case: bool) -> &mut Aldar {
        self.ignore_case = ignore_case;
        self
    }

    /// Configures which glyphset to use.
    pub fn use_glyphset(&mut self, glyphs: Box<dyn Glyphs>) -> &mut Aldar {
        self.glyphs = glyphs;
        self.sz_item = UNICODE_GLYPHSET.item().chars().count();
        self.sz_last = UNICODE_GLYPHSET.last().chars().count() + 1;
        self
    }

    /// Configures how deep directory recursion should go (default: -1 unrestricted).
    pub fn use_max_level(&mut self, lvl: i32) -> &mut Aldar {
        self.level = lvl;
        self
    }

    /// Configures whether to show full path for items or not.
    pub fn show_fullpath(&mut self, show_fullpath: bool) -> &mut Aldar {
        self.print_fullpath = show_fullpath;
        self
    }

    /// Configures whether to show size for items or not.
    pub fn show_size(&mut self, show_size: bool) -> &mut Aldar {
        self.print_size = show_size;
        self
    }

    /// Configures whether to show size in a human readable manner for items or not.
    pub fn show_human_readable(&mut self, show_human_readable: bool) -> &mut Aldar {
        self.human_readable = show_human_readable;
        self
    }

    /// Configures whether to replace non printables characters with a ?.
    pub fn do_replace_nonprintable_chars(&mut self, replace_nonprintables: bool) -> &mut Aldar {
        self.replace_nonprintables = replace_nonprintables;
        self
    }

    /// Configures aldar to use given strings as include patterns.
    pub fn set_include_patterns(&mut self, patterns: &[&str]) -> &mut Aldar {
        self.include_pattern = Some(RegexSetBuilder::new(patterns));
        self
    }

    /// Configures aldar to use given strings as exclude patterns.
    pub fn set_exclude_patterns(&mut self, patterns: &[&str]) -> &mut Aldar {
        self.exclude_pattern = Some(RegexSetBuilder::new(patterns));
        self
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.proc_dirs = 0;
        self.proc_files = 0;

        // Build include pattern if any was specified
        if let Some(builder) = self.include_pattern.as_mut() {
            builder.case_insensitive(self.ignore_case);
            let matcher = builder.build();
            if matcher.is_err() {
                return Err(Box::new(SimpleError::new(
                    "invalid include pattern specified",
                )));
            }

            self.include_matcher = Some(matcher.unwrap());
        }

        // Build exclude pattern if any was specified
        if let Some(builder) = self.exclude_pattern.as_mut() {
            builder.case_insensitive(self.ignore_case);
            let matcher = builder.build();
            if matcher.is_err() {
                return Err(Box::new(SimpleError::new(
                    "invalid exclude pattern specified",
                )));
            }

            self.exclude_matcher = Some(matcher.unwrap());
        }

        let working_dir = self.path.to_str().unwrap_or_else(|| ".").to_string();

        writeln!(self.output.as_mut(), "{}", working_dir.blue()).ok();

        self.show_dir(&working_dir, 0).ok();

        writeln!(
            self.output.as_mut(),
            "\n{} directories, {} files",
            self.proc_dirs,
            self.proc_files
        )
        .ok();
        Ok(())
    }

    fn show_dir(&mut self, working_dir: &str, lvl: i32) -> Result<(), Box<dyn Error>> {
        // Bail out if level is reached
        if self.level > -1 && lvl > self.level {
            return Ok(());
        }

        let dirs = self.fetch_directory(working_dir)?;
        let sz = dirs.len();

        for (i, entry) in dirs.iter().enumerate() {
            self.print_entry(entry, sz == i + 1);

            if entry.is_dir() {
                if let Some(p) = entry.path().to_str() {
                    self.do_indent(sz == i + 1);
                    self.show_dir(p, lvl + 1).ok();
                    self.do_unindent();
                }
            }
        }

        Ok(())
    }

    fn fetch_directory(&mut self, working_dir: &str) -> Result<Vec<DirEntry>, Box<dyn Error>> {
        if let Some(set) = self.exclude_matcher.as_ref() {
            if set.is_match(working_dir) {
                return Ok(vec![]);
            }
        }

        let mut entries: Vec<DirEntry> = fs::read_dir(working_dir)?
            .filter_map(|r| {
                if !r.is_ok() {
                    return None;
                }

                let entry = r.unwrap();
                if !self.show_hidden_files && entry.is_hidden() {
                    return None;
                }

                if !entry.is_dir() {
                    if let Some(matcher) = self.include_matcher.as_ref() {
                        if !matcher.is_match(entry.file_name().to_str().unwrap()) {
                            return None;
                        }
                    }

                    if let Some(matcher) = self.exclude_matcher.as_ref() {
                        if matcher.is_match(entry.file_name().to_str().unwrap()) {
                            return None;
                        }
                    }
                }

                if entry.is_dir() {
                    self.proc_dirs += 1;
                } else {
                    self.proc_files += 1;
                }

                Some(entry)
            })
            .collect();

        entries.sort_by(|a, b| {
            if a.path().is_dir() && b.path().is_file() {
                return Ordering::Less;
            }

            if b.path().is_dir() && a.path().is_file() {
                return Ordering::Greater;
            }

            a.path().as_path().cmp(b.path().as_path())
        });

        Ok(entries)
    }

    fn print_entry(&mut self, entry: &DirEntry, last: bool) {
        let mut indent = self.indent.clone();
        if last {
            indent.push(self.glyphs.last());
        } else {
            indent.push(self.glyphs.item());
        }

        if self.print_size {
            indent.push(self.size_as_str(entry.size()));
        }

        let mut file_name = match entry.file_name().to_os_string().to_str() {
            Some(s) => s.to_string(),
            _ => return,
        };

        if self.print_fullpath {
            let fp = self.path.canonicalize();
            if fp.is_ok() {
                if let Some(base) = fp.unwrap().to_str() {
                    file_name = entry.full_rel_path(base);
                }
            }
        }

        if entry.is_dir() && entry.is_hidden() {
            file_name = file_name.purple().to_string();
        } else if entry.is_dir() {
            file_name = file_name.blue().to_string();
        } else if entry.is_executable() {
            file_name = file_name.magenta().to_string();
        } else if entry.is_hidden() {
            file_name = file_name.cyan().to_string();
        }

        writeln!(
            self.output.as_mut(),
            "{} {}",
            indent.concat().to_string(),
            file_name
        )
        .ok();
    }

    fn do_indent(&mut self, is_last: bool) {
        if is_last {
            self.indent
                .push(String::from_utf8(vec![b' '; self.sz_last]).unwrap());
            return;
        }

        self.indent.push(
            self.glyphs.pipe()
                + String::from_utf8(vec![b' '; self.sz_item])
                    .unwrap()
                    .as_ref(),
        );
    }

    fn do_unindent(&mut self) {
        self.indent.pop();
    }

    fn size_as_str(&self, sz: u64) -> String {
        let create_str = |n: f64, unit: &str| -> String {
            let str_sz: String;
            if n.fract() == 0 as f64 {
                str_sz = format!("{:.0}{}", n, unit);
            } else {
                str_sz = format!("{:.2}{}", n, unit);
            }

            if self.human_readable {
                if str_sz.len() < 9 {
                    return format!(" [{: >8}]", str_sz);
                }
                return format!(" [{: >8.4E}{}]", n, unit);
            }

            if str_sz.len() < 12 {
                return format!(" [{: >11}]", str_sz);
            }
            return format!(" [{: >11.4E}{}]", n, unit);
        };

        if !self.human_readable {
            return create_str(sz as f64, "");
        }

        if sz > PB_SIZE {
            return create_str((sz as f64) / (PB_SIZE as f64), "PB");
        }

        if sz > EB_SIZE {
            return create_str((sz as f64) / (EB_SIZE as f64), "EB");
        }

        if sz > TB_SIZE {
            return create_str((sz as f64) / (TB_SIZE as f64), "TB");
        }

        if sz > GB_SIZE {
            return create_str((sz as f64) / (GB_SIZE as f64), "TB");
        }

        if sz > MB_SIZE {
            return create_str((sz as f64) / (MB_SIZE as f64), "MB");
        }

        if sz > KB_SIZE {
            return create_str((sz as f64) / (KB_SIZE as f64), "KB");
        }

        create_str(sz as f64, "")
    }
}
