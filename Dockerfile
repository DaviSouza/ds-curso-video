# Build: docker build -t rustwebservice . 
#         && docker images
# Run: docker run -p 8000:8000 rustwebservice // docker run -p <host-port>:<container-port> <image-name>
# Test: curl http://localhost:8000/

# Use rust-based image for container; rustc version 1.43.1
FROM rust:latest AS builder

# Set working directory in container
RUN mkdir /usr/src/ds-curso-video
WORKDIR /usr/src/ds-curso-video

# Copy all source code file from local computer to container
COPY src src
COPY Cargo.toml .

# Build release application
RUN cargo build --release

# Expose listening port for application
EXPOSE 8000

# Run the application
CMD ["target/release/ds-curso-video"]