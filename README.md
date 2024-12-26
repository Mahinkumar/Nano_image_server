> [!WARNING]  
> This Project is a work in progress and is not suitable to be used at this moment.  <br>
> The Processing algorithms are far from optimal and will undergo massive Improvements. <br>
> Star the repository for progress updates.

# Nano Image Server
Nano Image Server is a tiny, blazingly fast service to serve images with support for image operation on fly.

## Features
1. Low latency Image delivery
2. Image operation on fly via url queries
3. Support for GPU Acceleration
4. Simple Image browsing utility
5. Caching and Instant Retrieval

## Working

The following request returns a processed image of resolution 1920x1080px with black and white filter
```
https://<imageserver>.com/image/Nature.jpg?resx=1920&resy=1080&filter=bw
```

## Usage

1. Place Images you need in images folder next to the executable
2. Start the server
```bash
./nano_image_server #Linux
start nano_image_server.exe #Windows
```
3. Access the server from port 8000 in localhost.
4. To get image go to `/image/<imagename>.<format>`
5. If needed resizing use queries resx and resy `/image/Nature.jpg?resx=1920&resy=1080`
6. If specified size is 0 or left unspecified they display original size of the image


## Benchmarks 

### Nano_image_server **v0.1.0-alpha** With ApacheBench on 24 Threads (Balanced Power Mode on a Laptop)
```markdown
# Command 
ab -n 1000 -c 24 -k http://localhost:8000/image/in.jpg?resx=1080&resy=1920

# Benchmark ----
Server Hostname:        localhost
Server Port:            8000

Document Path:          /image/in.jpg?resx=1080
Document Length:        12079492 bytes

Concurrency Level:      24
Time taken for tests:   78.532 seconds
Complete requests:      1000
Failed requests:        0
Keep-Alive requests:    1000
Total transferred:      12079624000 bytes
HTML transferred:       12079492000 bytes
Requests per second:    12.73 [#/sec] (mean)
Time per request:       1884.773 [ms] (mean)
Time per request:       78.532 [ms] (mean, across all concurrent requests)
Transfer rate:          150212.34 [Kbytes/sec] received
```

