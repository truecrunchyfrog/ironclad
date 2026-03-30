use chumsky::prelude::*;

use crate::tag::tag::TagRule;
use crate::tag::{
    selection::Selection,
    selection_boundary::SelectionBoundary,
    tag::{Tag, TagRules},
};

type Error<'src> = extra::Full<Rich<'src, char>, (), ()>;

pub(crate) fn number_parser<'src>() -> impl Parser<'src, &'src str, usize, Error<'src>> {
    text::int(10).from_str().unwrapped()
}

pub(crate) fn selection_boundary_lines_notation_parser<'src>()
-> impl Parser<'src, &'src str, SelectionBoundary, Error<'src>> {
    number_parser()
        .then_ignore(just('L'))
        .map(SelectionBoundary::Lines)
}

pub(crate) fn selection_boundary_bytes_notation_parser<'src>()
-> impl Parser<'src, &'src str, SelectionBoundary, Error<'src>> {
    number_parser()
        .then_ignore(just('B'))
        .map(SelectionBoundary::Bytes)
}

pub(crate) fn selection_boundary_text_match_notation_parser<'src>()
-> impl Parser<'src, &'src str, SelectionBoundary, Error<'src>> {
    let escaped_char = just('\\').ignore_then(choice((
        just('\\').to('\\'),
        just('\'').to('\''),
        just('n').to('\n'),
        just('r').to('\r'),
        just('t').to('\t'),
    )));

    let normal_char = none_of("\\'");

    just('\'')
        .ignore_then(
            choice((escaped_char, normal_char))
                .repeated()
                .collect::<String>(),
        )
        .then_ignore(just('\''))
        .map(SelectionBoundary::Text)
}

