use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    fact::SampleExportEntry,
    operation::OperationContext,
    recipe::{RecipeError, Step},
    registry::Registry,
    sample::Sample,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Fact {
    description: Option<String>,
    #[serde(default)]
    imports: Vec<String>,
    #[serde(default)]
    exports: HashMap<String, SampleExportEntry>,
    #[serde(default)]
    steps: Vec<Step>,
    #[serde(default)]
    secret: bool,
}

impl Fact {
    #[must_use]
    pub fn new(
        description: Option<String>,
        imports: Vec<String>,
        exports: HashMap<String, SampleExportEntry>,
        steps: Vec<Step>,
        secret: bool,
    ) -> Self {
        Self {
            description,
            imports,
            exports,
            steps,
            secret,
        }
    }

    #[must_use]
    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn description_mut(&mut self) -> &mut Option<String> {
        &mut self.description
    }

    #[must_use]
    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub fn steps_mut(&mut self) -> &mut Vec<Step> {
        &mut self.steps
    }

    #[must_use]
    pub fn secret(&self) -> bool {
        self.secret
    }

    pub fn secret_mut(&mut self) -> &mut bool {
        &mut self.secret
    }

    #[must_use]
    pub fn imports(&self) -> &[String] {
        &self.imports
    }

    #[must_use]
    pub fn into_imports(self) -> Vec<String> {
        self.imports
    }

    #[must_use]
    pub fn exports(&self) -> &HashMap<String, SampleExportEntry> {
        &self.exports
    }

    #[must_use]
    pub fn into_exports(self) -> HashMap<String, SampleExportEntry> {
        self.exports
    }

    pub fn eval<F: FnMut(RecipeProgressEvent)>(
        &self,
        registry: &Registry,
        context: &OperationContext,
        imports: &HashMap<String, &Sample>,
        mut on_progress: F,
    ) -> Result<Vec<Sample>, RecipeError> {
        self.steps
            .iter()
            .enumerate()
            .try_fold(Vec::new(), |input, (index, step)| {
                on_progress(RecipeProgressEvent::StepStarted {
                    index,
                    step,
                    input: &input,
                });

                let samples = step.eval(registry, context, imports, input)?;
                on_progress(RecipeProgressEvent::StepFinished {
                    index,
                    step,
                    output: &samples,
                });

                Ok(samples)
            })
    }
}

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
