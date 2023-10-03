use serde_json::json;
use reqwest;

use super::types::TextToImagePayload;

fn b64_img(raw_b64_str: &str) -> String {
    "data:image/png;base64,".to_string() + raw_b64_str
}

pub struct Client {
    pub origin: String,
}

impl Client {
    pub fn new(origin: &str) -> Self {
        Self {
            origin: origin.to_string(),
        }
    }

    pub async fn txt2img(&self, payload: TextToImagePayload) -> String {
        // let payload_json = serde_json::to_string(&payload).unwrap();
        let payload_value = serde_json::to_value(&payload).unwrap();
        let mut request_payload = json!({
            //
        });
        request_payload.as_object_mut().unwrap().extend(payload_value.as_object().unwrap().clone());
        // merge payload1 to payload2
        let res = reqwest::Client::new()
            .post("http://localhost:7860/sdapi/v1/txt2img")
            .header("Content-Type", "application/json")
            .json(&request_payload)
            .send()
            .await
            .unwrap();
        println!("status = {}", res.status());
        let body = res.text().await.unwrap();
        println!("body = {}", body);
        let json_data: serde_json::Value = serde_json::from_str(&body).unwrap();
        let raw_b64_str = json_data["images"][0].as_str().unwrap();
        let b64_img_str = b64_img(raw_b64_str);
        return b64_img_str;
    }

}
