use std::vec;

use serde::{Deserialize, Serialize};
use serde_json::json;

// use super::enums::Sampler;
// use super::enums::Upscaler;


#[derive(Debug, Serialize, Deserialize)]
pub struct TextToImagePayload {
    pub enable_hr: bool,
    pub hr_scale: f32,
    pub hr_upscaler: String, // Upscaler
    pub hr_second_pass_steps: i32,
    pub hr_resize_x: i32,
    pub hr_resize_y: i32,
    pub denoising_strength: f32,
    pub firstphase_width: i32,
    pub firstphase_height: i32,
    pub prompt: String,
    pub styles: Vec<String>,
    pub seed: i32,
    pub subseed: i32,
    pub subseed_strength: f32,
    pub seed_resize_from_h: i32,
    pub seed_resize_from_w: i32,
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
    pub s_churn: f32,
    pub s_tmax: f32,
    pub s_tmin: f32,
    pub s_noise: f32,
    pub sampler_name: String, // Sampler
    // pub sampler_index: Sampler,
    pub send_images: bool,
    pub save_images: bool,

    // pub script_name: String,
    // pub script_args: Vec<String>,
    pub alwayson_scripts: serde_json::Value,
    // pub override_settings: {},
    // pub override_settings_restore_afterwards: bool,
}

// add default value to TextToImagePayload's sampler and upscaler
impl Default for TextToImagePayload {
    fn default() -> Self {
        Self {
            enable_hr: false,
            hr_scale: 2.0,
            hr_upscaler: "".to_owned(),
            hr_second_pass_steps: 0,
            hr_resize_x: 0,
            hr_resize_y: 0,
            denoising_strength: 0.0,
            firstphase_width: 0,
            firstphase_height: 0,
            prompt: "".to_string(),
            styles: vec![],
            seed: -1,
            subseed: -1,
            subseed_strength: 0.0,
            seed_resize_from_h: -1,
            seed_resize_from_w: -1,
            batch_size: 1,
            n_iter: 1,
            steps: 20,
            cfg_scale: 7.0,
            width: 512,
            height: 512,
            restore_faces: false,
            tiling: false,
            do_not_save_samples: false,
            do_not_save_grid: false,
            negative_prompt: "".to_owned(),
            eta: 0.0,
            s_churn: 0.0,
            s_tmax: 0.0,
            s_tmin: 0.0,
            s_noise: 1.0,
            sampler_name: "Euler a".to_owned(),
            send_images: true,
            save_images: false,
            alwayson_scripts: json!({}),
        }
    }
}

impl TextToImagePayload {
    pub fn add_controlnet_units(&mut self, controlnet_units: &Vec<ControlnetPayload>) {
        let alwayson_scripts = json!({
            "ControlNet": {
                "args": serde_json::to_value(&controlnet_units).unwrap()
            }
        });
        let alwayson_scripts = alwayson_scripts
            .as_object()
            .unwrap()
            .to_owned();
        self.alwayson_scripts
            .as_object_mut()
            .unwrap()
            .extend(alwayson_scripts);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextToImageResponse {
    pub images: Vec<String>,
    // pub parameters: TextToImagePayload,
    pub parameters: serde_json::Value,
    pub info: String,
}

// impl<'de> Deserialize<'de> for TextToImageResponse {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>
//     {
//         let mut map = serde_json::Map::deserialize(deserializer)?;
//         let images = map.remove("images").ok_or_else(|| serde::de::Error::missing_field("images"))?;
//         let parameters = map.remove("parameters").ok_or_else(|| serde::de::Error::missing_field("parameters"))?;
//         let info = map.remove("info").ok_or_else(|| serde::de::Error::missing_field("info"))?;
//         Ok(TextToImageResponse {
//             images: serde_json::from_value(images).unwrap(),
//             parameters: serde_json::from_value(parameters).unwrap(),
//             info: serde_json::from_value(info).unwrap(),
//         })
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct  ControlnetPayload {
    pub input_image: String,
    pub mask: String,
    pub module: String,
    pub model: String,
    pub weight: f32,
    pub resize_mode: String,
    pub lowvram: bool,
    pub processor_res: i32,
    pub threshold_a: i32,
    pub threshold_b: i32,
    pub guidance: f32,
    pub guidance_start: f32,
    pub guidance_end: f32,
    pub control_mode: i32,
    pub pixel_perfect: bool,
}

impl Default for ControlnetPayload {
    fn default() -> Self {
        Self {
            input_image: "".to_owned(),
            mask: "".to_owned(),
            module: "".to_owned(),
            model: "".to_owned(),
            weight: 1.0,
            resize_mode: "Resize and Fill".to_owned(),
            lowvram: false,
            processor_res: 512,
            threshold_a: 64,
            threshold_b: 64,
            guidance: 1.0,
            guidance_start: 0.0,
            guidance_end: 1.0,
            control_mode: 0,
            pixel_perfect: false,
        }
    }
}
