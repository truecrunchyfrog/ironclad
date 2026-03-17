use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::sample::Sample;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Batch {
    samples: Vec<Sample>,
    created: SystemTime,
}

impl Batch {
    pub fn new(samples: Vec<Sample>) -> Self {
        Self {
            samples,
            created: SystemTime::now(),
        }
    }

    // pub fn evolve<F>(self, f: F) -> Self
    // where
    //     F: Fn(Sample) -> Vec<(Trace, String)>,
    // {
    //     Self::new(
    //         self.samples
    //             .into_iter()
    //             .flat_map(|sample| sample.evolve(&f))
    //             .collect(),
    //     )
    // }

    pub fn into_samples(self) -> Vec<Sample> {
        self.samples
    }

    pub fn samples(&self) -> &[Sample] {
        &self.samples
    }

    pub fn samples_mut(&mut self) -> &mut Vec<Sample> {
        &mut self.samples
    }

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
