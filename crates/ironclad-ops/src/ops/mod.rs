use rnacl_core::operation::Operation;

mod head_file_text;
mod head_net_http;
mod json_find;
mod slice;
mod text_find;
mod text_lines;
mod text_replace;
mod text_split;
mod text_tag;
// mod text_filter;
mod html_attribute;
mod html_find;
mod html_inner_html;
mod html_inner_text;
// TODO Tree-sitter?

pub(crate) fn operations() -> Vec<(&'static str, Box<dyn Operation>)> {
    vec![
        ("head.file.text", head_file_text::HeadFileText.into()),
        ("head.net.http", head_net_http::HeadNetHttp.into()),
        ("slice", slice::Slice.into()),
        ("text.find", text_find::TextFind.into()),
        ("text.split", text_split::TextSplit.into()),
        ("text.lines", text_lines::TextLines.into()),
        ("text.replace", text_replace::TextReplace.into()),
        ("text.tag", text_tag::TextTag.into()),
        ("json.find", json_find::JsonFind.into()),
        ("html.find", html_find::HtmlFind.into()),
        ("html.inner.html", html_inner_html::HtmlInnerHtml.into()),
        ("html.inner.text", html_inner_text::HtmlInnerText.into()),
        ("html.attribute", html_attribute::HtmlAttribute.into()),
    ]
}
