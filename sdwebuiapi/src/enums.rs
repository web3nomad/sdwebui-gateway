// whole enum for stable diffusion webui sampler
use serde::{Deserialize, Serialize, Serializer};

#[derive(Deserialize)]
pub enum Sampler {
    EulerA,
    Euler,
}

impl Serialize for Sampler {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Sampler::EulerA => serializer.serialize_str("Euler a"),
            Sampler::Euler => serializer.serialize_str("Euler"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum Upscaler {
    none,
    Lanczos,
    Nearest,
    LDSR,
    BSRGAN,
    ESRGAN_4x,
    R_ESRGAN_General_4xV3,
    ScuNET_GAN,
    ScuNET_PSNR,
    SwinIR_4x,
}
