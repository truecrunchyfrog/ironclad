use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use crate::{operation::OperationContext, recipe::RecipeError, registry::Registry, sample::Sample};

#[derive(Serialize, Debug, Clone)]
pub struct Step {
    operation_id: String,
    options: toml::Value,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum StepDef {
    Shorthand(String),
    Full {
        r#use: String,
        #[serde(default = "empty_options")]
        options: toml::Value,
    },
}

fn empty_options() -> toml::Value {
    toml::Value::Table(Default::default())
}

impl From<StepDef> for Step {
    fn from(value: StepDef) -> Self {
        match value {
            StepDef::Shorthand(operation_id) => Self {
                operation_id,
                options: empty_options(),
            },
            StepDef::Full { r#use, options } => Self {
                operation_id: r#use,
                options,
            },
        }
    }
}

impl<'de> Deserialize<'de> for Step {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(StepDef::deserialize(deserializer)?.into())
    }
}

impl Step {
    #[must_use]
    pub fn new(operation_id: String, options: toml::Value) -> Self {
        Self {
            operation_id,
            options,
        }
    }

    #[must_use]
    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    #[must_use]
    pub fn options(&self) -> &toml::Value {
        &self.options
    }

    pub fn options_mut(&mut self) -> &mut toml::Value {
        &mut self.options
    }

    pub fn eval(
        &self,
        registry: &Registry,
        context: &OperationContext,
        imports: &HashMap<String, &Sample>,
        input: Vec<Sample>,
    ) -> Result<Vec<Sample>, RecipeError> {
        let operation = registry.resolve_op(&self.operation_id)?;
        let options = resolve_imports(self.options.clone(), imports);

        operation
            .eval(context, input, Some(options))
            .map_err(|err| RecipeError::Operation {
                operation_id: self.operation_id.clone(),
                source: err,
            })
    }
}

fn resolve_imports(mut value: toml::Value, imports: &HashMap<String, &Sample>) -> toml::Value {
    visit_toml_strings_mut(&mut value, &mut |s| {
        if let Some(label) = s
            .strip_prefix("$(")
            .and_then(|inner| inner.strip_suffix(')'))
        {
            if let Some(sample) = imports.get(label) {
                *s = sample.content().clone();
            }
        }
    });
    value
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::sample::{Sample, Trace};

    use super::{Step, resolve_imports};

    fn sample(content: &str) -> Sample {
        Sample::new(Trace::new(HashMap::new()), content.to_string())
    }

    #[test]
    fn resolves_exact_import_placeholders() {
        let resolved_sample = sample("resolved");
        let imports = HashMap::from([(String::from("foo"), &resolved_sample)]);
        let value = toml::Value::Table(toml::map::Map::from_iter([
            (
                String::from("exact"),
                toml::Value::String(String::from("$(foo)")),
            ),
            (
                String::from("literal"),
                toml::Value::String(String::from("prefix $(foo) suffix")),
            ),
            (
                String::from("nested"),
                toml::Value::Array(vec![
                    toml::Value::String(String::from("$(foo)")),
                    toml::Value::String(String::from("$(missing)")),
                ]),
            ),
        ]));

        let resolved = resolve_imports(value, &imports);

        let table = resolved.as_table().expect("table");
        assert_eq!(table["exact"].as_str(), Some("resolved"));
        assert_eq!(table["literal"].as_str(), Some("prefix $(foo) suffix"));

        let nested = table["nested"].as_array().expect("array");
        assert_eq!(nested[0].as_str(), Some("resolved"));
        assert_eq!(nested[1].as_str(), Some("$(missing)"));
    }

    #[test]
    fn deserializes_step_from_string() {
        #[derive(serde::Deserialize)]
        struct Wrapper {
            steps: Vec<Step>,
        }

        let wrapper: Wrapper = toml::from_str(
            r#"
steps = ["text.trim"]
"#,
        )
        .expect("deserialize steps");

        let step = &wrapper.steps[0];

        assert_eq!(step.operation_id(), "text.trim");
        assert_eq!(step.options(), &toml::Value::Table(Default::default()));
    }

    #[test]
    fn deserializes_step_from_table_without_options() {
        let step: Step = toml::from_str(
            r#"
use = "compact"
"#,
        )
        .expect("deserialize step");

        assert_eq!(step.operation_id(), "compact");
        assert_eq!(step.options(), &toml::Value::Table(Default::default()));
    }

    #[test]
    fn deserializes_step_from_full_table() {
        let step: Step = toml::from_str(
            r#"
use = "seed.file.text"

[options]
files = ["a.txt"]
"#,
        )
        .expect("deserialize step");

        assert_eq!(step.operation_id(), "seed.file.text");
        assert_eq!(
            step.options(),
            &toml::Value::Table(toml::map::Map::from_iter([(
                String::from("files"),
                toml::Value::Array(vec![toml::Value::String(String::from("a.txt"))]),
            )]))
        );
    }
}
