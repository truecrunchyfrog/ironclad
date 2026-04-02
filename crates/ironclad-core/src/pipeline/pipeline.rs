use serde::{Deserialize, Serialize};

use crate::{
    cluster::Cluster,
    pipeline::{PipelineError, stage::Stage},
    sample::batch::Batch,
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Pipeline(Vec<Stage>);

impl Pipeline {
    #[must_use]
    pub fn new(stages: Vec<Stage>) -> Self {
        Self(stages)
    }

    #[must_use]
    pub fn stages(&self) -> &[Stage] {
        &self.0
    }

    pub fn add(&mut self, index: Option<usize>, stage: Stage) -> Result<(), PipelineError> {
        match index {
            Some(index) if index > self.0.len() => Err(PipelineError::OutOfRange {
                index,
                length: self.0.len(),
            }),
            Some(index) => {
                self.0.insert(index, stage);
                Ok(())
            }
            None => {
                self.0.push(stage);
                Ok(())
            }
        }
    }

    pub fn remove(&mut self, index: Option<usize>) -> Result<Stage, PipelineError> {
        match index {
            Some(index) if index > self.0.len() => Err(PipelineError::OutOfRange {
                index,
                length: self.0.len(),
            }),
            Some(index) => Ok(self.0.remove(index)),
            None => self.0.pop().ok_or(PipelineError::OutOfRange {
                index: 0,
                length: self.0.len(),
            }),
        }
    }

    pub fn eval(&self, cluster: &Cluster) -> Result<Batch, PipelineError> {
        Ok(Batch::new(
            self.0
                .iter()
                .try_fold(Vec::new(), |input, stage| stage.eval(cluster, input))?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        ))
    }
}
