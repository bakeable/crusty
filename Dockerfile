# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory
WORKDIR /usr/src/app

# Copy the current directory contents into the container
COPY . .

# Build the application
RUN cargo build --release

# Run the application
CMD ["./target/release/crusty"]
