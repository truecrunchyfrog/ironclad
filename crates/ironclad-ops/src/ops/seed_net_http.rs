use std::collections::HashMap;

use ironclad_core::{
    catalog::Catalog,
    operation::TypedOperation,
    sample::{Sample, Trace},
};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;

pub(crate) struct SeedNetHttp;

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

impl TypedOperation for SeedNetHttp {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "HTTP GET a web resource."
    }

    fn eval_all(
        &self,
        _catalog: &Catalog,
        _input: Vec<Sample>,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let client = reqwest::blocking::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&options.user_agent)?);

        let response = client.get(options.url).headers(headers).send()?;

        Ok(vec![Sample::new(
            Trace::new(HashMap::from([(
                "http_status_code".to_string(),
                response.status().as_u16().to_string(),
            )])),
            response.text()?,
        )])
    }
}
