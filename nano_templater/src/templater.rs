use std::collections::HashMap;

use config::Config;
use itertools::Itertools;
use parser_iter::*;

use crate::templater::config::UnmachedAction;

pub mod config;
mod parser_iter;

/// The structure for templating
/// Templating is done in two stages
/// - Preparation
/// The template file is being prepared using the method [prepare](Templater::prepare)
/// - Render
/// Variables are being substituted for the actual values using the function [format](Templater::format)
/// # Example
///
/// ```
/// use nano_templater::Templater;
/// use std::collections::HashMap;
/// let template = "Hello, {name}!";
/// let templater = Templater::prepare(&template, Default::default());
/// let mut map = HashMap::new();
/// map.insert("name", "Foxie");
/// let hello_foxie = templater.format(&map).unwrap();
/// map.insert("name", "World");
/// let hello_world = templater.format(&map).unwrap();
/// ```
pub struct Templater<'a> {
    /// Parts, that are not to be substituted
    parts: Vec<&'a str>,
    /// Names of variables in the order they come
    /// Names can repeat
    keywords: Vec<&'a str>,
    /// Config
    config: Config,
}

/// Public logic is stored here
/// Functions [Templater::prepare] and [Templater::format]
impl<'a> Templater<'a> {
    /// This functions is used to prepare a template
    /// Generates a templater from a template file
    /// Templates are just files with variable names enclosed in brackets
    /// They are parsed with the following regular expression:
    /// \{\s*([a-zA-Z0-9_]+)\s*\}
    /// Where the capture group is an identifier of the variable
    /// And everything not matchig is just a simple text
    /// # Example
    /// Hello, {world}
    fn new(iter: TParserIter<'a>, config: Config) -> Self {
        let (parts, keywords) = iter.fold(
            (Vec::new(), Vec::new()),
            |(mut parts, mut keywords), token| {
                match token {
                    ParseToken::Pair {
                        before_part,
                        keyword,
                    } => {
                        parts.push(before_part);
                        keywords.push(keyword);
                    }
                    ParseToken::LastPart(part) => parts.push(part),
                }
                (parts, keywords)
            },
        );

        Self {
            parts,
            keywords,
            config,
        }
    }
    pub fn prepare(data: &'a str, config: Config) -> Self {
        Self::new(TParserIter::parse(data), config)
    }

    /// This function substitutes variables with the actual values
    /// I am not sure if it is really efficent
    /// But it is as it is
    pub fn format<T>(&self, dictionary: &HashMap<&'a str, T>) -> Option<String>
    where
        T: std::fmt::Display,
    {
        let keyword_values: Vec<String> = self
            .keywords
            .iter()
            .map(|key| {
                dictionary.get(*key).map_or(
                    match self.config.unmached_keywords {
                        UnmachedAction::Ignore => Some(String::new()),
                        UnmachedAction::ResultInNone => None,
                        UnmachedAction::SubsWithKeywordName => Some(key.to_string()),
                    },
                    |v| Some(v.to_string()),
                )
            })
            .collect::<Option<Vec<String>>>()?;

        let total_size = keyword_values
            .iter()
            .map(String::len)
            .chain(self.parts.iter().map(|p| p.len()))
            .sum::<usize>();

        self.parts
            .iter()
            .copied()
            .interleave(keyword_values.iter().map(String::as_str))
            .fold(String::with_capacity(total_size), |mut acc, item| {
                acc.push_str(item);
                acc
            })
            .into()
    }
}
