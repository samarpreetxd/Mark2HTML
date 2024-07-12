use pulldown_cmark::{html, Options, Parser};
use std::fs;
use std::io::{self, Write};

fn main() {
    // Ask the user for the Markdown file location
    let mut input = String::new();
    print!("Enter the path to the Markdown file: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim();

    // Read the Markdown file content
    let markdown_input = fs::read_to_string(input)
        .expect("Failed to read Markdown file");

    // Set options for the Markdown parser
    let options = Options::empty();
    let parser = Parser::new_ext(&markdown_input, options);

    // Create an HTML output buffer
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Add basic CSS styling
    let styled_html_output = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Markdown to HTML</title>
    <style>
        body {{
            font-family: Arial, sans-serif;
            line-height: 1.6;
            margin: 40px;
            background-color: #f9f9f9;
        }}
        h1, h2, h3, h4, h5, h6 {{
            color: #333;
        }}
        pre {{
            background-color: #eee;
            padding: 10px;
            border-radius: 5px;
        }}
        code {{
            background-color: #f4f4f4;
            padding: 2px 4px;
            border-radius: 3px;
        }}
        blockquote {{
            border-left: 10px solid #ccc;
            margin: 1.5em 10px;
            padding: 0.5em 10px;
            color: #666;
            background-color: #f9f9f9;
        }}
    </style>
</head>
<body>
{}
</body>
</html>"#,
        html_output
    );

    // Write the HTML output to a file
    fs::write("output.html", styled_html_output)
        .expect("Failed to write HTML output");

    println!("Markdown has been converted to HTML and saved to output.html");
}
