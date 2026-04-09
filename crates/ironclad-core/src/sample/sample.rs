use serde::{Deserialize, Serialize};

use crate::sample::trace::Trace;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Sample {
    traces: Vec<Trace>,
    content: String,
}

impl Sample {
    #[must_use]
    pub fn new(trace: Trace, content: String) -> Self {
        Self {
            traces: vec![trace],
            content,
        }
    }

    #[must_use]
    pub fn evolve(&self, trace: Trace, content: String) -> Self {
        Self {
            traces: self
                .traces
                .iter()
                .cloned()
                .chain(std::iter::once(trace))
                .collect(),
            content,
        }
    }

    #[must_use]
    pub fn traces(&self) -> &[Trace] {
        &self.traces
    }

    #[must_use]
    pub fn content(&self) -> &String {
        &self.content
    }

    #[must_use]
    pub fn into_content(self) -> String {
        self.content
    }
}
