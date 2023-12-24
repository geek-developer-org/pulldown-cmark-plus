use once_cell::sync::Lazy;
use pulldown_cmark::{html, CodeBlockKind, Event, Options, Parser, Tag};

use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());

#[allow(dead_code)]
pub fn to_html(markdown: &str) -> String {
    let mut syntax = None;
    let mut in_codeblock = false;
    let mut source = String::new();

    let parser = Parser::new_ext(markdown, Options::all()).map(|event| match event {
        Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref cowstr))) => {
            in_codeblock = true;

            let cowstr = cowstr.as_ref().to_owned();
            let mut codeblock = String::from("");

            let (lang, file, no_style) = get_lang(&cowstr);

            if !file.is_empty() {
                codeblock.push_str("<div class=\"codeblock-file\"><span>");
                codeblock.push_str(file);
                codeblock.push_str("</span></div>\n");
            }
            codeblock.push_str("<pre");
            if !no_style {
                codeblock.push_str(" class=\"code\"");
            }
            codeblock.push_str("><code");

            if !lang.is_empty() {
                codeblock.push_str(" class=\"");
                codeblock.push_str(lang);
                codeblock.push_str("\" data-lang=\"language-");
                codeblock.push_str(lang);
                codeblock.push_str("\"");

                syntax = SYNTAX_SET.find_syntax_by_extension(lang);
            }
            codeblock.push_str(">");

            Event::Html(codeblock.into())
        }

        Event::Text(cowstr) => {
            let s = match in_codeblock {
                true => {
                    source.push_str(&cowstr);
                    "".to_string()
                }
                _ => cowstr.as_ref().to_owned(),
            };
            Event::Text(s.into())
        }

        Event::End(Tag::CodeBlock(_)) if in_codeblock => {
            in_codeblock = false;

            let mut hg = ClassedHTMLGenerator::new_with_class_style(
                syntax.unwrap_or(SYNTAX_SET.find_syntax_plain_text()),
                &SYNTAX_SET,
                ClassStyle::Spaced,
            );
            for line in LinesWithEndings::from(&source) {
                hg.parse_html_for_line_which_includes_newline(line).unwrap();
            }
            let mut html = hg.finalize();
            html.push_str("</code></pre>\n");

            source = String::new();
            syntax = None;

            Event::Html(html.into())
        }

        _ => event,
    });

    let mut buf = String::new();
    html::push_html(&mut buf, parser);

    buf
}

fn get_lang(cowstr: &str) -> (&str, &str, bool) {
    let mut lang: &str = "";
    let mut file: &str = "";
    let mut no_style: bool = false;

    let parts = match cowstr.contains(":") {
        true => cowstr.split(':').collect::<Vec<&str>>(),
        _ => cowstr.split(',').collect::<Vec<&str>>(),
    };

    for part in parts {
        if !lang.is_empty() && !file.is_empty() && no_style {
            break;
        }
        if let Some(pos) = part.find("=") {
            let (key, val) = (part[..pos].trim(), part[pos + 1..].trim());
            if lang.is_empty() && key == "lang" && !val.is_empty() {
                lang = val;
            }
            if file.is_empty() && key == "file" && !val.is_empty() {
                file = val;
            }
            if !no_style && key == "no_style" && !val.is_empty() {
                if val.to_lowercase() == "true" {
                    no_style = true
                }
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
