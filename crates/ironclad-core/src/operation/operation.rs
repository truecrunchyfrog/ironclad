use serde::de::DeserializeOwned;

use crate::{catalog::Catalog, operation::OperationError, sample::Sample};

pub trait Operation: Send + Sync {
    fn description(&self) -> &'static str;

    fn eval(
        &self,
        catalog: &Catalog,
        input: Vec<Sample>,
        options: serde_json::Value,
    ) -> Result<Vec<Sample>, OperationError>;
}

pub trait TypedOperation: Send + Sync + 'static {
    type Options: DeserializeOwned + Clone;
    type Error: std::error::Error + Send + Sync;

    fn description(&self) -> &'static str;

    fn eval_all(
        &self,
        catalog: &Catalog,
        input: Vec<Sample>,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        input.into_iter().try_fold(Vec::new(), |mut acc, sample| {
            acc.extend(self.eval_each(catalog, sample, options.clone())?);
            Ok(acc)
        })
    }

    fn eval_each(
        &self,
        #[allow(unused)] catalog: &Catalog,
        input: Sample,
        #[allow(unused)] options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        Ok(vec![input])
    }
}

struct TypedOperationAdapter<T: TypedOperation>(T);

impl<T: TypedOperation> Operation for TypedOperationAdapter<T> {
    fn description(&self) -> &'static str {
        self.0.description()
    }

    fn eval(
        &self,
        catalog: &Catalog,
        input: Vec<Sample>,
        options: serde_json::Value,
    ) -> Result<Vec<Sample>, OperationError> {
        self.0
            .eval_all(
                catalog,
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
