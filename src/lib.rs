use base64::{engine::general_purpose, Engine as _};
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use dotenv::dotenv;

fn get_api_key() -> String {
    dotenv().ok();

    env::var("ANTHROPIC_API_KEY").expect("ANTHROPIC_API_KEY must be set")
}

fn get_api_url() -> String {
    env::var("ANTHROPIC_API_URL").expect("ANTHROPIC_API_URL must be set")
}

fn get_api_version() -> String {
    env::var("ANTHROPIC_API_VERSION").expect("ANTHROPIC_API_VERSION must be set")
}

/// Sends a text request to the Anthropic API and returns the response.
///
/// This function takes the text content to be sent and the name of the model to use as parameters,
/// sends a POST request to the Anthropic API, and returns the API response as a string.
///
/// # Arguments
///
/// * `content` - The text content to send to the API.
/// * `model` - The name of the model to use.
///
/// # Returns
///
/// A `Result` containing the API response as a string, or an error if something went wrong.
///
/// # Errors
///
/// Returns an error if there was a problem sending the request or if there was a problem parsing the API response.
///
/// # Examples
///
/// ```
/// use anthropic_client::send_text_request;
///
/// let content = "Hello, world!";
/// let model = "claude-3-opus-20240229";
///
/// match send_text_request(content, model) {
///     Ok(response) => println!("API response: {}", response),
///     Err(error) => eprintln!("Error: {}", error),
/// }
/// ```
pub fn send_text_request(content: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let api_url = get_api_url();
    let api_version = get_api_version();
    let client = Client::new();

    let response = client
        .post(api_url)
        .header("x-api-key", api_key)
        .header("anthropic-version", api_version)
        .header("content-type", "application/json")
        .json(&json!({
            "model": model,
            "max_tokens": 1024,
            "messages": [
                {"role": "user", "content": content}
            ]
        }))
        .send()?;

    response.text().map_err(Into::into)
}

/// Sends an image request to the Anthropic API and returns the response.
///
/// This function takes the URL of the image to be sent, the media type of the image, and the name of the model to use as parameters.
/// It downloads the image data from the provided URL, encodes it as a base64 string, and sends a POST request to the Anthropic API
/// with the encoded image data. The API response is then returned as a string.
///
/// # Arguments
///
/// * `image_url` - The URL of the image to send to the API.
/// * `image_media_type` - The media type of the image (e.g., "image/jpeg", "image/png").
/// * `model` - The name of the model to use for processing the image.
///
/// # Returns
///
/// A `Result` containing the API response as a string, or an error if something went wrong.
///
/// # Errors
///
/// Returns an error if there was a problem downloading the image, encoding the image data, sending the request,
/// or if there was a problem parsing the API response.
///
/// # Examples
///
/// ```
/// use anthropic_client::send_image_request;
///
/// let image_url = "https://example.com/image.jpg";
/// let image_media_type = "image/jpeg";
/// let model = "claude-3-opus-20240229";
///
/// match send_image_request(image_url, image_media_type, model) {
///     Ok(response) => println!("API response: {}", response),
///     Err(error) => eprintln!("Error: {}", error),
/// }
/// ```
pub fn send_image_request(image_url: &str, image_media_type: &str, model: &str) -> Result<String, Box<dyn std::error::Error>> {
    let api_key = get_api_key();
    let api_url = get_api_url();
    let api_version = get_api_version();

    let client = Client::new();
    let response = client.get(image_url).send()?;
    let image_array_buffer = response.bytes()?;
    let image_data = general_purpose::STANDARD.encode(image_array_buffer);

    let client = Client::new();
    let response = client
        .post(api_url)
        .header("x-api-key", api_key)
        .header("anthropic-version", api_version)
        .header("content-type", "application/json")
        .json(&json!({
            "model": model,
            "max_tokens": 1024,
            "messages": [
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "image",
                            "source": {
                                "type": "base64",
                                "media_type": image_media_type,
                                "data": image_data,
                            },
                        }
                    ],
                }
            ]
        }))
        .send()?;

    response.text().map_err(Into::into)
}