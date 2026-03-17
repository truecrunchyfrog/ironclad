use std::{ffi::OsStr, path::Path};

use crate::ledger::{Ledger, error::LedgerError};

impl Ledger {
    pub(crate) fn is_ledger_dir(path: &Path) -> bool {
        path.file_name() == Some(OsStr::new(".rnacl")) && path.is_dir()
    }

    pub fn find_for_working_dir(working_dir: &Path) -> Result<Ledger, LedgerError> {
        working_dir
            .ancestors()
            .find_map(|ancestor| {
                ancestor
                    .read_dir()
                    .ok()
                    .map(|read_dir| {
                        read_dir
                            .flatten()
                            .map(|child| child.path())
                            .find(|child| Ledger::is_ledger_dir(child))
                            .map(|ledger_dir| Ledger::new(ledger_dir))
                    })
                    .flatten()
            })
            .ok_or_else(|| LedgerError::PathNotFound(working_dir.to_path_buf()))
    }
}
