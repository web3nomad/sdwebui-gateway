use std::vec;

use serde::{Deserialize, Serialize, Deserializer};

// use super::enums::Sampler;
// use super::enums::Upscaler;


#[derive(Debug, Serialize, Deserialize)]
pub struct TextToImagePayload {
    pub enable_hr: bool,
    pub denoising_strength: f32,
    pub firstphase_width: i32,
    pub firstphase_height: i32,
    pub hr_scale: f32,
    // pub hr_upscaler: Upscaler,
    pub hr_upscaler: String,
    pub hr_second_pass_steps: i32,
    pub hr_resize_x: i32,
    pub hr_resize_y: i32,
    pub hr_sampler_name: String,
    pub hr_prompt: String,
    pub hr_negative_prompt: String,
    pub prompt: String,
    pub styles: Vec<String>,
    pub seed: i32,
    pub subseed: i32,
    pub subseed_strength: f32,
    pub seed_resize_from_h: i32,
    pub seed_resize_from_w: i32,
    // pub sampler_name: Sampler,
    pub sampler_name: String,
    pub batch_size: i32,
    pub n_iter: i32,
    pub steps: i32,
    pub cfg_scale: f32,
    pub width: i32,
    pub height: i32,
    pub restore_faces: bool,
    pub tiling: bool,
    pub do_not_save_samples: bool,
    pub do_not_save_grid: bool,
    pub negative_prompt: String,
    pub eta: f32,
    pub s_min_uncond: f32,
    pub s_churn: f32,
    pub s_tmax: f32,
    pub s_tmin: f32,
    pub s_noise: f32,
    // pub override_settings: unknown,
    pub override_settings_restore_afterwards: bool,
    pub script_args: Vec<String>,
    // pub sampler_index: Sampler,
    pub script_name: String,
    pub send_images: bool,
    pub save_images: bool,
    // pub alwayson_scripts: i32,
}

// add default value to TextToImagePayload's sampler and upscaler
impl Default for TextToImagePayload {
    fn default() -> Self {
        Self {
            enable_hr: false,
            denoising_strength: 0.0,
            firstphase_width: 0,
            firstphase_height: 0,
            hr_scale: 2.0,
            // hr_upscaler: Upscaler::None,
            hr_upscaler: "".to_owned(),
            hr_second_pass_steps: 0,
            hr_resize_x: 0,
            hr_resize_y: 0,
            hr_sampler_name: "".to_owned(),
            hr_prompt: "".to_owned(),
            hr_negative_prompt: "".to_owned(),
            prompt: "".to_string(),
            styles: vec![],
            seed: -1,
            subseed: -1,
            subseed_strength: 0.0,
            seed_resize_from_h: -1,
            seed_resize_from_w: -1,
            // sampler_name: Sampler::EulerA,
            sampler_name: "Euler a".to_owned(),
            batch_size: 1,
            n_iter: 1,
            steps: 50,
            cfg_scale: 7.0,
            width: 512,
            height: 512,
            restore_faces: false,
            tiling: false,
            do_not_save_samples: false,
            do_not_save_grid: false,
            negative_prompt: "".to_owned(),
            eta: 0.0,
            s_min_uncond: 0.0,
            s_churn: 0.0,
            s_tmax: 0.0,
            s_tmin: 0.0,
            s_noise: 1.0,
            // override_settings: {},
            override_settings_restore_afterwards: true,
            script_args: vec![],
            // sampler_index: Sampler::EulerA,
            script_name: "".to_owned(),
            send_images: true,
            save_images: false,
            // alwayson_scripts: {}
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TextToImageResponse {
    pub images: Vec<String>,
    pub parameters: TextToImagePayload,
    pub info: String,
}

impl<'de> Deserialize<'de> for TextToImageResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let mut map = serde_json::Map::deserialize(deserializer)?;
        let images = map.remove("images").ok_or_else(|| serde::de::Error::missing_field("images"))?;
        let parameters = map.remove("parameters").ok_or_else(|| serde::de::Error::missing_field("parameters"))?;
        let info = map.remove("info").ok_or_else(|| serde::de::Error::missing_field("info"))?;
        Ok(TextToImageResponse {
            images: serde_json::from_value(images).unwrap(),
            parameters: serde_json::from_value(parameters).unwrap(),
            info: serde_json::from_value(info).unwrap(),
        })
    }
}
