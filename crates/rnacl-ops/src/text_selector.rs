use regex::Regex;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub(crate) enum TextSelector {
    #[serde(rename = "text")]
    Plaintext(String),
    #[serde(rename = "regex")]
    Regex(#[serde(with = "serde_regex")] Regex),
}
