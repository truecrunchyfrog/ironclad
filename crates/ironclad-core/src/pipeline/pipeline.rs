use serde::{Deserialize, Serialize};

use crate::{
    ledger::Ledger,
    pipeline::{PipelineError, stage::Stage},
    sample::batch::Batch,
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Pipeline(Vec<Stage>);

impl Pipeline {
    pub fn new(stages: Vec<Stage>) -> Self {
        Self(stages)
    }

    pub fn stages(&self) -> &[Stage] {
        &self.0
    }

    pub fn add(&mut self, index: Option<usize>, stage: Stage) -> Result<(), PipelineError> {
        match index {
            Some(index) if index > self.0.len() => Err(PipelineError::OutOfRange {
                index,
                length: self.0.len(),
            }),
            Some(index) => Ok(self.0.insert(index, stage)),
            None => Ok(self.0.push(stage)),
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

    pub fn eval(&self, ledger: &Ledger) -> Result<Batch, PipelineError> {
        Ok(Batch::new(
            self.0
                .iter()
                .try_fold(Vec::new(), |input, stage| stage.eval(ledger, input))?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        ))
    }
}
