/// Options for the templater
pub enum UnmachedAction {
    /// Substitutes it with empty string
    Ignore,
    /// Substitutes it with keyword name
    SubsWithKeywordName,
    /// Fail the whole templating
    ResultInNone,
}

pub struct Config {
    pub unmached_keywords: UnmachedAction,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            unmached_keywords: UnmachedAction::ResultInNone,
        }
    }
}
