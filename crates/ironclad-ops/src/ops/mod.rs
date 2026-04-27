use ironclad_core::operation::Operation;

mod json_find;
mod run;
mod seed_file_text;
mod seed_net_http;
mod seed_run;
mod slice;
mod text_find;
mod text_lines;
mod text_replace;
mod text_split;
mod text_tag;
// mod text_filter; TODO
mod html_attribute;
mod html_find;
mod html_inner_html;
mod html_inner_text;

pub(crate) fn operations() -> Vec<(&'static str, Box<dyn Operation>)> {
    vec![
        ("seed.file.text", seed_file_text::SeedFileText.into()),
        ("seed.net.http", seed_net_http::SeedNetHttp.into()),
        ("seed.run", seed_run::SeedRun.into()),
        ("run", run::Run.into()),
        ("html.attribute", html_attribute::HtmlAttribute.into()),
        ("html.find", html_find::HtmlFind.into()),
        ("html.inner.html", html_inner_html::HtmlInnerHtml.into()),
        ("html.inner.text", html_inner_text::HtmlInnerText.into()),
        ("json.find", json_find::JsonFind.into()),
        ("slice", slice::Slice.into()),
        ("text.find", text_find::TextFind.into()),
        ("text.lines", text_lines::TextLines.into()),
        ("text.replace", text_replace::TextReplace.into()),
        ("text.split", text_split::TextSplit.into()),
        ("text.tag", text_tag::TextTag.into()),
    ]
}
