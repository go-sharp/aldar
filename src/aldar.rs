
// Copyright © 2018 The Aldar Authors
//
// Use of this source code is governed by an BSD-style
// license that can be found in the LICENSE file.

/// Represents a glyphset.
#[derive(Debug)]
pub struct GlyphSet(&'static str, &'static str, &'static str);

impl Glyphs for GlyphSet {
    fn pipe(&self) -> &str {
        self.0
    }

    fn item(&self) -> &str {
        self.2
    }

    fn last(&self) -> &str {
        self.1
    }
}


/// Unicode glyphset uses unicode charachters.
pub const UNICODE_GLYPHSET: GlyphSet = GlyphSet("│", "└──", "├──");


/// Unicode glyphset uses unicode charachters.
pub const ASCII_GLYPHSET: GlyphSet = GlyphSet("|", "`--", "|--");

trait Glyphs {
    fn pipe(&self) -> &str;
    fn item(&self) -> &str;
    fn last(&self) -> &str;
}