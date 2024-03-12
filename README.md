```markdown
# Anthropic API Client

A Rust library for interacting with the Anthropic API.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
anthropic_client = "0.1.4"
```

## Usage

Here's a simple example of how to use the `anthropic_client` crate to send a request to the Anthropic API:

```rust
use anthropic_client::send_test_request;

fn main() {
    let content = "Hello, Anthropic!";
    let model = "claude-3-opus-20240229";

    match send_text_request(content, model) {
        Ok(response) => println!("Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

Image example
```rust
use anthropic_client::send_image_request;

fn main() {
    let image_url = "https://example.com/image.jpg";
    let image_media_type = "image/jpeg";
    let model = "claude-3-opus-20240229";

    match send_image_request(image_url, image_media_type, model) {
        Ok(response) => println!("Response: {}", response),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

Make sure to set the `ANTHROPIC_API_KEY` environment variable with your Anthropic API key before running the code.
Make sure to set the `ANTHROPIC_API_URL` environment variable with your Anthropic API key before running the code.
Make sure to set the `ANTHROPIC_API_VERSION` environment variable with your Anthropic API key before running the code.
```yaml
ANTHROPIC_API_URL=https://api.anthropic.com/v1/messages
ANTHROPIC_API_VERSION=2023-06-01
ANTHROPIC_API_KEY=xxxxxxxx
```


## API

### `send_request(content: &str) -> Result<String, Box<dyn std::error::Error>>`

Sends a request to the Anthropic API with the provided content and returns the response as a `String`.

- `content`: The content to send to the Anthropic API.
- Returns: A `Result` containing the response string on success, or an error on failure.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.