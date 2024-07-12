# Markdown to HTML Converter

A simple Rust program to convert Markdown documents to HTML format, featuring basic CSS styling for better appearance.

## Features

- **Interactive Input**: Prompts the user for the Markdown file location.
- **Markdown Parsing**: Utilizes the `pulldown-cmark` library to parse Markdown syntax.
- **HTML Generation**: Converts Markdown content to HTML format.
- **Basic Styling**: Adds CSS styling to the HTML output for improved readability.

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system.

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/your-username/markdown-to-html.git
    cd markdown-to-html
    ```

2. Add the `pulldown-cmark` dependency to your `Cargo.toml`:
    ```toml
    [dependencies]
    pulldown-cmark = "0.9"
    ```

### Usage

1. Create a Markdown file (`example.md`) in the project directory:
    ```markdown
    # Hello, World!

    This is a simple Markdown to HTML converter in Rust.

    - Item 1
    - Item 2
    - Item 3

    **Bold Text**
    ```

2. Run the project:
    ```sh
    cargo run
    ```

3. When prompted, enter the path to your Markdown file:
    ```sh
    Enter the path to the Markdown file: example.md
    ```

4. The HTML output will be saved to `output.html`.

### Example

Given the following Markdown content in `example.md`:

    ```markdown
    # Hello, World!

    This is a simple Markdown to HTML converter in Rust.

    - Item 1
    - Item 2
    - Item 3

    **Bold Text**
    ```

The program will generate an `output.html` file with the following content:

    ```html
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <title>Markdown to HTML</title>
        <style>
            body {
                font-family: Arial, sans-serif;
                line-height: 1.6;
                margin: 40px;
                background-color: #f9f9f9;
            }
            h1, h2, h3, h4, h5, h6 {
                color: #333;
            }
            pre {
                background-color: #eee;
                padding: 10px;
                border-radius: 5px;
            }
            code {
                background-color: #f4f4f4;
                padding: 2px 4px;
                border-radius: 3px;
            }
            blockquote {
                border-left: 10px solid #ccc;
                margin: 1.5em 10px;
                padding: 0.5em 10px;
                color: #666;
                background-color: #f9f9f9;
            }
        </style>
    </head>
    <body>
    <h1>Hello, World!</h1>
    <p>This is a simple Markdown to HTML converter in Rust.</p>
    <ul>
    <li>Item 1</li>
    <li>Item 2</li>
    <li>Item 3</li>
    </ul>
    <p><strong>Bold Text</strong></p>
    </body>
    </html>
    ```

### Contributing

Contributions are welcome! Please feel free to submit a pull request.

### License

This project is licensed under the MIT License. See the [LICENSE](https://raw.githubusercontent.com/samarpreetxd/Mark2HTML/main/LICENSE) file for details.
