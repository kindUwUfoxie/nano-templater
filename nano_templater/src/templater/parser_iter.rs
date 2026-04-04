use std::sync::LazyLock;

use regex::{CaptureMatches, Regex};

static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\{\s*([a-zA-Z0-9_]+)\s*\}"#).unwrap());

pub(super) struct TParserIter<'a> {
    data: &'a str,
    matches: CaptureMatches<'static, 'a>,
    last: usize,
}

pub(super) enum ParseToken<'a> {
    Pair {
        before_part: &'a str,
        keyword: &'a str,
    },
    LastPart(&'a str),
}

impl<'a> TParserIter<'a> {
    pub(super) fn parse(data: &'a str) -> Self {
        let captures = RE.captures_iter(data);
        Self {
            data,
            matches: captures,
            last: 0,
        }
    }
}

impl<'a> Iterator for TParserIter<'a> {
    type Item = ParseToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(m) = self.matches.next() {
            let place = m.get(0).expect("Unreachable: match could no have no match");
            let kw = m
                .get(1)
                .expect("Unreachable: match must have the group matched");

            let item = &self.data[self.last..place.start()];
            self.last = place.end();
            Some(ParseToken::Pair {
                before_part: item,
                keyword: kw.as_str(),
            })
        } else if self.last < self.data.len() {
            let item = &self.data[self.last..];
            self.last = self.data.len();
            Some(ParseToken::LastPart(item))
        } else {
            None
        }
    }
}
