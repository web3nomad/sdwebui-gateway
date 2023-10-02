// whole enum for stable diffusion webui sampler
pub enum Sampler {
    EulerA,
    Euler
}

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
