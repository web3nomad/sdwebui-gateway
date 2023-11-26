use reqwest;
// use serde_json::json;
use super::types::*;

#[derive(Clone)]
pub struct OpenApiV1Auth {
    pub username: String,
    pub password: String,
}

pub struct OpenApiV1 {
    pub api_root: String,
    pub api_auth: Option<OpenApiV1Auth>,
}

impl OpenApiV1 {
    pub fn new(origin: &str, api_auth: Option<OpenApiV1Auth>) -> Self {
        // assign origin to _origin, if origin is not ends with "/", add it
        let api_root = if origin.ends_with("/") {
            format!("{}{}", origin, "sdapi/v1/")
        } else {
            format!("{}{}", origin, "/sdapi/v1/")
        };
        Self { api_root, api_auth }
    }

    pub async fn post(
        &self,
        api_path: &str,
        request_payload: serde_json::Value,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url: String = format!("{}{}", self.api_root, api_path);
        let req = reqwest::Client::new()
            .post(url)
            .json(&request_payload)
            .header("Content-Type", "application/json");
        let req = if let Some(api_auth) = &self.api_auth {
            req.basic_auth(&api_auth.username, Some(&api_auth.password))
        } else {
            req
        };
        let res = req.send().await?;
        let res_status = res.status();
        let res_body_text = res.text().await.unwrap();
        if res_status != 200 {
            println!("res.status(): {}", res_status);
            println!("res.text(): {}", res_body_text);
            panic!("res.status() != 200");
        }
        let res_body_json: serde_json::Value = serde_json::from_str(&res_body_text).unwrap();
        return Ok(res_body_json);
    }

    pub async fn get(&self, api_path: &str) -> Result<serde_json::Value, reqwest::Error> {
        let url: String = format!("{}{}", self.api_root, api_path);
        let req = reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json");
        let req = if let Some(api_auth) = &self.api_auth {
            req.basic_auth(&api_auth.username, Some(&api_auth.password))
        } else {
            req
        };
        let res = req.send().await?;
        let res_status = res.status();
        let res_body_text = res.text().await.unwrap();
        if res_status != 200 {
            println!("res.status(): {}", res_status);
            println!("res.text(): {}", res_body_text);
            panic!("res.status() != 200");
        }
        let res_body_json: serde_json::Value = serde_json::from_str(&res_body_text).unwrap();
        return Ok(res_body_json);
    }
}

pub struct Client {
    sd_models: Vec<SDModel>,
    pub open_api_v1: OpenApiV1,
}

impl Client {
    pub fn new(origin: &str, api_auth: Option<OpenApiV1Auth>) -> Self {
        Self {
            sd_models: vec![],
            open_api_v1: OpenApiV1::new(origin, api_auth),
        }
    }

    pub async fn txt2img(&self, payload: TextToImagePayload) -> TextToImageResponse {
        let payload_value = serde_json::to_value(&payload).unwrap();
        let json_data = self
            .open_api_v1
            .post("txt2img", payload_value)
            .await
            .unwrap();
        let response: TextToImageResponse = serde_json::from_value(json_data).unwrap();
        return response;
    }

    pub async fn set_model_by_title(&mut self, title: &str) {
        let sd_models = self.get_sd_models().await;
        if let Some(sd_model) = sd_models.iter().find(|sd_model| sd_model.title == title) {
            let webui_config = WebUIConfig {
                // sd_model_checkpoint: format!("{} [{}]", sd_model.model_name, sd_model.hash),
                // sd_checkpoint_hash: sd_model.hash.clone(),
                sd_model_checkpoint: sd_model.title.clone(),
                sd_checkpoint_hash: sd_model.sha256.clone(),
            };
            self.set_webui_config(webui_config).await;
        } else {
            panic!("No model found with title: {}", title);
        }
    }

    pub async fn get_current_model(&mut self) -> &SDModel {
        let webui_config = self.get_webui_config().await;
        let sd_models = self.get_sd_models().await;
        let found = sd_models
            .iter()
            // .find(|sd_model| sd_model.hash == webui_config.sd_checkpoint_hash);
            .find(|sd_model| sd_model.title == webui_config.sd_model_checkpoint);
        if let Some(sd_model) = found {
            sd_model
        } else {
            // &sd_models.to_owned()[0]
            // sd_models.get(0).unwrap()
            panic!("No current model found!");
        }
    }

    pub async fn get_sd_models(&mut self) -> &Vec<SDModel> {
        if self.sd_models.len() == 0 {
            let json_data = self.open_api_v1.get("sd-models").await.unwrap();
            let sd_models: Vec<SDModel> =
                serde_json::from_value::<Vec<SDModel>>(json_data).unwrap();
            self.sd_models = sd_models;
        }
        &self.sd_models
    }

    async fn get_webui_config(&self) -> WebUIConfig {
        let json_data = self.open_api_v1.get("options").await.unwrap();
        serde_json::from_value(json_data).unwrap()
    }

    async fn set_webui_config(&self, webui_config: WebUIConfig) {
        let payload = serde_json::to_value(webui_config).unwrap();
        self.open_api_v1.post("options", payload).await.unwrap();
    }
}
