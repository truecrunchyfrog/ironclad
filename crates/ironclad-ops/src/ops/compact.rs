use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::Sample,
};

pub(crate) struct Compact;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {}

impl TypedOperation for Compact {
    type Options = ();
    type Error = Error;

    fn description(&self) -> &'static str {
        "Remove samples whose content is empty."
    }

    fn eval_all(
        &self,
        _context: &OperationContext,
        input: Vec<Sample>,
        _options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        Ok(input
            .into_iter()
            .filter(|sample| !sample.content().is_empty())
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ironclad_core::{
        operation::{OperationContext, TypedOperation},
        sample::{Sample, Trace},
    };

    use super::Compact;

    #[test]
    fn removes_empty_samples() {
        let op = Compact;
        let context = OperationContext::for_working_dir(std::env::temp_dir());
        let input = vec![
            Sample::new(Trace::new(HashMap::new()), String::from("hello")),
            Sample::new(Trace::new(HashMap::new()), String::new()),
            Sample::new(Trace::new(HashMap::new()), String::from("world")),
        ];

        let output = op.eval_all(&context, input, ()).expect("compact");

        assert_eq!(output.len(), 2);
        assert_eq!(output[0].content(), "hello");
        assert_eq!(output[1].content(), "world");
    }
}
