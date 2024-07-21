use similar::{ChangeTag, TextDiff};

pub fn generate_html_diff(old_content: String, new_content: String, name: &str) -> String {
    let diff = TextDiff::from_lines(&old_content, &new_content);

    let mut html = String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>HTML Diff</title>
    <style>
        .delete { background-color: #ffcccc; text-decoration: line-through; }
        .insert { background-color: #ccffcc; }
    </style>
</head>
<body>
    <h1>Diff for: "#,
    );
    html.push_str(name);
    html.push_str("</h1><pre>");

    for change in diff.iter_all_changes() {
        let (class, text) = match change.tag() {
            ChangeTag::Delete => ("delete", change.as_str()),
            ChangeTag::Insert => ("insert", change.as_str()),
            ChangeTag::Equal => ("", change.as_str()),
        };

        if !class.is_empty() {
            html.push_str(&format!(
                "<span class=\"{}\">{}</span>",
                class,
                html_escape(text.unwrap())
            ));
        } else {
            html.push_str(&html_escape(text.unwrap()));
        }
    }

    html.push_str("</pre></body></html>");
    html
}

pub fn generate_html_error(name: &str, error_message: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Error: {}</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; padding: 20px; }}
        h1 {{ color: #d9534f; }}
        .error-message {{ background-color: #f2dede; border: 1px solid #ebccd1; color: #a94442; padding: 15px; border-radius: 4px; }}
    </style>
</head>
<body>
    <h1>Error occurred while comparing: {}</h1>
    <div class="error-message">
        <p><strong>Error details:</strong></p>
        <p>{}</p>
    </div>
</body>
</html>"#,
        name, name, error_message
    )
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
