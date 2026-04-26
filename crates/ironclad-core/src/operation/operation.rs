use serde::de::DeserializeOwned;

use crate::{
    operation::{OperationContext, OperationError},
    sample::Sample,
};

pub trait Operation: Send + Sync {
    fn description(&self) -> &'static str;

    fn eval(
        &self,
        context: &OperationContext,
        input: Vec<Sample>,
        options: Option<toml::Value>,
    ) -> Result<Vec<Sample>, OperationError>;
}

pub trait TypedOperation: Send + Sync + 'static {
    type Options: DeserializeOwned + Clone + Default;
    type Error: std::error::Error + Send + Sync;

    fn description(&self) -> &'static str;

    fn eval_all(
        &self,
        context: &OperationContext,
        input: Vec<Sample>,
        options: Self::Options,
    ) -> Result<Vec<Sample>, Self::Error> {
        input.into_iter().try_fold(Vec::new(), |mut acc, sample| {
            acc.extend(self.eval_each(context, sample, options.clone())?);
            Ok(acc)
        })
    }

    fn eval_each(
        &self,
        #[allow(unused)] context: &OperationContext,
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
        context: &OperationContext,
        input: Vec<Sample>,
        options: Option<toml::Value>,
    ) -> Result<Vec<Sample>, OperationError> {
        self.0
            .eval_all(
                context,
                input,
                options
                    .map(toml::Value::try_into)
                    .transpose()?
                    .unwrap_or_default(),
            )
            .map_err(|err| OperationError::Other(Box::new(err)))
    }
}

impl<T: TypedOperation> From<T> for Box<dyn Operation> {
    fn from(op: T) -> Self {
        Box::new(TypedOperationAdapter(op))
    }
}
