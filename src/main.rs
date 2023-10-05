use std::vec;

use tokio;
// use sdwebuiapi::TextToImagePayload;
// use data_encoding::BASE64;

// fn b64_img(raw_b64_str: &str) -> String {
//     "data:image/png;base64,".to_string() + raw_b64_str
// }

#[tokio::main]
async fn main() {
    // read image from tmp/input-image.png and convert it to base64
    let input_image = std::fs::read("tmp/input-image.png").unwrap();
    let raw_b64_str = data_encoding::BASE64.encode(&input_image);

    let controlnet_payload = sdwebuiapi::ControlnetPayload {
        input_image: raw_b64_str.clone(),
        // mask: raw_b64_str.clone(),
        // model: "control_v11p_sd15_mlsd [aca30ff0]".to_owned(),
        // module: "mlsd".to_owned(),
        model: "control_v11p_sd15_canny [d14c016b]".to_owned(),
        module: "canny".to_owned(),
        ..Default::default()
    };
    let controlnet_units = vec![controlnet_payload];
    let mut payload = sdwebuiapi::TextToImagePayload {
        prompt: "a cyberpunk cat".to_string(),
        // prompt: "a cyberpunk cat <lora:add_detail:1>".to_string(),
        // prompt: "a circle".to_string(),
        ..Default::default()
    };
    payload.add_controlnet_units(&controlnet_units);

    let client = sdwebuiapi::Client::new("http://localhost:7860/");
    let response = client.txt2img(payload).await;
    // println!("response.parameters = {:?}", response.parameters);
    // println!("response.info = {:?}", response.info);

    let raw_b64_str = &response.images[0];
    let output_image = data_encoding::BASE64.decode(raw_b64_str.as_bytes()).unwrap();
    std::fs::write("tmp/output-image.png", output_image).unwrap();

    // let b64_img_str = b64_img(raw_b64_str);
    // std::fs::write("tmp/output.txt", b64_img_str).unwrap();
}
