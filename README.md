Project Name: tiles_rs

## Brief Introduction

Tiles_rs is a high-performance, lightweight tile map server written in Rust. It efficiently serves map tiles for web mapping applications, leveraging Rust's speed and safety features to provide a robust and scalable solution for geographic data visualization.

## Detailed Description

Tiles_rs is an open-source project that aims to provide a fast and reliable tile map server implementation using Rust. Built on top of the Actix web framework, this project offers a modern approach to serving map tiles, catering to the needs of developers working on geographic information systems (GIS) and web mapping applications.

## Key Features

- High Performance: Utilizing Rust's zero-cost abstractions and efficient memory management, RusticTiles delivers exceptional speed in serving map tiles.
- Lightweight: With a minimal dependency footprint, the server maintains a small resource footprint, making it suitable for deployment in various environments, from powerful servers to resource-constrained systems.
- RESTful API: The server exposes a simple and intuitive RESTful API for requesting map tiles, following common conventions used in the geospatial industry.
- Flexible Tile Storage: RusticTiles supports serving tiles from a local file system, allowing easy integration with pre-generated tile sets.

## Usage

```
tiles_rs.exe --help
tiles_rs.exe --tiles-dir=C:\\Users\\Tiles --host=0.0.0.0 --port=5000 --log_level=warn
```