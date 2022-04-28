// Copyright © SixtyFPS GmbH <info@slint-ui.com>
// SPDX-License-Identifier: GPL-3.0-only OR LicenseRef-Slint-commercial

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BreakOpportunity {
    Allowed,
    Mandatory,
}

#[derive(Clone)]
pub struct LineBreakIterator<'a> {
    it: core::str::CharIndices<'a>,
    leading_whitespace: bool,
}

impl<'a> LineBreakIterator<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { it: text.char_indices(), leading_whitespace: false }
    }
}

impl<'a> Iterator for LineBreakIterator<'a> {
    type Item = (usize, BreakOpportunity);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((byte_offset, char)) = self.it.next() {
            let maybe_opportunity = match char {
                '\u{2028}' | '\u{2029}' => Some(BreakOpportunity::Mandatory), // unicode line- and paragraph separators
                '\n' => Some(BreakOpportunity::Mandatory),                    // ascii line break
                _ if self.leading_whitespace => Some(BreakOpportunity::Allowed),
                _ => None,
            };
            self.leading_whitespace = char.is_ascii_whitespace();

            if let Some(opportunity) = maybe_opportunity {
                return Some((byte_offset, opportunity));
            }
        }

        None
    }
}
