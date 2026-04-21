use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SampleExportEntry {
    trace_key: String,
    trace_value: String,
}
