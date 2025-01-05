use std::process::{Command, Stdio};
use std::io::Write;

// Generate Markdown from HTML using `pandoc`
pub fn generate_html_from_md(markdown: String) -> String {
    let mut proc = Command::new("pandoc")
        .arg("-f")
        .arg("markdown")
        .arg("-t")
        .arg("html")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Error converting to HTML!");

    if let Some(mut stdin) = proc.stdin.take() {
        stdin.write_all(markdown.as_bytes()).unwrap();
    }

    let output = proc.wait_with_output().unwrap();
    let html = String::from_utf8_lossy(&output.stdout).to_string();

    return html;
}
