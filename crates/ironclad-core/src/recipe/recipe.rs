use serde::{Deserialize, Serialize};

use crate::{
    catalog::Catalog,
    recipe::{RecipeError, step::Step},
    sample::Sample,
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Recipe(Vec<Step>);

pub enum RecipeProgressEvent<'a> {
    BeforeEvaluateStep {
        step: &'a Step,
        input: &'a Vec<Sample>,
    },
    AfterEvaluateStep {
        step: &'a Step,
        output: &'a Vec<Sample>,
    },
}

impl Recipe {
    #[must_use]
    pub fn new(steps: Vec<Step>) -> Self {
        Self(steps)
    }

    #[must_use]
    pub fn steps(&self) -> &[Step] {
        &self.0
    }

    pub fn add(&mut self, index: Option<usize>, step: Step) -> Result<(), RecipeError> {
        match index {
            Some(index) if index > self.0.len() => Err(RecipeError::OutOfRange {
                index,
                length: self.0.len(),
            }),
            Some(index) => {
                self.0.insert(index, step);
                Ok(())
            }
            None => {
                self.0.push(step);
                Ok(())
            }
        }
    }

    pub fn remove(&mut self, index: Option<usize>) -> Result<Step, RecipeError> {
        match index {
            Some(index) if index > self.0.len() => Err(RecipeError::OutOfRange {
                index,
                length: self.0.len(),
            }),
            Some(index) => Ok(self.0.remove(index)),
            None => self.0.pop().ok_or(RecipeError::OutOfRange {
                index: 0,
                length: self.0.len(),
            }),
        }
    }

    pub fn eval<F: FnMut(RecipeProgressEvent)>(
        &self,
        catalog: &Catalog,
        mut on_progress: F,
    ) -> Result<Vec<Sample>, RecipeError> {
        self.0.iter().try_fold(Vec::new(), |input, step| {
            on_progress(RecipeProgressEvent::BeforeEvaluateStep {
                step: &step,
                input: &input,
            });
            let samples = step.eval(catalog, input)?;
            on_progress(RecipeProgressEvent::AfterEvaluateStep {
                step: &step,
                output: &samples,
            });
            Ok(samples)
        })
    }
}
