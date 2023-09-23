use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let result = sdwebuiapi::add(1, 2);
    println!("1 + 2 = {}", result);
    let client = sdwebuiapi::Client::new("http://localhost:7860/");
    client.txt2img("test").await;
    println!("Client host: {}", client.origin);
}
