use std::{borrow::Cow, collections::HashMap};

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
/// map.insert("name".to_string(), "Foxie");
/// let hello_foxie = templater.format(&map).unwrap();
/// map.insert("name".to_string(), "World");
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

/// The main parsing login is being stored in the iterator implementation
/// So, this part is just uses it
impl<'r, 'a> From<TParserIter<'r, 'a>> for Templater<'a> {
    /// Converter of the token iterator into the actual representation
    fn from(value: TParserIter<'r, 'a>) -> Self {
        let mut parts = Vec::new();
        let mut keywords = Vec::new();

        for part in value {
            match part {
                ParseToken::Pair {
                    before_part,
                    keyword,
                } => {
                    parts.push(before_part);
                    keywords.push(keyword);
                }

                ParseToken::LastPart(part) => {
                    parts.push(part);
                }
            }
        }

        Self {
            parts,
            keywords,
            config: Default::default(),
        }
    }
}

/// Public logic is stored here
/// Functions [Templater::prepare] and [Templater::format]
impl<'a> Templater<'a> {
    /// This functions is used to prepare a template
    /// Generates a templater from a template file
    /// Templates are just files with variable names enclosed in brackets
    /// They are parsed with the following regular expression:
    /// \{\s*([a-zA-Z0-9_])+\s*\}
    /// Where the capture group is an identifier of the variable
    /// And everything not matchig is just a simple text
    /// # Example
    /// Hello, {world}
    pub fn prepare(data: &'a str, config: Config) -> Self {
        Self {
            config,
            ..TParserIter::parse(data).into()
        }
    }

    /// This function substitutes variables with the actual values
    /// I am not sure if it is really efficent
    /// But it is as it is
    pub fn format<T>(&self, dictionary: &HashMap<String, T>) -> Option<String>
    where
        T: std::string::ToString,
    {
        let templated = self.keywords.iter().map(|key| {
            dictionary.get(*key).map_or(
                match self.config.unmached_keywords {
                    UnmachedAction::Ignore => Some(Cow::Borrowed("")),
                    UnmachedAction::SubsWithKeywordName => Some(Cow::Borrowed(*key)),
                    UnmachedAction::ResultInNone => None,
                },
                |v| Some(Cow::Owned(v.to_string())),
            )
        });

        let interleaved = self
            .parts
            .iter()
            .map(|&s| Some(Cow::Borrowed(s)))
            .interleave(templated)
            .collect::<Option<String>>()?;

        Some(interleaved)
    }
}
