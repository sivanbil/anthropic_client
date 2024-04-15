```markdown
# Anthropic API Client

A Rust library for interacting with the Anthropic API.

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
anthropic_client = "1.0.0"
```

## Usage

Here's a simple example of how to use the `anthropic_client` crate to send a request to the Anthropic API:

```rust
use anthropic_client::send_test_request;

fn main() {
    let content = "Hello, Anthropic! My name is sivanliao.";
    let model = "claude-3-opus-20240229";
    let anthropic_client = Anthropic::new();
    let messages = vec![
        json!({"role": "user", "content": content}),
    ];
    match anthropic_client.messages.create(&anthropic_client, model, 1024, &messages) {
        Ok(response) => {
            println!("{:?}", response);
            // Assert the response or perform other checks
            assert!(!response.is_empty());
        }
        Err(error) => {
            // Handle the error case
            panic!("Error: {}", error);
        }
    }
}
```

Image example
```rust
use anthropic_client::send_image_request;

fn main() {
    let image_url = "https://imagepphcloud.thepaper.cn/pph/image/300/508/637.jpg";
    let image_media_type = "image/jpeg";
    let model = "claude-3-opus-20240229";
    let anthropic_client = Anthropic::new();
    let messages = Anthropic::pack_vision_content(image_url.parse().unwrap(), image_media_type);

    match messages {
        Ok(pack_messages) => {
            match anthropic_client.messages.create(&anthropic_client, model, 1024, &pack_messages) {
                Ok(response) => {
                    println!("{:?}", response);
                    // Assert the response or perform other checks
                    assert!(!response.is_empty());
                }
                Err(error) => {
                    // Handle the error case
                    panic!("Error: {}", error);
                }
            }
        }
        Err(error) => {
            panic!("Error: {}", error);
        }
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

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.