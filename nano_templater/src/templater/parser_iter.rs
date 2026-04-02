use regex::{CaptureMatches, Regex};

pub(super) struct TParserIter<'r, 'a> {
    data: &'a str,
    matches: CaptureMatches<'r, 'a>,
    last: usize,
    finished: bool,
}

pub(super) enum ParseToken<'a> {
    Pair {
        before_part: &'a str,
        keyword: &'a str,
    },
    LastPart(&'a str),
}

impl<'r, 'a> TParserIter<'r, 'a> {
    pub(super) fn parse(re: &'r mut Option<Regex>, data: &'a str) -> Self {
        *re = Some(Regex::new(r#"\{\s*([a-zA-Z0-9_]+)\s*\}"#).unwrap());
        let r = re.as_ref().unwrap();
        let captures = r.captures_iter(data);
        Self {
            data: data,
            matches: captures,
            last: 0,
            finished: false,
        }
    }
}

impl<'r, 'a> Iterator for TParserIter<'r, 'a> {
    type Item = ParseToken<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            None
        } else {
            if let Some(m) = self.matches.next() {
                let (Some(place), Some(kw)) = (m.get(0), m.get(1)) else {
                    panic!("Could not compile the template");
                };

                let item = &self.data[self.last..place.start()];
                self.last = place.end();
                Some(ParseToken::Pair {
                    before_part: item,
                    keyword: kw.as_str(),
                })
            } else {
                self.finished = true;
                let item = &self.data[self.last..];
                Some(ParseToken::LastPart(item))
            }
        }
    }
}
