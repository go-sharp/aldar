
// Copyright © 2018 The Aldar Authors
//
// Use of this source code is governed by an BSD-style
// license that can be found in the LICENSE file.

/// Represents a glyphset.
#[derive(Debug)]
pub struct GlyphSet(&'static str, &'static str, &'static str);

impl Glyphs for GlyphSet {
    fn pipe(&self) -> &'static str {
        self.0
    }

    fn item(&self) -> &'static str {
        self.2
    }

    fn last(&self) -> &'static str {
        self.1
    }
}


/// Unicode glyphset uses unicode charachters.
pub const UNICODE_GLYPHSET: GlyphSet = GlyphSet("│", "└──", "├──");


/// Unicode glyphset uses unicode charachters.
pub const ASCII_GLYPHSET: GlyphSet = GlyphSet("|", "`--", "|--");

trait Glyphs {
    fn pipe(&self) -> &'static str;
    fn item(&self) -> &'static str;
    fn last(&self) -> &'static str;
}