# sdwebuiapi

A Rust library for interacting with openapi of AUTOMATIC1111/stable-diffusion-webui.

Supports txt2img API calls, with controlnet 2.0 support and lora support.

The parameters for API calls are based on the documentation at http://localhost:7860/docs and have been successfully tested on webui 1.5.

# Install

```shell
cargo add sdwebuiapi
```

# Usage

webuiapi_demo.ipynb contains example code with original images. Images are compressed as jpeg in this document.

## create API client

```rust
let api_auth = Some(sdwebuiapi::OpenApiV1Auth {
    username: "username set in webui's --api-auth arg",
    password: "password set in webui's --api-auth arg",
});
// or `let api_auth = None` if webui is not protected by api_auth
let client = sdwebuiapi::Client::new("http://localhost:7860", api_auth);
```

## txt2img

```rust
let mut payload = sdwebuiapi::TextToImagePayload {
    prompt: "a cyberpunk cat".to_owned(),
    ..Default::default()
};

payload.set_base_model("v1-5-pruned-emaonly.safetensors [6ce0161689]");

let response: sdwebuiapi::TextToImageResponse = client.txt2img(payload).await;

// the generated image is in base64 format
let raw_b64_str: &str = &response.images[0];
```

## txt2img with controlnet and lora

encode image to base64, controlnet accepts raw base64 png image string, without headers

```rust
let input_image = std::fs::read("input-image.png").unwrap();
let raw_b64_str = data_encoding::BASE64.encode(&input_image);
```

build txt2img params

```rust
let controlnet_payload = sdwebuiapi::ControlnetPayload {
    input_image: raw_b64_str.clone(),
    model: "control_v11p_sd15_canny [d14c016b]".to_owned(),
    module: "canny".to_owned(),
    ..Default::default()
};

let lora_paylod = sdwebuiapi::LoraPayload {
    name: "add_detail".to_owned(),
    weight: 1.0
};

let mut payload = sdwebuiapi::TextToImagePayload {
    prompt: "a cyberpunk cat".to_owned(),
    ..Default::default()
};

payload
    .set_base_model("v1-5-pruned-emaonly.safetensors [6ce0161689]")
    .add_loras(&vec![lora_paylod])
    .add_controlnet_units(&vec![controlnet_payload]);
```

generate image

```rust
let response = client.txt2img(payload).await;
```
