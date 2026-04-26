use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{catalog::Catalog, recipe::RecipeError, registry::Registry, sample::Sample};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Step {
    #[serde(rename = "use")]
    operation_id: String,
    options: toml::Value,
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
        catalog: &Catalog,
        imports: &HashMap<String, &Sample>,
        input: Vec<Sample>,
    ) -> Result<Vec<Sample>, RecipeError> {
        let operation = registry.resolve_op(&self.operation_id)?;
        let options = resolve_imports(self.options.clone(), imports);

        operation
            .eval(catalog, input, Some(options))
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

    use super::resolve_imports;

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
}
