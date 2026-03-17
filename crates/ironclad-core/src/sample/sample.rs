use serde::{Deserialize, Serialize};

use crate::sample::trace::Trace;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Sample {
    traces: Vec<Trace>,
    content: String,
}

impl Sample {
    pub fn new(trace: Trace, content: String) -> Self {
        Self {
            traces: vec![trace],
            content,
        }
    }

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

    pub fn traces(&self) -> &[Trace] {
        &self.traces
    }

    pub fn content(&self) -> &String {
        &self.content
    }

    pub fn into_content(self) -> String {
        self.content
    }
}
