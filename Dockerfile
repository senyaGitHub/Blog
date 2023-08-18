# Use a Rust base image
FROM rust:latest AS build

# Set the working directory to /app
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src src

# Build the application with cargo
RUN cargo build --release 
# Use a lightweight alpine image for the final container
FROM ubuntu:latest

# Set the working directory to /app
WORKDIR /app

# Copy the binary from the build container to the final container
COPY --from=build /app/target/release/something .

# Start the application
CMD ["./something"]
