use std::cmp::min;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum SelectionBoundary {
    Lines(usize),
    Bytes(usize),
    Text(String),
}

pub(crate) enum SelectionDirection {
    Ltr,
    Rtl,
}

impl SelectionBoundary {
    pub(crate) fn split<'a>(
        self,
        text: &'a str,
        direction: &SelectionDirection,
    ) -> (&'a str, &'a str, &'a str) {
        match self {
            Self::Lines(lines) => {
                let split_from = if lines > 0 {
                    min(
                        match direction {
                            SelectionDirection::Ltr => text
                                .match_indices('\n')
                                .take(lines)
                                .last()
                                .map(|(index, _)| index)
                                .unwrap_or(text.len()),
                            SelectionDirection::Rtl => text
                                .rmatch_indices('\n')
                                .take(lines)
                                .last()
                                .map(|(index, _)| index)
                                .unwrap_or(0),
                        },
                        text.len(),
                    )
                } else {
                    match direction {
                        SelectionDirection::Ltr => 0,
                        SelectionDirection::Rtl => text.len(),
                    }
                };
                let split_to = if lines > 0 {
                    min(split_from + 1, text.len())
                } else {
                    split_from
                };

                let left = &text[..split_from];
                let middle = &text[split_from..split_to];
                let right = &text[split_to..];

                match direction {
                    SelectionDirection::Ltr => (left, middle, right),
                    SelectionDirection::Rtl => (right, middle, left),
                }
            }
            Self::Bytes(bytes) => {
                let split_on = min(
                    match direction {
                        SelectionDirection::Ltr => bytes,
                        SelectionDirection::Rtl => {
                            text.len().checked_sub(bytes).unwrap_or(text.len())
                        }
                    },
                    text.len(),
                );

                let left = &text[..split_on];
                let middle = &text[split_on..split_on];
                let right = &text[split_on..];

                match direction {
                    SelectionDirection::Ltr => (left, middle, right),
                    SelectionDirection::Rtl => (right, middle, left),
                }
            }
            Self::Text(boundary) => {
                let split_from = match direction {
                    SelectionDirection::Ltr => text.find(&boundary),
                    SelectionDirection::Rtl => text.rfind(&boundary),
                }
                .unwrap_or(text.len().checked_sub(boundary.len()).unwrap_or(0));
                let split_to = min(split_from + boundary.len(), text.len());

                let left = &text[..split_from];
                let middle = &text[split_from..split_to];
                let right = &text[split_to..];

                match direction {
                    SelectionDirection::Ltr => (left, middle, right),
                    SelectionDirection::Rtl => (right, middle, left),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_boundaries() {
        use SelectionBoundary::Lines;
        use SelectionDirection::*;

        let text = "1\n2\n\n\r\n5\r\n6";

        let cases = [
            (Lines(5).split(text, &Rtl), ("2\n\n\r\n5\r\n6", "\n", "1")),
            (Lines(5).split(text, &Ltr), ("1\n2\n\n\r\n5\r", "\n", "6")),
            (Lines(4).split(text, &Rtl), ("\n\r\n5\r\n6", "\n", "1\n2")),
            (Lines(3).split(text, &Ltr), ("1\n2\n", "\n", "\r\n5\r\n6")),
            (Lines(2).split(text, &Ltr), ("1\n2", "\n", "\n\r\n5\r\n6")),
            (Lines(1).split(text, &Ltr), ("1", "\n", "2\n\n\r\n5\r\n6")),
            (Lines(0).split(text, &Ltr), ("", "1", "\n2\n\n\r\n5\r\n6")),
            (Lines(0).split(text, &Rtl), ("\n2\n\n\r\n5\r\n6", "1", "")),
            (Lines(100).split(text, &Ltr), ("1\n2\n\n\r\n5\r", "\n", "6")),
        ];

        for (actual, expected) in cases {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn byte_boundaries() {
        use SelectionBoundary::Bytes;
        use SelectionDirection::*;

        let text = "123\n4\n\n\r\n5678\r\nöe";

        let cases = [
            (
                Bytes(0).split(text, &Rtl),
                ("", "", "123\n4\n\n\r\n5678\r\nöe"),
            ),
            (
                Bytes(0).split(text, &Ltr),
                ("", "1", "23\n4\n\n\r\n5678\r\nöe"),
            ),
            (
                Bytes(1).split(text, &Rtl),
                ("", "e", "123\n4\n\n\r\n5678\r\nö"),
            ),
            (
                Bytes(4).split(text, &Rtl),
                ("öe", "\n", "123\n4\n\n\r\n5678\r"),
            ),
            (
                Bytes(5).split(text, &Ltr),
                ("123\n4", "\n", "\n\r\n5678\r\nöe"),
            ),
            (
                Bytes(100).split(text, &Ltr),
                ("123\n4\n\n\r\n5678\r\nöe", "", ""),
            ),
        ];

        for (actual, expected) in cases {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn text_boundaries() {
        use SelectionBoundary::Text;
        use SelectionDirection::*;

        let text = "123\n4\n\n\r\n5'67'8\r\nöe";

        let cases = [
            (
                Text("5".to_string()).split(text, &Rtl),
                ("'67'8\r\nöe", "5", "123\n4\n\n\r\n"),
            ),
            (
                Text("5'".to_string()).split(text, &Ltr),
                ("123\n4\n\n\r\n", "5'", "67'8\r\nöe"),
            ),
            (
                Text("\n".to_string()).split(text, &Ltr),
                ("123", "\n", "4\n\n\r\n5'67'8\r\nöe"),
            ),
            (
                Text("\n".to_string()).split(text, &Rtl),
                ("öe", "\n", "123\n4\n\n\r\n5'67'8\r"),
            ),
            (
                Text("'67'8\r\nöe".to_string()).split(text, &Ltr),
                ("123\n4\n\n\r\n5", "'67'8\r\nöe", ""),
            ),
        ];

        for (actual, expected) in cases {
            assert_eq!(actual, expected);
        }
    }
}
