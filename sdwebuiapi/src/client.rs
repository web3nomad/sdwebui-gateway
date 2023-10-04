use serde_json::json;
use reqwest;

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
        Self {
            api_root,
        }
    }

    pub async fn call(&self, api_path: &str, request_payload: serde_json::Value)
        -> Result<serde_json::Value, reqwest::Error>
    {
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
        let mut request_payload = json!({
            //
        });
        // merge payload1 to payload2
        request_payload.as_object_mut().unwrap().extend(payload_value.as_object().unwrap().clone());
        let json_data = self.open_api_v1.call("txt2img", request_payload).await.unwrap();
        let response: TextToImageResponse = serde_json::from_value(json_data).unwrap();
        return response;
    }

}
