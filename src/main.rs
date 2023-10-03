// use sdwebuiapi::TextToImagePayload;
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let result = sdwebuiapi::add(1, 2);
    println!("1 + 2 = {}", result);
    let client = sdwebuiapi::Client::new("http://localhost:7860/");
    // println!("Client host: {}", client.origin);
    // let b64_img_str = client.txt2img("a cyberpunk cat").await;
    // let xyz = Default::default();
    // let xyz: TextToImagePayload = Default::default();
    let payload = sdwebuiapi::TextToImagePayload {
        prompt: "a cyberpunk cat".to_string(),
        ..Default::default()
    };
    let b64_img_str = client.txt2img(payload).await;
    // println!("b64_img_str = {}", b64_img_str);
    // output b64_img_str to a file
    // let img_bytes = base64::decode(b64_img_str).unwrap();
    // std::fs::write("test.png", img_bytes).unwrap();
    std::fs::write("test.txt", b64_img_str).unwrap();
}
