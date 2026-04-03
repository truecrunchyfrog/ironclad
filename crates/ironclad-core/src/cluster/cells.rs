use std::{
    fs::{self, DirEntry},
    path::Path,
};

use log::{info, warn};

use crate::{
    cell::{Cell, error::CellError, id::CellId},
    cluster::{cluster::Cluster, error::ClusterError},
};

impl Cluster {
    fn cell_files(&self) -> Vec<DirEntry> {
        let cells_dir = self.cells_dir();
        let entries = cells_dir
            .read_dir()
            .unwrap_or_else(|_| panic!("cannot read {cells_dir:#?} as directory"));

        entries
            .filter_map(|entry| {
                entry
                    .inspect_err(|err| warn!("strange entry: {err}"))
                    .ok()
                    .filter(|entry| {
                        entry
                            .file_type()
                            .inspect_err(|err| warn!("cannot check file type: {err}"))
                            .is_ok_and(|filetype| {
                                if filetype.is_file() {
                                    true
                                } else {
                                    warn!("non-file cell entry ignored: {:#?}", entry.path());
                                    false
                                }
                            })
                    })
            })
            .collect::<Vec<_>>()
    }

    #[must_use]
    pub fn cell_ids(&self) -> Vec<CellId> {
        self.cell_files()
            .iter()
            .map(|file| CellId::for_path(&file.path()))
            .collect()
    }

    pub fn resolve_cell_id(&self, id: &str) -> Result<CellId, CellError> {
        let cell_ids = self.cell_ids();

        if let Some(cell_id) = cell_ids.iter().find(|cell_id| cell_id.to_string() == id) {
            return Ok(cell_id.clone());
        }

        let mut possible_ids = cell_ids
            .iter()
            .filter(|cell_id| cell_id.to_string().starts_with(id));

        match (possible_ids.next(), possible_ids.next()) {
            (Some(cell_id), None) => Ok(cell_id.clone()),
            (None, _) => Err(CellError::NoSuchCellId(id.to_string())),
            _ => Err(CellError::AmbiguousCellId(id.to_string())),
        }
    }

    pub fn resolve_cell(&self, id: &str) -> Result<Cell, CellError> {
        self.load_cell_for_id(&self.resolve_cell_id(id)?)
    }

    pub fn load_cells(&self) -> Result<Vec<Cell>, ClusterError> {
        Ok(self
            .cell_files()
            .iter()
            .flat_map(|entry| {
                self.load_cell_for_path(&entry.path())
                    .inspect_err(|err| warn!("failed to load cell {:#?}: {}", entry.path(), err))
            })
            .collect::<Vec<_>>())
    }

    pub fn load_cell_for_path(&self, path: &Path) -> Result<Cell, CellError> {
        if !path.try_exists()? {
            return Err(CellError::PathNotFound(path.to_path_buf()));
        }

        let mut cell: Cell = serde_json::from_str(&fs::read_to_string(path)?)?;
        *cell.id_mut() = CellId::for_path(path);

        Ok(cell)
    }

    pub fn load_cell_for_id(&self, id: &CellId) -> Result<Cell, CellError> {
        self.load_cell_for_path(&self.cell_path(id))
    }

    pub fn save_cell(&self, cell: &Cell) -> Result<(), CellError> {
        let path = self.cell_path(cell.id());

        if !path.try_exists()? {
            return Err(CellError::PathNotFound(path));
        }

        write_cell(&path, cell)?;

        Ok(())
    }

    pub fn add_cell(&self, cell: &Cell) -> Result<(), CellError> {
        let path = self.cell_path(cell.id());

        if path.try_exists()? {
            return Err(CellError::PathAlreadyExists(path));
        }

        write_cell(&path, cell)?;

        Ok(())
    }

    pub fn remove_cell(&self, id: &CellId) -> Result<(), ClusterError> {
        let path = self.cell_path(id);

        if !path.try_exists()? {
            return Err(CellError::PathNotFound(path).into());
        }

        info!("removing cell at {path:?}");
        fs::remove_file(path)?;

        let cells = self.load_cells()?;

        for mut cell in cells {
            let deps = cell.dependencies_mut();
            if deps.contains(id) {
                *deps = deps.iter().filter(|dep| dep == &id).cloned().collect();
                self.save_cell(&cell)?;
            }
        }

        Ok(())
    }
}

fn write_cell(path: &Path, cell: &Cell) -> Result<(), CellError> {
    info!("writing cell at {path:?}");
    fs::write(path, serde_json::to_vec_pretty(cell)?)?;
    Ok(())
}