pub(crate) fn selection_boundary_parser<'src>()
-> impl Parser<'src, &'src str, SelectionBoundary, Error<'src>> {
    choice((
        selection_boundary_lines_notation_parser(),
        selection_boundary_bytes_notation_parser(),
        selection_boundary_text_match_notation_parser(),
    ))
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ArrowDirection {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct Arrow {
    direction: ArrowDirection,
    inclusive: bool,
}

pub(crate) fn arrow_parser<'src>() -> impl Parser<'src, &'src str, Arrow, Error<'src>> {
    let arrow_left = just("<-");
    let arrow_right = just("->");
    let pipe = just('|');

    choice((
        pipe.or_not().then_ignore(arrow_left).map(|pipe| Arrow {
            direction: ArrowDirection::Left,
            inclusive: pipe.is_none(),
        }),
        arrow_right.ignore_then(pipe.or_not()).map(|pipe| Arrow {
            direction: ArrowDirection::Right,
            inclusive: pipe.is_none(),
        }),
    ))
}

pub(crate) fn tag_rule_parser<'src>() -> impl Parser<'src, &'src str, TagRule, Error<'src>> {
    selection_boundary_parser()
        .or_not()
        .then(arrow_parser())
        .then(selection_boundary_parser().or_not())
        .map(|((left_boundary, arrow), right_boundary)| match arrow {
            Arrow {
                direction: ArrowDirection::Left,
                inclusive,
            } => TagRule::Left(Selection::new(right_boundary, left_boundary, inclusive)),
            Arrow {
                direction: ArrowDirection::Right,
                inclusive,
            } => TagRule::Right(Selection::new(left_boundary, right_boundary, inclusive)),
        })
}

pub(crate) fn tag_rules_parser<'src>() -> impl Parser<'src, &'src str, TagRules, Error<'src>> {
    tag_rule_parser()
        .padded()
        .repeated()
        .at_most(2)
        .collect::<Vec<_>>()
        .map(|selections| {
            let mut left = None;
            let mut right = None;

            for sel in selections {
                match sel {
                    TagRule::Left(selection) => left = Some(selection),
                    TagRule::Right(selection) => right = Some(selection),
                }
            }

            TagRules {
                left: left.unwrap_or_else(|| Selection::new(None, None, true)),
                right: right.unwrap_or_else(|| Selection::new(None, None, true)),
            }
        })
}

pub(crate) fn tag_parser<'src>() -> impl Parser<'src, &'src str, Tag, Error<'src>> {
    just("~ic=")
        .ignore_then(
            none_of("= \t\r\n")
                .repeated()
                .at_least(1)
                .collect::<String>(),
        )
        .then(
            just("=(")
                .ignore_then(tag_rules_parser())
                .then_ignore(just(')'))
                .or_not(),
        )
        .map(|(id, rules)| Tag {
            id,
            rules: rules.unwrap_or_else(|| TagRules {
                left: Selection::new(None, None, true),
                right: Selection::new(None, None, true),
            }),
        })
}

pub(crate) fn tags_in_arbitrary_text_parser<'src>()
-> impl Parser<'src, &'src str, Vec<Spanned<Tag>>, Error<'src>> {
    any()
        .and_is(tag_parser().not())
        .repeated()
        .ignore_then(tag_parser().spanned())
        .repeated()
        .collect()
        .then_ignore(any().and_is(tag_parser().not()).repeated())
        .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use chumsky::prelude::*;

    use crate::tag::{
        parser::{ArrowDirection, selection_boundary_parser},
        selection::Selection,
        selection_boundary::SelectionBoundary,
        tag::TagRules,
    };

    #[test]
    fn parse_numbers_ok() {
        let cases = [
            ("0", 0),
            ("1", 1),
            ("9", 9),
            ("10", 10),
            ("1234", 1234),
            ("238410234", 238410234),
        ];

        for (serialized, expected_result) in cases {
            assert_eq!(
                super::number_parser()
                    .parse(serialized)
                    .into_output()
                    .unwrap(),
                expected_result
            );
        }
    }

    #[test]
    fn parse_numbers_err() {
        let cases = [
            "", ".", "1.", "1.0", "_", "100_00", "00", "01", "-1", "1-", "-0-",
        ];

        for serialized in cases {
            let result = super::number_parser().parse(serialized);
            assert!(result.into_result().is_err());
        }
    }

    #[test]
    fn parse_selection_boundaries() {
        use SelectionBoundary::*;

        let cases = [
            ("1L", Lines(1)),
            ("99900130L", Lines(99900130)),
            ("0L", Lines(0)),
            ("1B", Bytes(1)),
            ("99900130B", Bytes(99900130)),
            ("0B", Bytes(0)),
            ("''", Text(String::from(""))),
            ("'a'", Text(String::from("a"))),
            (
                "'abc-_*+0123=joasdf'",
                Text(String::from("abc-_*+0123=joasdf")),
            ),
            ("'\"'", Text(String::from("\""))),
            ("'\\''", Text(String::from("'"))),
            ("'\\'some text\\''", Text(String::from("'some text'"))),
            (
                "'\\'first, then \\'some text\\''",
                Text(String::from("'first, then 'some text'")),
            ),
            ("'\\n'", Text(String::from("\n"))),
            ("'\n'", Text(String::from("\n"))),
        ];

        for (serialized, expected_result) in cases {
            assert_eq!(
                selection_boundary_parser()
                    .parse(serialized)
                    .into_result()
                    .unwrap(),
                expected_result
            );
        }
    }

    #[test]
    fn parse_arrows() {
        use crate::tag::parser::Arrow;

        let cases = [
            (
                "<-",
                Arrow {
                    direction: ArrowDirection::Left,
                    inclusive: true,
                },
            ),
            (
                "->",
                Arrow {
                    direction: ArrowDirection::Right,
                    inclusive: true,
                },
            ),
            (
                "|<-",
                Arrow {
                    direction: ArrowDirection::Left,
                    inclusive: false,
                },
            ),
            (
                "->|",
                Arrow {
                    direction: ArrowDirection::Right,
                    inclusive: false,
                },
            ),
        ];

        for (serialized_arrow, expected) in cases {
            assert_eq!(
                super::arrow_parser().parse(serialized_arrow).into_result(),
                Ok(expected)
            );
        }
    }

    #[test]
    fn parse_tag_no_rules() {
        let serialized_tag = "~ic=123";
        let tag = super::tag_parser()
            .parse(serialized_tag)
            .into_result()
            .unwrap();
        assert_eq!(tag.id, "123");
    }

    #[test]
    fn parse_tag_unicode_ids() {
        let cases = vec![
            "a",
            "tag",
            "hello_world",
            "123",
            "αβγ",
            "こんにちは",
            "🚀",
            "multiwordtag",
            "with-dash",
            "with_underscore",
            "with.dot",
            "CAPS",
            "!@#$%^&*()",
            "[]",
            "[][]",
            "δ",
            "🚀🔥🌍✨💡",
            "αβγδεζηθικλμνξοπρστυφχψω",
            "longlonglonglonglonglonglonglong",
            "mixed123αβ🚀",
            "with/slash",
            "with:colon",
            "with;semicolon",
            "with,comma",
            "with?question",
            "with!exclaim",
            "with|pipe",
            "with~tilde",
            "with`backtick`",
            "with'quote'",
            "with\"doublequote\"",
        ];

        for id in cases {
            let serialized_tag = format!("~ic={}", id);
            let tag = super::tag_parser()
                .parse(&serialized_tag)
                .into_result()
                .unwrap();

            assert_eq!(tag.id, id);
        }
    }

    #[test]
    fn cut_off_ids() {
        let cases = vec![("~ic=123 456", "123")];

        for (serialized_tag, expected_id) in cases {
            let mut tags = super::tags_in_arbitrary_text_parser()
                .parse(serialized_tag)
                .into_result()
                .unwrap()
                .into_iter();

            assert!(matches!(tags.next(), Some(tag) if tag.id == expected_id));
        }
    }

    #[test]
    fn malformed_tags() {
        let cases = vec![
            "ic=123",
            "~ic=",
            "~ic =123=()",
            "~ic = 123 = ()",
            "~ic=",
            "~ic= ",
            "~ic= 123",
            "~ic=\n123",
        ];

        for case in cases {
            assert!(super::tag_parser().parse(case).into_result().is_err());
        }
    }

    #[test]
    fn with_rules() {
        use SelectionBoundary::*;

        let cases = [
            (
                "",
                TagRules {
                    left: Selection::new(None, None, true),
                    right: Selection::new(None, None, true),
                },
            ),
            (
                "<- ->",
                TagRules {
                    left: Selection::new(None, None, true),
                    right: Selection::new(None, None, true),
                },
            ),
            (
                "|<- ->|",
                TagRules {
                    left: Selection::new(None, None, false),
                    right: Selection::new(None, None, false),
                },
            ),
            (
                "1L<-0L",
                TagRules {
                    left: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: true,
                    },
                    right: Selection::new(None, None, true),
                },
            ),
            (
                "0L->1L",
                TagRules {
                    left: Selection::new(None, None, true),
                    right: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: true,
                    },
                },
            ),
            (
                "1L<-0L 0L->1L",
                TagRules {
                    left: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: true,
                    },
                    right: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: true,
                    },
                },
            ),
            (
                "0L->1L 1L<-0L",
                TagRules {
                    left: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: true,
                    },
                    right: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: true,
                    },
                },
            ),
            (
                "1L<-0L0L->1L",
                TagRules {
                    left: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: true,
                    },
                    right: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: true,
                    },
                },
            ),
            (
                " \n 1L<-0L \n \n \n 0L->1L \n ",
                TagRules {
                    left: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: true,
                    },
                    right: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: true,
                    },
                },
            ),
            (
                "100L<-10L 10L->100L",
                TagRules {
                    left: Selection {
                        drop: Lines(10),
                        take: Lines(100),
                        inclusive: true,
                    },
                    right: Selection {
                        drop: Lines(10),
                        take: Lines(100),
                        inclusive: true,
                    },
                },
            ),
            (
                "10B<-20L 30B->40L",
                TagRules {
                    left: Selection {
                        drop: Lines(20),
                        take: Bytes(10),
                        inclusive: true,
                    },
                    right: Selection {
                        drop: Bytes(30),
                        take: Lines(40),
                        inclusive: true,
                    },
                },
            ),
            (
                "1L|<-0L 0L->|1L",
                TagRules {
                    left: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: false,
                    },
                    right: Selection {
                        drop: Lines(0),
                        take: Lines(1),
                        inclusive: false,
                    },
                },
            ),
            (
                "'start'<- ->'end'",
                TagRules {
                    left: Selection::new(None, Some(Text("start".to_string())), true),
                    right: Selection::new(None, Some(Text("end".to_string())), true),
                },
            ),
            (
                "'\\'the beginning\\''<-5L ->'here\\'s\tthe\nend.\\n'",
                TagRules {
                    left: Selection::new(
                        Some(Lines(5)),
                        Some(Text("'the beginning'".to_string())),
                        true,
                    ),
                    right: Selection::new(
                        None,
                        Some(Text("here's\tthe\nend.\n".to_string())),
                        true,
                    ),
                },
            ),
        ];

        for (serialized, expected) in cases {
            let rules = super::tag_rules_parser().parse(serialized).into_result();
            assert_eq!(rules, Ok(expected));
        }
    }
}
