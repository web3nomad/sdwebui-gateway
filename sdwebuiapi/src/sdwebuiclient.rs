use serde_json::json;
use reqwest;

use super::enums::Sampler;
use super::enums::Upscaler;

pub struct TextToImagePayload {
    pub prompt: String,
    pub sampler: Sampler,
    pub upscaler: Upscaler,
}

// add default value to TextToImagePayload's sampler and upscaler
impl Default for TextToImagePayload {
    fn default() -> Self {
        Self {
            prompt: "".to_string(),
            sampler: Sampler::EulerA,
            upscaler: Upscaler::none,
        }
    }
}

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

    pub async fn textToImage(&self, payload: TextToImagePayload) -> String {
        "".to_string()
    }

    pub async fn txt2img(&self, prompt: &str) -> String {
        let payload = json!({
            "enable_hr": false,
            "denoising_strength": 0,
            "firstphase_width": 0,
            "firstphase_height": 0,
            "hr_scale": 2,
            "hr_upscaler": "",
            "hr_second_pass_steps": 0,
            "hr_resize_x": 0,
            "hr_resize_y": 0,
            "hr_sampler_name": "",
            "hr_prompt": "",
            "hr_negative_prompt": "",
            "prompt": prompt.to_string(),
            "styles": [],
            "seed": -1,
            "subseed": -1,
            "subseed_strength": 0,
            "seed_resize_from_h": -1,
            "seed_resize_from_w": -1,
            "sampler_name": "Euler a",
            "batch_size": 1,
            "n_iter": 1,
            "steps": 50,
            "cfg_scale": 7,
            "width": 512,
            "height": 512,
            "restore_faces": false,
            "tiling": false,
            "do_not_save_samples": false,
            "do_not_save_grid": false,
            "negative_prompt": "",
            "eta": 0,
            "s_min_uncond": 0,
            "s_churn": 0,
            "s_tmax": 0,
            "s_tmin": 0,
            "s_noise": 1,
            "override_settings": {},
            "override_settings_restore_afterwards": true,
            "script_args": [],
            "sampler_index": "Euler",
            "script_name": "",
            "send_images": true,
            "save_images": false,
            "alwayson_scripts": {}
        });
        let res = reqwest::Client::new()
            .post("http://localhost:7860/sdapi/v1/txt2img")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .unwrap();
        println!("status = {}", res.status());
        let body = res.text().await.unwrap();
        let json_data: serde_json::Value = serde_json::from_str(&body).unwrap();
        let raw_b64_str = json_data["images"][0].as_str().unwrap();
        let b64_img_str = b64_img(raw_b64_str);
        return b64_img_str;
    }

}
