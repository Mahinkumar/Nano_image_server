> [!WARNING]
> Not production-ready | v0.7.0-beta with experimental S3-FIFO cache
> ⭐ Star for updates

> [!NOTE]
> Nano Image Server requires HTTPS (v0.6.0+) <br>
> TLS certificate required - use included `keygen.sh` script (Linux) to generate test certs <br>
> Image processing and caching are optional features configured at compile time

# Nano Image Server
![Version](https://img.shields.io/badge/version-0.7.0--beta-orange?style=flat-square)
![Rust-Linux Workflow](https://github.com/mahinkumar/Nano_image_server/actions/workflows/Rust_Linux.yml/badge.svg)

<hr>

![image](https://github.com/user-attachments/assets/c43b43bf-b42e-4115-b225-da9a76f26894)
<hr>

A tiny, blazingly fast image server with on-the-fly processing and intelligent caching. Built with Axum and Tokio for true async, memory-safe, thread-safe performance.

## Features

### Core
- **Low-latency image delivery** - Optimized async I/O with Tokio runtime
- **HTTPS-only** - Secure by default with TLS 1.3 support
- **Modular design** - Enable only the features you need

### Optional Features
- **S3-FIFO Cache** *(new in v0.7)* - Intelligent frequency-based caching achieving **85%+ hit rates**
- **On-the-fly processing** - Resize, transform, filter images via URL parameters
- **Selective compilation** - Minimal builds for edge deployment

## Performance

With S3-FIFO cache enabled:
- **88.92% cache hit rate** on realistic workloads
- Sub-millisecond cached response times
- Thread-safe concurrent access with lock-free frequency tracking
- Automatic hot/cold data separation



## Usage

Minimal (serving only):
```bash
cargo build --release
```
With caching:
```bash
cargo build --release --features cache 
```
With image processing:
```bash
cargo build --release --features processing
```
Full featured:
```bash
cargo build --release --features cache,processing 
```
3. Place images in the images/ directory
```bash mkdir images
cp your-images/* images/
```
4. Start the server
```bash
# Basic usage
./target/release/nano_image_server --cert-path ./certs
```

### With cache (100 image capacity)
```bash
./target/release/nano_image_server --cert-path ./certs --cache-capacity 100
```

`Use --help for all available parameters`

<hr>

## Image operations

> [!WARNING]  
> A plugin based system for image operations is being developed. The provided API can change drastically until stable release.
> V0.6.0-beta does not include image processing by default but you can opt in by the following method during building

```bash
cargo build --release -F processing 
```

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
