use pulldown_cmark_plus::{get_title, to_html};

#[test]
fn test_to_html() {
    let markdown = "```rust
                    fn main() {
                        println!(\"Hello, world!\");
                    }
                    ```";
    let html = to_html(markdown);
    assert!(!html.contains("<div class=\"codeblock-file\">"));
    assert!(html.contains("<pre><code class=\"rust\" data-lang=\"language-rust\">"));
    assert!(html.contains("fn main() {"));
    assert!(html.contains("println!(&quot;Hello, world!&quot;);"));
    assert!(html.contains("</code></pre>"));

    let markdown = "```shell:main.sh
                    ls -la
                    ```";
    let html = to_html(markdown);
    assert!(html.contains("<div class=\"codeblock-file\"><span>main.sh</span></div>"));
    assert!(html.contains("<pre><code class=\"shell\" data-lang=\"language-shell\">"));
    assert!(html.contains("ls -la"));
    assert!(html.contains("</code></pre>"));

    let markdown = "```js,file=main.js
                    console.log(\"Hello,World!\");
                    ```";
    let html = to_html(markdown);
    assert!(html.contains("<div class=\"codeblock-file\"><span>main.js</span></div>"));
    assert!(html.contains("<pre><code class=\"js\" data-lang=\"language-js\">"));
    assert!(html.contains("console.log(&quot;Hello,World!&quot;);"));
    assert!(html.contains("</code></pre>"));

    let markdown = "";
    let html = to_html(markdown);
    assert!(html.is_empty());

    let markdown = "# Hello,World!";
    let html = to_html(markdown);
    assert!(html.contains("<h1>Hello,World!</h1>"));
}

#[test]
fn test_get_title() {
    let markdown = "# Rust Code Example\n\nSome text here.";
    let title = get_title(markdown);
    assert_eq!(title, "Rust Code Example");

    let markdown_no_title = "Some text here.";
    let title_no_title = get_title(markdown_no_title);
    assert!(title_no_title.is_empty());
}
