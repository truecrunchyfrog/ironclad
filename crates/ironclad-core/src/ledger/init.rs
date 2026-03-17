use std::{fs, path::Path};

use log::info;

use crate::ledger::{error::LedgerError, ledger::Ledger};

impl Ledger {
    pub fn create_ledger(working_dir: &Path) -> Result<Ledger, LedgerError> {
        let ledger = Ledger::new(Ledger::ledger_dir(working_dir));

        populate_ledger_dir(&ledger)?;

        Ok(ledger)
    }
}

fn populate_ledger_dir(ledger: &Ledger) -> Result<(), LedgerError> {
    if ledger.dir().try_exists()? {
        if ledger.dir().is_dir() {
            return Err(LedgerError::PathAlreadyExists(ledger.dir().to_path_buf()));
        }

        return Err(LedgerError::PathNotDirectory(ledger.dir().to_path_buf()));
    }

    {
        info!("creating {:#?}", ledger.dir());
        fs::create_dir(&ledger.dir())?;

        {
            let nodes_dir = ledger.nodes_dir();
            info!("creating {:#?}", nodes_dir);
            fs::create_dir(nodes_dir)?;
        }

        {
            let snapshots_dir = ledger.snapshots_dir();
            info!("creating {:#?}", snapshots_dir);
            fs::create_dir(snapshots_dir)?;

            {
                let snapshot_baseline_path = ledger.snapshot_baseline_path();
                info!("creating {:#?}", snapshot_baseline_path);
                fs::write(snapshot_baseline_path, "{}")?; // TODO
            }

            {
                let snapshot_pending_path = ledger.snapshot_pending_path();
                info!("creating {:#?}", snapshot_pending_path);
                fs::write(snapshot_pending_path, "{}")?; // TODO
            }
        }
    }

    Ok(())
}
