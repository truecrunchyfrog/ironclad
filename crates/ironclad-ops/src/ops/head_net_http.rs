use std::collections::HashMap;

use ironclad_core::{
    catalog::Catalog,
    operation::TypedOperation,
    sample::{Sample, Trace},
};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;

pub(crate) struct HeadNetHttp;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    url: String,
    #[serde(default = "default_user_agent")]
    user_agent: String,
}

fn default_user_agent() -> String {
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/144.0.0.0 Safari/537.36".to_string()
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
}

impl TypedOperation for HeadNetHttp {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "HTTP GET a web resource."
    }

    fn eval(
        &self,
        _catalog: &Catalog,
        _input: Vec<Vec<Sample>>,
        options: Self::Options,
    ) -> Result<Vec<Vec<Sample>>, Self::Error> {
        let client = reqwest::blocking::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&options.user_agent)?);

        let response_text = client.get(options.url).headers(headers).send()?.text()?;

        Ok(vec![vec![Sample::new(
            Trace::new(HashMap::new()),
            response_text,
        )]])
    }
}
