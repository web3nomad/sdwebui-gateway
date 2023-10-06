use dotenv::dotenv;
use std::env;
use std::vec;

use tokio;

fn b64_img(raw_b64_str: &str) -> String {
    "data:image/png;base64,".to_string() + raw_b64_str
}

async fn generate_image_1(webui_origin: &str) {
    println!("generate_image_1 starts");

    let input_image = std::fs::read("test-client/tmp/input-image.png").unwrap();
    let raw_b64_str = data_encoding::BASE64.encode(&input_image);

    let controlnet_payload = sdwebuiapi::ControlnetPayload {
        input_image: raw_b64_str.clone(),
        // model: "control_v11p_sd15_mlsd [aca30ff0]".to_owned(),
        // module: "mlsd".to_owned(),
        model: "control_v11p_sd15_canny [d14c016b]".to_owned(),
        module: "canny".to_owned(),
        ..Default::default()
    };
    let controlnet_units = vec![controlnet_payload];

    let lora_paylod = sdwebuiapi::LoraPayload {
        name: "add_detail".to_owned(),
        weight: 1.0
    };
    let loras = vec![lora_paylod];

    let mut payload = sdwebuiapi::TextToImagePayload {
        prompt: "a cyberpunk cat".to_owned(),
        // prompt: "a cyberpunk cat <lora:add_detail:1>".to_string(),
        // prompt: "a circle".to_string(),
        ..Default::default()
    };

    payload
        .set_base_model("level4_v50BakedVAEFp16.safetensors [c61df6130b]")
        .add_loras(&loras)
        .add_controlnet_units(&controlnet_units);
    // println!("prompt = {:?}", payload.prompt);
    // println!("payload = {:?}",  payload);

    let client = sdwebuiapi::Client::new(webui_origin);
    let response = client.txt2img(payload).await;
    // println!("response.parameters = {:?}", response.parameters);
    // println!("response.info = {:?}", response.info);

    let raw_b64_str = &response.images[0];
    let output_image = data_encoding::BASE64.decode(raw_b64_str.as_bytes()).unwrap();
    std::fs::write("test-client/tmp/output-image-1.png", output_image).unwrap();
    std::fs::write("test-client/tmp/output-image-1.txt", b64_img(raw_b64_str)).unwrap();

    println!("generate_image_1 ends");
}

async fn generate_image_2(webui_origin: &str) {
    println!("generate_image_2 starts");
    let mut payload = sdwebuiapi::TextToImagePayload {
        prompt: "a circle".to_string(),
        ..Default::default()
    };
    payload.set_base_model("DreamShaper_6_BakedVae.safetensors [b76cc78ad9]");
    let client = sdwebuiapi::Client::new(webui_origin);
    let response = client.txt2img(payload).await;
    let raw_b64_str = &response.images[0];
    let output_image = data_encoding::BASE64.decode(raw_b64_str.as_bytes()).unwrap();
    std::fs::write("test-client/tmp/output-image-2.png", output_image).unwrap();
    std::fs::write("test-client/tmp/output-image-2.txt", b64_img(raw_b64_str)).unwrap();
    println!("generate_image_2 ends");
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let webui_origin = env::var("SD_WEBUI_TEST_ORIGIN").unwrap();
    let response = tokio::join!(
        generate_image_1(&webui_origin),
        generate_image_2(&webui_origin)
    );
    println!("response = {:?}", response);
}
