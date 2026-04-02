use std::collections::HashMap;

use itertools::Itertools;
use parser_iter::*;

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
/// use std::collection::HashMap;
/// let template = "Hello, {name}!";
/// let templater = Templater::prepare(&template);
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

        Self { parts, keywords }
    }
}

/// A hack to rule them all
enum S<'a> {
    SR(&'a str),
    S(String),
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
    pub fn prepare(data: &'a str) -> Self {
        let mut re = None;
        TParserIter::parse(&mut re, data).into()
    }

    /// This function substitutes variables with the actual values
    /// I am not sure if it is really efficent
    /// But it is as it is
    pub fn format<T>(&self, dictionary: &HashMap<String, T>) -> Option<String>
    where
        T: std::string::ToString,
    {
        let templated = self
            .keywords
            .iter()
            .map(|key| dictionary.get(*key).map(|t| S::S(t.to_string())))
            .collect::<Option<Vec<S>>>()?;

        let interleaved = self.parts.iter().map(|sr| S::SR(sr)).interleave(templated);

        let mut result = String::new();

        for part in interleaved {
            match part {
                S::SR(sr) => result.push_str(sr),
                S::S(s) => result.push_str(&s),
            }
        }

        Some(result)
    }
}
