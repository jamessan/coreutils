//  * This file is part of the uutils coreutils package.
//  *
//  * (c) Michael Gehring <mg@ebfe.org>
//  * (c) kwantam <kwantam@gmail.com>
//  *     * 2015-04-28 ~ created `expand` module to eliminate most allocs during setup
//  *
//  * For the full copyright and license information, please view the LICENSE
//  * file that was distributed with this source code.

// spell-checker:ignore (ToDO) allocs slen unesc

use std::char::from_u32;
use std::cmp::min;
use std::iter::Peekable;
use std::ops::RangeInclusive;

#[inline]
fn unescape_char(s: &str) -> (char, usize) {
    // Longest escape sequence is an octal number of 3 digits
    let mut chars = s.chars().take(3);
    match chars.next().unwrap() {
        'a' => (0x07u8 as char, 1),
        'b' => (0x08u8 as char, 1),
        'f' => (0x0cu8 as char, 1),
        'v' => (0x0bu8 as char, 1),
        'n' => ('\n', 1),
        'r' => ('\r', 1),
        't' => ('\t', 1),
        '\\' => ('\\', 1),
        n @ '0'..='7' => {
            let mut ord = n.to_digit(8).unwrap();
            let mut len = 1;
            while let Some(c) = chars.next() {
                // Octal numbers >= 256 (e.g., \410) only consume the first two digits, leaving the
                // rest for later parsing (\410 -> \41 0)
                match c {
                    n @ '0'..='7' => {
                        let next = ord * 8 + n.to_digit(8).unwrap();
                        if next >= std::u8::MAX.into() {
                            break;
                        }
                        ord = next;
                        len += 1;
                    }
                    _ => break
                }
            }

            (from_u32(ord).unwrap(), len)
        },
        c => (c, 1),
    }
}

struct Unescape<'a> {
    string: &'a str,
}

impl<'a> Iterator for Unescape<'a> {
    type Item = char;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let slen = self.string.len();
        (min(slen, 1), None)
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.string.is_empty() {
            return None;
        }

        // is the next character an escape?
        let (ret, idx) = match self.string.chars().next().unwrap() {
            '\\' if self.string.len() > 1 => {
                // yes---it's \ and it's not the last char in a string
                // we know that \ is 1 byte long so we can index into the string safely
                let (c, len) = unescape_char(&self.string[1..]);
                (Some(c), len + c.len_utf8())
            }
            c => (Some(c), c.len_utf8()), // not an escape char
        };

        self.string = &self.string[idx..]; // advance the pointer to the next char
        ret
    }
}

pub struct ExpandSet<'a> {
    range: RangeInclusive<u32>,
    unesc: Peekable<Unescape<'a>>,
}

impl<'a> Iterator for ExpandSet<'a> {
    type Item = char;

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.unesc.size_hint()
    }

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // while the Range has elements, try to return chars from it
        // but make sure that they actually turn out to be Chars!
        while let Some(n) = self.range.next() {
            if let Some(c) = from_u32(n) {
                return Some(c);
            }
        }

        if let Some(first) = self.unesc.next() {
            // peek ahead
            if self.unesc.peek() == Some(&'-') && self.unesc.size_hint().0 > 1 {
                self.unesc.next(); // this is the '-'
                let last = self.unesc.next().unwrap(); // this is the end of the range

                {
                    self.range = first as u32 + 1..=last as u32;
                }
            }

            return Some(first); // in any case, return the next char
        }

        None
    }
}

impl<'a> ExpandSet<'a> {
    #[inline]
    pub fn new(s: &'a str) -> ExpandSet<'a> {
        ExpandSet {
            range: 0..=0,
            unesc: Unescape { string: s }.peekable(),
        }
    }
}
