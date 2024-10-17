# Use the official Rust image as the base
FROM rust:1.80.1

# Set the working directory inside the container
WORKDIR /app

# Copy the current directory contents into the container
COPY . .

# Install the wasm32-unknown-unknown target
RUN rustup target add wasm32-unknown-unknown

# Make sure build.sh is executable
RUN chmod +x build.sh

# Expose the server port (adjust if your server uses a different port)
EXPOSE 3000

# Set the environment variables (if needed)
ENV SERVER_IP=0.0.0.0
ENV SERVER_PORT=3000

# Run the build.sh script when the container launches
CMD ["./build.sh"]
