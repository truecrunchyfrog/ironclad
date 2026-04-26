use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    catalog::Catalog,
    fact::SampleExportEntry,
    recipe::{RecipeError, Step},
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
        catalog: &Catalog,
        imports: &HashMap<&String, &Sample>,
        mut on_progress: F,
    ) -> Result<Vec<Sample>, RecipeError> {
        self.steps
            .iter()
            .enumerate()
            .try_fold(Vec::new(), |input, (index, step)| {
                let mut step = step.clone();
                visit_toml_strings_mut(step.options_mut(), &mut |s| {
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

fn visit_toml_strings_mut<F: FnMut(&mut String)>(value: &mut toml::Value, f: &mut F) {
    match value {
        toml::Value::String(s) => f(s),
        toml::Value::Array(array) => {
            for item in array {
                visit_toml_strings_mut(item, f);
            }
        }
        toml::Value::Table(map) => {
            for (_, value) in map {
                visit_toml_strings_mut(value, f);
            }
        }
        toml::Value::Integer(_)
        | toml::Value::Float(_)
        | toml::Value::Boolean(_)
        | toml::Value::Datetime(_) => {}
    }
}
