// use sdwebuiapi::TextToImagePayload;
use tokio;

fn b64_img(raw_b64_str: &str) -> String {
    "data:image/png;base64,".to_string() + raw_b64_str
}

#[tokio::main]
async fn main() {
    let client = sdwebuiapi::Client::new("http://localhost:7860/");
    let payload = sdwebuiapi::TextToImagePayload {
        prompt: "a cyberpunk cat".to_string(),
        ..Default::default()
    };

    let response = client.txt2img(payload).await;
    println!("{:?}", response.parameters);
    println!("{:?}", response.info);

    let raw_b64_str = &response.images[0];
    let b64_img_str = b64_img(raw_b64_str);

    std::fs::write("test.txt", b64_img_str).unwrap();
}
