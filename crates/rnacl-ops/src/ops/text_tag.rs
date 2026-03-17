use std::collections::HashMap;

use chumsky::prelude::*;
use rnacl_core::{
    ledger::Ledger,
    operation::{SampleEvolution, TypedOperation},
    sample::{Sample, Trace},
};
use serde::Deserialize;

use crate::tag::tag::TagRule;

pub(crate) struct TextTag;

#[derive(Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub(crate) struct Options {
    tag: String,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum Error {}

impl TypedOperation for TextTag {
    type Options = Options;
    type Error = Error;

    fn description(&self) -> &'static str {
        "Find tags."
    }

    fn eval_sample(
        &self,
        _ledger: &Ledger,
        input: Sample,
        options: Self::Options,
    ) -> Result<SampleEvolution, Self::Error> {
        /*

        Only this line:
            no
            yes ~ic=123
            no
        Only this line:
            no
            yes ~ic=123=(1L<-0L 0L->1L)
            no
        Three lines above:
            no
            yes
            yes
            yes
            no ~ic=123=(3L<-1L)
            no
        This line and three lines above:
            no
            yes
            yes
            yes
            yes ~ic=123=(4L<-)no
            no
        Append 'b' to use bytes instead of lines:
            no
            noyes~ic=123=(3B<-)no
            no
        Four lines below:
            no
            no ~ic=123=(1L->4L)
            yes
            yes
            yes
            yes
            no
        One line above and three lines below:
            no
            yes
            no ~ic=123=(1L<-1L 1L->3L)
            yes
            yes
            yes
            no
        From line above until text appears (excluding boundary):
            no
            boundary
            yes
            no ~ic=123=('boundary'|<-1L)
            no
        Until text appears after tag (excluding boundary):
            no
            no ~ic=123=(1L->|'boundary')
            yes
            boundary
            no
        Until text appears after tag (including boundary):
            no
            no ~ic=123=(1L->'boundary')
            yes
            boundary
            no
        All text within boundary except current line:
            no
            boundary
            yes
            no ~ic=123=('boundary'<-1L 1L->'boundary')
            yes
            boundary
            no
        All text within boundary:
            no
            boundary
            yes
            yes ~ic=123=('boundary'<- ->'boundary')
            yes
            boundary
            no

        The tag itself is removed from the output.

        */

        let parser = crate::tag::parser::tag_parser();
        let parse = parser.parse(input.content());
        dbg!(parse.output());
        dbg!(parse.errors().collect::<Vec<_>>());
        todo!()

        // let content = input.content();
        // let mut tags = Vec::new();

        // let mut pos = 0;
        // while pos < content.len() {
        //     let slice = &content[pos..];
        //     match parser.parse(slice).into_result() {
        //         Ok(tag) => {
        //             if tag.id == options.tag {
        //                 let rules = tag.inner.rules;

        //                 dbg!(&rules);

        //                 let left_text = SelectionKind::Left(rules.left).select(&content[..pos]);
        //                 let right_text = SelectionKind::Right(rules.right)
        //                     .select(&content[pos + tag.span.end..]);

        //                 tags.push(input.evolve(
        //                     Trace::new(HashMap::from([
        //                         ("start".to_string(), (pos + tag.span.start).to_string()),
        //                         ("end".to_string(), (pos + tag.span.end).to_string()),
        //                     ])),
        //                     format!("{}{}", left_text, right_text),
        //                 ))
        //             }
        //             pos += tag.span.end;
        //         }
        //         Err(_) => pos += slice.chars().next().unwrap().len_utf8(),
        //     };
        // }

        // Ok(SampleEvolution::Split(tags))
    }
}
