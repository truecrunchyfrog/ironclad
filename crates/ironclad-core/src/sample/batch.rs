use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::sample::Sample;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Batch {
    samples: Vec<Sample>,
    created: SystemTime,
}

impl Batch {
    #[must_use]
    pub fn new(samples: Vec<Sample>) -> Self {
        Self {
            samples,
            created: SystemTime::now(),
        }
    }

    #[must_use]
    pub fn into_samples(self) -> Vec<Sample> {
        self.samples
    }

    #[must_use]
    pub fn samples(&self) -> &[Sample] {
        &self.samples
    }

    pub fn samples_mut(&mut self) -> &mut Vec<Sample> {
        &mut self.samples
    }

    #[must_use]
    pub fn created(&self) -> &SystemTime {
        &self.created
    }
}

impl Default for Batch {
    fn default() -> Self {
        Self {
            samples: Default::default(),
            created: SystemTime::now(),
        }
    }
}
