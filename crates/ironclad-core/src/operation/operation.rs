use serde::de::DeserializeOwned;

use crate::{ledger::Ledger, operation::OperationError, sample::Sample};

pub trait Operation: Send + Sync {
    fn description(&self) -> &'static str;

    fn eval(
        &self,
        ledger: &Ledger,
        input: Vec<Vec<Sample>>,
        options: serde_json::Value,
    ) -> Result<Vec<Vec<Sample>>, OperationError>;
}

pub enum SampleEvolution {
    Drop,
    Transform(Sample),
    Split(Vec<Sample>),
}

pub trait TypedOperation: Send + Sync + 'static {
    type Options: DeserializeOwned + Clone;
    type Error: std::error::Error + Send + Sync;

    fn description(&self) -> &'static str;

    fn eval(
        &self,
        ledger: &Ledger,
        input: Vec<Vec<Sample>>,
        options: Self::Options,
    ) -> Result<Vec<Vec<Sample>>, Self::Error> {
        input.into_iter().try_fold(Vec::new(), |mut result, batch| {
            result.extend(self.eval_sample_set(ledger, batch, options.clone())?);
            Ok(result)
        })
    }

    fn eval_sample_set(
        &self,
        ledger: &Ledger,
        input: Vec<Sample>,
        options: Self::Options,
    ) -> Result<Vec<Vec<Sample>>, Self::Error> {
        let (old_set, new_sets) = input.into_iter().try_fold(
            (vec![], vec![]),
            |(mut old_set, mut new_sets), sample| {
                match self.eval_sample(ledger, sample, options.clone())? {
                    SampleEvolution::Drop => (),
                    SampleEvolution::Transform(sample) => old_set.push(sample),
                    SampleEvolution::Split(samples) => new_sets.push(samples),
                }

                Ok((old_set, new_sets))
            },
        )?;

        let mut sample_sets = new_sets;

        if !old_set.is_empty() {
            sample_sets.push(old_set);
        }

        Ok(sample_sets)
    }

    fn eval_sample(
        &self,
        #[allow(unused)] ledger: &Ledger,
        input: Sample,
        #[allow(unused)] options: Self::Options,
    ) -> Result<SampleEvolution, Self::Error> {
        Ok(SampleEvolution::Transform(input))
    }
}

struct TypedOperationAdapter<T: TypedOperation>(T);

impl<T: TypedOperation> Operation for TypedOperationAdapter<T> {
    fn description(&self) -> &'static str {
        self.0.description()
    }

    fn eval(
        &self,
        ledger: &Ledger,
        input: Vec<Vec<Sample>>,
        options: serde_json::Value,
    ) -> Result<Vec<Vec<Sample>>, OperationError> {
        self.0
            .eval(
                ledger,
                input,
                serde_json::from_value::<T::Options>(options)?,
            )
            .map_err(|err| OperationError::Other(Box::new(err)))
    }
}

impl<T: TypedOperation> From<T> for Box<dyn Operation> {
    fn from(op: T) -> Self {
        Box::new(TypedOperationAdapter(op))
    }
}
