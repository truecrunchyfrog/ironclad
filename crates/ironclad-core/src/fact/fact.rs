use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{fact::SampleExportEntry, recipe::Recipe};

#[derive(Serialize, Deserialize, Debug)]
pub struct Fact {
    description: Option<String>,
    #[serde(default)]
    import: Vec<String>,
    #[serde(default)]
    export: HashMap<String, SampleExportEntry>,
    steps: Recipe,
    #[serde(default)]
    secret: bool,
}

impl Fact {
    #[must_use]
    pub fn new(
        description: Option<String>,
        import: Vec<String>,
        export: HashMap<String, SampleExportEntry>,
        steps: Recipe,
        secret: bool,
    ) -> Self {
        Self {
            description,
            import,
            export,
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
}
