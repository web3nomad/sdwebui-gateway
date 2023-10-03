// use sdwebuiapi::TextToImagePayload;
use tokio;

#[tokio::main]
async fn main() {
    let client = sdwebuiapi::Client::new("http://localhost:7860/");
    let payload = sdwebuiapi::TextToImagePayload {
        prompt: "a cyberpunk cat".to_string(),
        ..Default::default()
    };
    let b64_img_str = client.txt2img(payload).await;
    std::fs::write("test.txt", b64_img_str).unwrap();
}
