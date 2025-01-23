> [!WARNING]  
> This Project is a work in progress and is not suitable to be used at this moment.  <br>
> The Processing algorithms are far from optimal and will undergo massive Improvements. <br>
> Star the repository for progress updates.

# Nano Image Server
![Rust-Linux Worklflow](https://github.com/mahinkumar/Nano_image_server/actions/workflows/Rust_Linux.yml/badge.svg)
![Rust-Windows Worklflow](https://github.com/mahinkumar/Nano_image_server/actions/workflows/Rust_Windows.yml/badge.svg)


<hr>

![image](https://github.com/user-attachments/assets/c43b43bf-b42e-4115-b225-da9a76f26894)
<hr>

|<a href="https://docs.mahinkumar.com/nanoimageserver/"> Docs </a>|<a href="https://docs.mahinkumar.com/nanoimageserver/"> Usage </a> | <a href="https://docs.mahinkumar.com/nanoimageserver/"> References </a> |

Nano Image Server is a tiny, blazingly fast service to serve images with support for image operation on fly.

It is truly asynchronous, Memory safe and thread safe. Built upon axum and tokio runtime. The image processing, encoding and decoding are done by the image crate. GPU support and caching are planned before the stable release.

## Available Features
1. Low latency Image delivery
2. Caching and Instant Retrieval
3. Support for Linux and Windows
4. Basic image operations on fly via url queries

## Usage

1. Place Images you need in images folder next to the executable
2. Start the server
```bash
./nano_image_server #Linux
start nano_image_server.exe #Windows
```
3. Access the server from port 8000 in localhost.
4. To get image go to `/<imagename>.<format>`

<hr>

## Availible Image operations

### Resizing Filters

| Algorithm | Description | Query |
|-----------|-------------|-------|
| Nearest | Simplest method, nearest neighbor interpolation | resfilter=nearest |
| Triangle | Linear interpolation, moderate quality scaling | resfilter=triangle |
| Catmull-Rom | Cubic interpolation, preserves image details | resfilter=catmullrom |
| Gaussian | Soft, smooth scaling, reduces sharpness | resfilter=gaussian |
| Lanczos | High-quality resampling, preserves details | resfilter=lanczos |
| Optimal | Automatically selects best algorithm | resfilter=optimal |

### Image Filters

| Filter | Description | Query      | f_param (argument) |
|--------|-------------|------------|--------------------|
| Blur | Reduces image sharpness, softens details | filter=blur | Yes |
| Grayscale | Converts image to black and white | filter=bw | No|
| Brighten | Increases overall image luminosity | filter=brighten | Yes |
| Contrast | Enhances difference between light and dark areas | filter=contrast | Yes |

### Image Transforms

| Transform | Description | Query | t_param (argument) |
|-----------|-------------|-------|--------------------|
| Flip Horizontal | Mirrors image vertically | transform=fliph | No |
| Flip Vertical | Mirrors image horizontally | transform=flipv | No |
| Rotate | Rotation at 90°, 180°, 270° angles | transform=rotate | Yes |
| Hue Rotate | Shifts color spectrum | transform=huerotate | Yes |

### Image Processing 

| Processing | Description | Query | p1 (argument) | p2 (argument) |
|------------|-------------|-------|---------------|---------------|
| Invert | Reverses image colors, creating a negative effect | process=invert | no | no |
| Unsharpen | Enhances image details by reducing blur | process=unsharpen | yes | yes |

---

## Supported Image formats
The following formats are supported via encoders and decoders from the Image-rs Library

| Format   | Decoding                                  | Encoding                                |
| -------- | ----------------------------------------- | --------------------------------------- |
| AVIF     | Yes\*                                    | Yes(lossy only)                        |
| BMP      | Yes                                      | Yes                                    |
| Farbfeld | Yes                                      | Yes                                    |
| GIF      | Yes                                      | Yes                                    |
| HDR      | Yes                                      | Yes                                    |
| ICO      | Yes                                      | Yes                                    |
| JPEG     | Yes                                      | Yes                                    |
| EXR      | Yes                                      | Yes                                    |
| PNG      | Yes                                      | Yes                                    |
| PNM      | Yes                                      | Yes                                    |
| QOI      | Yes                                      | Yes                                    |
| TGA      | Yes                                      | Yes                                    |
| TIFF     | Yes                                      | Yes                                    |
| WebP     | Yes                                      | Yes(lossless only)                     |
<hr>
