use anthropic_client::{send_text_request,send_image_request};

#[test]
fn test_send_text_request() {
    let content = "Hello, Anthropic!";
    let model = "claude-3-opus-20240229";

    match send_text_request(content, model) {
        Ok(response) => {
            // Assert the response or perform other checks
            assert!(!response.is_empty());
        }
        Err(error) => {
            // Handle the error case
            panic!("Error: {}", error);
        }
    }
}

#[test]
fn test_send_image_request() {
    let image_url = "https://example.com/image.jpg";
    let image_media_type = "image/jpeg";
    let model = "claude-3-opus-20240229";

    match send_image_request(image_url, image_media_type, model) {
        Ok(response) => {
            // Assert the response or perform other checks
            assert!(!response.is_empty());
        }
        Err(error) => {
            // Handle the error case
            panic!("Error: {}", error);
        }
    }
}