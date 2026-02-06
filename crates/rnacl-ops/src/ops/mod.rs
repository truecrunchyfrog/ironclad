use rnacl_core::operation::Operation;

mod head_file_text;
mod head_net_http;
mod json_find;
mod slice;
mod text_find;
mod text_lines;
mod text_replace;
mod text_split;
// mod text_tag;
// mod text_filter;
mod html_document_find;
mod html_fragment_attribute;
mod html_fragment_find;
mod html_fragment_inner_html;
mod html_fragment_inner_text;
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
        ("json.find", json_find::JsonFind.into()),
        (
            "html.document.find",
            html_document_find::HtmlDocumentFind.into(),
        ),
        (
            "html.fragment.find",
            html_fragment_find::HtmlFragmentFind.into(),
        ),
        (
            "html.fragment.inner.html",
            html_fragment_inner_html::HtmlFragmentInnerHtml.into(),
        ),
        (
            "html.fragment.inner.text",
            html_fragment_inner_text::HtmlFragmentInnerText.into(),
        ),
        (
            "html.fragment.attribute",
            html_fragment_attribute::HtmlFragmentAttribute.into(),
        ),
    ]
}
