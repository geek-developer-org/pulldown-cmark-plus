#[allow(dead_code)]
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};

#[allow(dead_code)]
pub fn to_html(markdown: &str) -> String {
    let parser = Parser::new_ext(markdown, Options::all()).map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref cowstr))) => {
            let cowstr = cowstr.as_ref().to_owned();
            let mut codeblock = String::from("");

            let (lang, file, _) = get_lang(&cowstr);

            if !file.is_empty() {
                codeblock.push_str("<div class=\"codeblock-file\"><span>");
                codeblock.push_str(file);
                codeblock.push_str("</span></div>\n");
            }
            codeblock.push_str("<pre><code");
            if !lang.is_empty() {
                codeblock.push_str(" class=\"");
                codeblock.push_str(lang);
                codeblock.push_str("\" data-lang=\"language-");
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

fn get_lang(cowstr: &str) -> (&str, &str, bool) {
    let parts = match cowstr.contains(":") {
        true => cowstr.split(':').collect::<Vec<&str>>(),
        _ => cowstr.split(',').collect::<Vec<&str>>(),
    };
    if cowstr.contains(":") {
        return (
            match parts.get(0) {
                Some(s) => s.trim(),
                None => "",
            },
            match parts.get(1) {
                Some(s) => s.trim(),
                None => "",
            },
            match parts.get(2) {
                Some(s) => s.trim().to_lowercase() == "true",
                None => false,
            },
        );
    }

    let mut lang: &str = "";
    let mut file: &str = "";
    let mut no_style = false;
    for part in parts {
        if !lang.is_empty() && !file.is_empty() && no_style {
            break;
        }
        if let Some(pos) = part.find("=") {
            let (key, val) = (part[..pos].trim(), part[pos + 1..].trim());
            if lang.is_empty() && key.trim() == "lang" && !val.trim().is_empty() {
                lang = val.trim();
            }
            if file.is_empty() && key.trim() == "file" && !val.trim().is_empty() {
                file = val.trim();
            }
            if !no_style && key.trim() == "no_style" && !val.trim().is_empty() {
                no_style = val.trim() == "true";
            }
        } else if lang.is_empty()
            && !part.trim().is_empty()
            && part.trim() != "linenostart"
            && part.trim() != "linenos"
            && part.trim() != "hl_lines"
            && part.trim() != "hide_lines"
            && part.trim() != "file"
            && part.trim() != "no_style"
        {
            lang = part.trim();
        }
    }

    (lang, file, no_style)
}

#[allow(dead_code)]
pub fn get_title(markdown: &str) -> String {
    let mut i: usize = 0;
    for line in markdown.lines().collect::<Vec<&str>>() {
        if line.starts_with("#") {
            let title = line.trim_start_matches('#').trim();

            if !title.is_empty() {
                return title.to_string();
            }
        }
        i += 1;
        if i > 30 {
            break;
        }
    }
    "".to_string()
}
