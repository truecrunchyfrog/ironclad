use std::collections::HashMap;

use ironclad_core::{
    operation::{OperationContext, TypedOperation},
    sample::{Sample, Trace},
};

pub(crate) struct TextTrim;

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {}

impl TypedOperation for TextTrim {
    type Options = ();
    type Error = Error;

    fn description(&self) -> &'static str {
        "Trim leading and trailing whitespace from each sample."
    }

    fn eval_each(
        &self,
        _context: &OperationContext,
        input: Sample,
        _options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        Ok(vec![input.evolve(
            Trace::new(HashMap::new()),
            input.content().trim().to_string(),
        )])
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ironclad_core::{
        operation::{OperationContext, TypedOperation},
        sample::{Sample, Trace},
    };

    use super::TextTrim;

    #[test]
    fn trims_sample_content() {
        let op = TextTrim;
        let context = OperationContext::for_working_dir(std::env::temp_dir());
        let input = Sample::new(Trace::new(HashMap::new()), String::from("  hello \n"));

        let output = op.eval_each(&context, input, ()).expect("trim");

        assert_eq!(output.len(), 1);
        assert_eq!(output[0].content(), "hello");
    }
}
