# Using the Rust official image...
FROM rust:1.67

# Copy the files in your machine to the Docker image...
COPY ./ ./

# Build your program for release...
RUN cargo build --release

# And run the binary!
CMD ["./target/release/server"]
