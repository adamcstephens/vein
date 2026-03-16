use pulldown_cmark::{Options, Parser, html};

/// Convert markdown text to HTML suitable for Vikunja's TipTap editor.
pub fn markdown_to_html(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

/// Convert HTML from Vikunja back to markdown for agent consumption.
pub fn html_to_markdown(input: &str) -> Result<String, std::io::Error> {
    htmd::convert(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- markdown_to_html tests --

    #[test]
    fn md_to_html_paragraph() {
        assert_eq!(markdown_to_html("hello world"), "<p>hello world</p>\n");
    }

    #[test]
    fn md_to_html_heading() {
        assert_eq!(markdown_to_html("# Title"), "<h1>Title</h1>\n");
    }

    #[test]
    fn md_to_html_bold() {
        assert_eq!(
            markdown_to_html("some **bold** text"),
            "<p>some <strong>bold</strong> text</p>\n"
        );
    }

    #[test]
    fn md_to_html_italic() {
        assert_eq!(
            markdown_to_html("some *italic* text"),
            "<p>some <em>italic</em> text</p>\n"
        );
    }

    #[test]
    fn md_to_html_inline_code() {
        assert_eq!(
            markdown_to_html("use `foo()` here"),
            "<p>use <code>foo()</code> here</p>\n"
        );
    }

    #[test]
    fn md_to_html_list() {
        let input = "- one\n- two";
        let output = markdown_to_html(input);
        assert!(output.contains("<ul>"));
        assert!(output.contains("<li>one</li>"));
        assert!(output.contains("<li>two</li>"));
    }

    #[test]
    fn md_to_html_strikethrough() {
        assert_eq!(
            markdown_to_html("~~deleted~~"),
            "<p><del>deleted</del></p>\n"
        );
    }

    #[test]
    fn md_to_html_link() {
        assert_eq!(
            markdown_to_html("[docs](https://example.com)"),
            "<p><a href=\"https://example.com\">docs</a></p>\n"
        );
    }

    #[test]
    fn md_to_html_empty() {
        assert_eq!(markdown_to_html(""), "");
    }

    #[test]
    fn md_to_html_plain_text() {
        assert_eq!(markdown_to_html("just text"), "<p>just text</p>\n");
    }

    // -- html_to_markdown tests --

    #[test]
    fn html_to_md_paragraph() {
        let result = html_to_markdown("<p>hello world</p>").unwrap();
        assert_eq!(result.trim(), "hello world");
    }

    #[test]
    fn html_to_md_heading() {
        let result = html_to_markdown("<h1>Title</h1>").unwrap();
        assert_eq!(result.trim(), "# Title");
    }

    #[test]
    fn html_to_md_bold() {
        let result = html_to_markdown("<p>some <strong>bold</strong> text</p>").unwrap();
        assert_eq!(result.trim(), "some **bold** text");
    }

    #[test]
    fn html_to_md_link() {
        let result = html_to_markdown("<p><a href=\"https://example.com\">docs</a></p>").unwrap();
        assert_eq!(result.trim(), "[docs](https://example.com)");
    }

    #[test]
    fn html_to_md_empty() {
        let result = html_to_markdown("").unwrap();
        assert_eq!(result, "");
    }

    // -- round-trip tests --

    #[test]
    fn roundtrip_simple_text() {
        let md = "hello world";
        let html = markdown_to_html(md);
        let back = html_to_markdown(&html).unwrap();
        assert_eq!(back.trim(), md);
    }

    #[test]
    fn roundtrip_bold() {
        let md = "some **bold** text";
        let html = markdown_to_html(md);
        let back = html_to_markdown(&html).unwrap();
        assert_eq!(back.trim(), md);
    }

    #[test]
    fn roundtrip_heading() {
        let md = "# Title";
        let html = markdown_to_html(md);
        let back = html_to_markdown(&html).unwrap();
        assert_eq!(back.trim(), md);
    }
}
