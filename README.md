> [!NOTE]  
> This Project is a work in progress and is not suitable to be used at this moment.
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
