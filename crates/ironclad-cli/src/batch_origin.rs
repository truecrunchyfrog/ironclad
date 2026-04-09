use ironclad_core::cell::id::CellId;

#[derive(PartialEq)]
pub(crate) enum BatchOrigin {
    DirtyCell(CellId),
    StaleDependencyCell {
        dependency: CellId,
        dependent: CellId,
    },
}
