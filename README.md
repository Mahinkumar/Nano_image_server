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

Nano Image Server is a tiny, blazingly fast service to serve images with support for image operation on fly.

## Available Features
1. Low latency Image delivery
2. Image operation on fly via url queries
3. Caching and Instant Retrieval
4. Support for Linux and Windows

## Planned features 
1. Support for GPU Acceleration
2. Full server management interface
3. Advanced image browsing and management interface 

## Usage

1. Place Images you need in images folder next to the executable
2. Start the server
```bash
./nano_image_server #Linux
start nano_image_server.exe #Windows
```
3. Access the server from port 8000 in localhost.
4. To get image go to `/image/<imagename>.<format>`

<hr>

### Resizing
1. If needed resizing use queries resx and resy `/image/Nature.jpg?resx=1920&resy=1080`
2. When resizing use query resfilter `/image/Nature.jpg?resx=1920&resy=1080&resfilter=lanczos`
3. If specified size is 0 or left unspecified they display original size of the image
4. If resfilter query is unspecified, nearest is chosen by default
5. Choose from several resize algorithms for resizing using the resfilter query.<br>
    Availible resize algorithms are,
    - Nearest
    - Triangle
    - Catmullrom
    - Gaussian
    - Lanczos

### Filters
The availible filters are 
1. Blur
2. Grayscale 
3. Brighten
4. Contrast

### Transforms
The availible transforms are
1. Flip horizontal
2. Flip Vertical
3. Rotate (90deg, 180deg, 270deg)
4. Hue rotate

### Supported Image formats
The following formats are supported via encoders and decoders from the Image-rs Library

| Format   | Decoding                                  | Encoding                                |
| -------- | ----------------------------------------- | --------------------------------------- |
| AVIF     | Yes \*                                    | Yes (lossy only)                        |
| BMP      | Yes                                       | Yes                                     |
| Farbfeld | Yes                                       | Yes                                     |
| GIF      | Yes                                       | Yes                                     |
| HDR      | Yes                                       | Yes                                     |
| ICO      | Yes                                       | Yes                                     |
| JPEG     | Yes                                       | Yes                                     |
| EXR      | Yes                                       | Yes                                     |
| PNG      | Yes                                       | Yes                                     |
| PNM      | Yes                                       | Yes                                     |
| QOI      | Yes                                       | Yes                                     |
| TGA      | Yes                                       | Yes                                     |
| TIFF     | Yes                                       | Yes                                     |
| WebP     | Yes                                       | Yes (lossless only)                     |
<hr>


## Version Benchmarks 
```markdown
# command
ab -n 1000 -c 24 -k 'http://localhost:8000/image/in.jpg?resx=1080&resy=1920' #With Processing
ab -n 1000 -c 24 -k 'http://localhost:8000/image/in.jpg' #Without Processing
```
### Nano_image_server With ApacheBench on 24 Threads (Balanced Power Mode on a Laptop)
![image](https://github.com/user-attachments/assets/a15ca744-08d2-4d65-8f08-ab62556ab752)

