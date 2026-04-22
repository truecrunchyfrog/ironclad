use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SampleExportEntry {
    pub trace_key: String,
    pub trace_value: String,
}
