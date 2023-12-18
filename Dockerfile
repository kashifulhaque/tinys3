# Use a rust base image
FROM rust:1.74.1-slim-bookworm as builder

# Set the working directory
WORKDIR /usr/src/tinys3

# Copy the files to docker image
COPY . .

# Build the rust app
RUN cargo install --path .

# Use a minimal debian based base image for the final image
FROM debian:bookworm-slim

# Copy the binary from the build stage
COPY --from=builder /usr/local/cargo/bin/tinys3 /usr/local/bin/tinys3

# Expose the ports
EXPOSE 9000
EXPOSE 9090

# Start the application
CMD [ "tinys3" ]