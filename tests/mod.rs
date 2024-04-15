use anthropic_client::{client::Anthropic};
use serde_json::json;

#[test]
fn test_send_text_request() {
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



#[test]
fn test_send_image_request() {
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
