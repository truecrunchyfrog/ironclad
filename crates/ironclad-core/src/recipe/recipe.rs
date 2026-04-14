use serde::{Deserialize, Serialize};

use crate::{
    catalog::Catalog,
    recipe::{RecipeError, step::Step},
    sample::batch::Batch,
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Recipe(Vec<Step>);

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

    pub fn eval(&self, catalog: &Catalog) -> Result<Batch, RecipeError> {
        Ok(Batch::new(
            self.0
                .iter()
                .try_fold(Vec::new(), |input, step| step.eval(catalog, input))?
                .into_iter()
                .flatten()
                .collect::<Vec<_>>(),
        ))
    }
}
