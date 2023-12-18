# **Tiny S3** 🦀

A self-hosted alternative to S3 (not meant to be used in production)

## Prerequisites 🛠️

Make sure you have Rust installed on your system. If not, follow these steps to install Rust:

1. Visit [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
2. Follow the instructions for your operating system to install Rust.

## Clone the Repository 📥

Now that Rust is installed, let's clone the repository:

```bash
git clone https://github.com/kashifulhaque/tinys3
cd tinys3
```

## Run the Actix Web Server 🚀

Navigate to the project directory and run the server with the following commands:

```bash
cd tinys3
cargo run
```

This will start the server on `http://127.0.0.1:8080`. Open your web browser and visit [http://127.0.0.1:8080](http://127.0.0.1:8080) to see the "Hello World!" message.

## Explore Endpoints 🌐

- **Root Endpoint:**
  - [http://127.0.0.1:8080](http://127.0.0.1:8080)

- **Echo Endpoint (POST):**
  - Use a tool like [curl](https://curl.se/) or [Postman](https://www.postman.com/) to send a POST request to [http://127.0.0.1:8080/echo](http://127.0.0.1:8080/echo) with a request body. The server will echo back the provided data.

- **Manual Index Endpoint:**
  - [http://127.0.0.1:8080/api](http://127.0.0.1:8080/api)