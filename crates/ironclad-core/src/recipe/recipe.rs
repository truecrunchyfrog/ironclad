use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    catalog::Catalog,
    recipe::{RecipeError, step::Step},
    sample::Sample,
};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Recipe(Vec<Step>);

pub enum RecipeProgressEvent<'a> {
    StepStarted {
        index: usize,
        step: &'a Step,
        input: &'a Vec<Sample>,
    },
    StepFinished {
        index: usize,
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

    #[must_use]
    pub fn steps_mut(&mut self) -> &mut Vec<Step> {
        &mut self.0
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
        imports: &HashMap<&String, &Sample>,
        mut on_progress: F,
    ) -> Result<Vec<Sample>, RecipeError> {
        self.0
            .iter()
            .zip(0..)
            .try_fold(Vec::new(), |input, (step, index)| {
                let mut step = step.clone();
                let mut options = step.options_mut();
                visit_json_strings_mut(&mut options, &mut |s| {
                    for (label, sample) in imports {
                        *s = s.replace(&format!("$({label})"), sample.content());
                    }
                });

                on_progress(RecipeProgressEvent::StepStarted {
                    index,
                    step: &step,
                    input: &input,
                });

                let samples = step.eval(catalog, input)?;
                on_progress(RecipeProgressEvent::StepFinished {
                    index,
                    step: &step,
                    output: &samples,
                });

                Ok(samples)
            })
    }
}

fn visit_json_strings_mut<F: FnMut(&mut String)>(value: &mut serde_json::Value, f: &mut F) {
    match value {
        serde_json::Value::String(s) => f(s),
        serde_json::Value::Array(array) => {
            for item in array {
                visit_json_strings_mut(item, f);
            }
        }
        serde_json::Value::Object(map) => {
            for (_, value) in map {
                visit_json_strings_mut(value, f);
            }
        }
        serde_json::Value::Null | serde_json::Value::Number(_) | serde_json::Value::Bool(_) => {}
    }
}
