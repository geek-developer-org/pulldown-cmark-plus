#[allow(dead_code)]
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};

#[allow(dead_code)]
pub fn to_html(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all()).map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref cowstr))) => {
            let cowstr = cowstr.as_ref().to_owned();
            let mut codeblock = String::from("");

            let mut lang: &str = "";
            let mut file: &str = "";
            for part in text.split(',').collect::<Vec<&str>>() {
                if !lang.is_empty() && !file.is_empty() {
                    break;
                }
                if let Some(pos) = part.find("=") {
                    if file.is_empty() {
                        let (key, val) = (part[..pos].trim(), part[pos + 1..].trim());
                        if key == "file" && !val.is_empty() {
                            file = val;
                        }
                    }
                } else if lang.is_empty()
                    && !part.trim().is_empty()
                    && part.trim() != "linenostart"
                    && part.trim() != "linenos"
                    && part.trim() != "hl_lines"
                    && part.trim() != "hide_lines"
                    && part.trim() != "file"
                {
                    lang = part.trim();
                }
            }

            if !file.is_empty() {
                codeblock.push_str("<div class=\"codeblock-file\"><span>");
                codeblock.push_str(file);
                codeblock.push_str("</span></div>\n");
            }
            codeblock.push_str("<pre><code");
            if !lang.is_empty() {
                codeblock.push_str(" class=\"");
                codeblock.push_str(lang);
                codeblock.push_str(" language-");
                codeblock.push_str(lang);
                codeblock.push_str("\"");
            }
            codeblock.push_str(">");

            Event::Html(codeblock.into())
        }
        _ => event,
    });

    let mut buf = String::new();
    html::push_html(&mut buf, parser);

    buf
}

#[allow(dead_code)]
pub fn get_title(markdown: &str) -> String {
    for line in markdown.lines().collect::<Vec<&str>>() {
        if line.starts_with("#") {
            let title = line.trim_start_matches('#').trim();

            if !title.is_empty() {
                return title.to_string();
            }
        }
    }
    "".to_string()
}
