FROM alpine:latest

RUN apk add zola

WORKDIR /app

COPY src /app

# Expose the port that Zola will listen on
EXPOSE 8080

# Configure Zola to listen on all available network interfaces

# Command to run when the container starts
CMD ["zola", "serve", "--interface", "0.0.0.0", "--port", "8080"]
