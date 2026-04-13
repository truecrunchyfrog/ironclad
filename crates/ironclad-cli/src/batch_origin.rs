use ironclad_core::fact::id::FactId;

#[derive(PartialEq)]
pub(crate) enum BatchOrigin {
    DirtyFact(FactId),
    StaleDependencyFact {
        dependency: FactId,
        dependent: FactId,
    },
}
