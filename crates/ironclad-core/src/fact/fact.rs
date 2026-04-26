use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{fact::SampleExportEntry, recipe::Recipe};

#[derive(Serialize, Deserialize, Debug)]
pub struct Fact {
    description: Option<String>,
    #[serde(default)]
    imports: Vec<String>,
    #[serde(default)]
    exports: HashMap<String, SampleExportEntry>,
    #[serde(default)]
    steps: Recipe,
    #[serde(default)]
    secret: bool,
}

impl Fact {
    #[must_use]
    pub fn new(
        description: Option<String>,
        imports: Vec<String>,
        exports: HashMap<String, SampleExportEntry>,
        steps: Recipe,
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
    pub fn steps(&self) -> &Recipe {
        &self.steps
    }

    pub fn steps_mut(&mut self) -> &mut Recipe {
        &mut self.steps
    }

    #[must_use]
    pub fn secret(&self) -> bool {
        self.secret
    }

    pub fn secret_mut(&mut self) -> &mut bool {
        &mut self.secret
    }

    pub fn imports(&self) -> &[String] {
        &self.imports
    }

    pub fn into_imports(self) -> Vec<String> {
        self.imports
    }

    pub fn exports(&self) -> &HashMap<String, SampleExportEntry> {
        &self.exports
    }

    pub fn into_exports(self) -> HashMap<String, SampleExportEntry> {
        self.exports
    }
}
