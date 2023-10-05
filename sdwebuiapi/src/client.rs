use reqwest;
// use serde_json::json;

use super::types::TextToImagePayload;
use super::types::TextToImageResponse;

pub struct OpenApiV1 {
    pub api_root: String,
}

impl OpenApiV1 {
    pub fn new(origin: &str) -> Self {
        // assign origin to _origin, if origin is not ends with "/", add it
        let api_root = if origin.ends_with("/") {
            format!("{}{}", origin, "sdapi/v1/")
        } else {
            format!("{}{}", origin, "/sdapi/v1/")
        };
        Self { api_root }
    }

    pub async fn post(
        &self,
        api_path: &str,
        request_payload: serde_json::Value,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url: String = format!("{}{}", self.api_root, api_path);
        let res = reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .json(&request_payload)
            .send()
            .await?;
        // println!("status = {}", res.status());
        let res_body_text = res.text().await.unwrap();
        // println!("body = {}", res_body_text);
        let res_body_json: serde_json::Value = serde_json::from_str(&res_body_text).unwrap();
        return Ok(res_body_json);
    }

    pub async fn get(&self, api_path: &str) -> Result<serde_json::Value, reqwest::Error> {
        let url: String = format!("{}{}", self.api_root, api_path);
        let res = reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        let res_body_text = res.text().await.unwrap();
        let res_body_json: serde_json::Value = serde_json::from_str(&res_body_text).unwrap();
        return Ok(res_body_json);
    }
}

pub struct Client {
    pub open_api_v1: OpenApiV1,
}

impl Client {
    pub fn new(origin: &str) -> Self {
        Self {
            open_api_v1: OpenApiV1::new(origin),
        }
    }

    pub async fn txt2img(&self, payload: TextToImagePayload) -> TextToImageResponse {
        let payload_value = serde_json::to_value(&payload).unwrap();
        // println!("request_payload = {}", payload_value.to_string());
        let json_data = self
            .open_api_v1
            .post("txt2img", payload_value)
            .await
            .unwrap();
        let response: TextToImageResponse = serde_json::from_value(json_data).unwrap();
        return response;
    }
}
