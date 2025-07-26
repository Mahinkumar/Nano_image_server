> [!WARNING]
> Not production-ready |
> ⭐ Star for updates

> [!NOTE]
> Nano Image Server requires HTTPS (v0.6.0-beta+) <br>
> TLS certificate is required. You can generate a sample using keygen.sh script included (linux)


# Nano Image Server
![Version](https://img.shields.io/badge/version-0.6.0--beta-orange?style=flat-square)
![Rust-Linux Worklflow](https://github.com/mahinkumar/Nano_image_server/actions/workflows/Rust_Linux.yml/badge.svg)

<hr>

![image](https://github.com/user-attachments/assets/c43b43bf-b42e-4115-b225-da9a76f26894)
<hr>

Nano Image Server is a tiny, blazingly fast service to serve images with support for image operation on fly.<br>
It is truly asynchronous, Memory safe and thread safe. Built upon axum and tokio runtime.


## Available Features
1. Low latency Image delivery
2. Caching and Instant Retrieval
3. Basic image operations on fly via url queries


## Usage

1. Place Images you need in images folder next to the executable
2. Start the server
```bash
./nano_image_server #Linux
```
3. Access the server from port 8000 in localhost.
4. To get image go to `/<imagename>.<format>`

<hr>

## Image operations

> [!WARNING]  
> A plugin based system for image operations is being developed. The provided API can change drastically until stable release.

### Availible image operations
| Operation | Query | Examples |
|-----------|--------|----------|
| Resize | resfilter=nearest/triangle/lanczos | resfilter=nearest |
| Filter | filter=blur/bw/brighten/contrast | filter=blur&f_param=1.0 |
| Transform | transform=fliph/flipv/rotate | transform=rotate&t_param=90 |
| Convert | to=format | to=webp |

### Supported Formats
| Format | Support Level |
|--------|---------------|
| AVIF | Decode: Yes*, Encode: Lossy |
| BMP, GIF, ICO, JPEG, PNG | Full Support |
| WebP | Decode: Yes, Encode: Lossless |
| TIFF, TGA, PNM, QOI, HDR, EXR | Full Support |
