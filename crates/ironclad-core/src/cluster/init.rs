use std::{fs, path::Path};

use log::info;

use crate::cluster::{cluster::Cluster, error::ClusterError};

const GITIGNORE_CONTENT: &str = "snapshots/pending.json";

impl Cluster {
    pub fn create_cluster(working_dir: &Path) -> Result<Cluster, ClusterError> {
        let cluster = Cluster::new(Cluster::cluster_dir(working_dir));

        populate_cluster_dir(&cluster)?;

        Ok(cluster)
    }
}

fn populate_cluster_dir(cluster: &Cluster) -> Result<(), ClusterError> {
    if cluster.dir().try_exists()? {
        if cluster.dir().is_dir() {
            return Err(ClusterError::PathAlreadyExists(cluster.dir().to_path_buf()));
        }

        return Err(ClusterError::PathNotDirectory(cluster.dir().to_path_buf()));
    }

    {
        info!("creating {:#?}", cluster.dir());
        fs::create_dir(cluster.dir())?;

        let gitignore_path = cluster.dir().join(".gitignore");
        info!("creating {gitignore_path:#?}");
        fs::write(gitignore_path, GITIGNORE_CONTENT)?;

        {
            let facts_dir = cluster.facts_dir();
            info!("creating {facts_dir:#?}");
            fs::create_dir(facts_dir)?;
        }

        {
            let snapshots_dir = cluster.snapshots_dir();
            info!("creating {snapshots_dir:#?}");
            fs::create_dir(snapshots_dir)?;

            {
                let snapshot_baseline_path = cluster.snapshot_baseline_path();
                info!("creating {snapshot_baseline_path:#?}");
                fs::write(snapshot_baseline_path, "{}")?;
            }

            {
                let snapshot_pending_path = cluster.snapshot_pending_path();
                info!("creating {snapshot_pending_path:#?}");
                fs::write(snapshot_pending_path, "{}")?;
            }
        }
    }

    Ok(())
}
