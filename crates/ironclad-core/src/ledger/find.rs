use std::{ffi::OsStr, path::Path};

use crate::ledger::{Ledger, error::LedgerError};

impl Ledger {
    pub(crate) fn is_ledger_dir(path: &Path) -> bool {
        path.file_name() == Some(OsStr::new(".ironclad")) && path.is_dir()
    }

    pub fn find_for_working_dir(working_dir: &Path) -> Result<Ledger, LedgerError> {
        working_dir
            .ancestors()
            .find_map(|ancestor| {
                ancestor
                    .read_dir()
                    .ok()
                    .and_then(|read_dir| {
                        read_dir
                            .flatten()
                            .map(|child| child.path())
                            .find(|child| Ledger::is_ledger_dir(child))
                            .map(Ledger::new)
                    })
            })
            .ok_or_else(|| LedgerError::PathNotFound(working_dir.to_path_buf()))
    }
}
