use crate::tag::{selection::Selection, selection_boundary::SelectionDirection};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TagRules {
    pub(crate) left: Selection,
    pub(crate) right: Selection,
}

pub(crate) enum TagRule {
    Left(Selection),
    Right(Selection),
}

impl TagRule {
    pub(crate) fn selection(self) -> Selection {
        match self {
            Self::Left(s) => s,
            Self::Right(s) => s,
        }
    }

    pub(crate) fn select(self, text: &str) -> String {
        let direction = match self {
            Self::Left(_) => SelectionDirection::Rtl,
            Self::Right(_) => SelectionDirection::Ltr,
        };

        let selection = self.selection();
        let (_, _, after_drop) = selection.drop.split(text, &direction);
        let (take, boundary, _) = selection.take.split(after_drop, &direction);

        if selection.inclusive {
            match direction {
                SelectionDirection::Ltr => format!("{take}{boundary}"),
                SelectionDirection::Rtl => format!("{boundary}{take}"),
            }
        } else {
            take.to_string()
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Tag {
    pub(crate) id: String,
    pub(crate) rules: TagRules,
}
