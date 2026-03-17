use crate::tag::selection_boundary::SelectionBoundary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Selection {
    pub(crate) drop: SelectionBoundary,
    pub(crate) take: SelectionBoundary,
    pub(crate) inclusive: bool,
}

impl Selection {
    pub(crate) fn new(
        drop: Option<SelectionBoundary>,
        take: Option<SelectionBoundary>,
        inclusive: bool,
    ) -> Self {
        Self {
            drop: drop.unwrap_or(SelectionBoundary::Lines(0)),
            take: take.unwrap_or(SelectionBoundary::Lines(1)),
            inclusive,
        }
    }
}
