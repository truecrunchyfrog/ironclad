use std::collections::HashMap;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};

pub(crate) struct SeedNetHttp;

#[derive(Deserialize, Serialize, Clone, Default)]
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
        "Fetch one URL with HTTP GET.\n\n\
HTTP requests use blocking `reqwest`:\n\
https://docs.rs/reqwest/latest/reqwest/\n\n\
The operation performs one GET request, checks for HTTP error status with \
`error_for_status()`, and returns the response body as a single sample. A \
custom user-agent can be provided when the default is not appropriate.\n\n\
Options:\n\
- `url`: request target.\n\
- `user_agent`: HTTP user-agent header value."
    }

    fn eval_all(
        &self,
        _context: &OperationContext,
        _input: Vec<Sample>,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        let client = reqwest::blocking::Client::new();
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&options.user_agent)?);

        let response_text = client
            .get(options.url)
            .headers(headers)
            .send()?
            .error_for_status()?
            .text()?;

        Ok(vec![Sample::new(Trace::new(HashMap::new()), response_text)])
    }
}
