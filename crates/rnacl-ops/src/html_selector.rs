use serde::Deserialize;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct HtmlSelectorOptions {
    pub(crate) selector: String,
}

impl HtmlSelectorOptions {
    pub(crate) fn parse(&self) -> Result<scraper::Selector, HtmlSelectorError> {
        scraper::Selector::parse(&self.selector)
            .map_err(|err| HtmlSelectorError::Selector(err.to_string()))
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum HtmlSelectorError {
    #[error("invalid CSS selector: {0}")]
    Selector(String),
}
