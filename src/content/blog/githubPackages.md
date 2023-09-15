+++
title = "github packages"
date = 2023-08-23
weight = 1
+++

# Github Packages

GitHub Packages is a platform for hosting and managing packages, including containers and other dependencies. It's simple to use and free up to 500 mb of storage. 

It can be used as alternative to docker hub.

As an example i compiled a simple dockerfile. 
```
FROM alpine:latest

RUN apk add zola

WORKDIR /app

COPY src /app

# Expose the port that Zola will listen on
EXPOSE 8080

# Configure Zola to listen on all available network interfaces

# Command to run when the container starts
CMD ["zola", "serve", "--interface", "0.0.0.0", "--port", "8080"]

```
This dockerfile creates simple image which serves zola static website on localhost. 
After that it can be pushed on github package.
`docker push ghcr.io/[username]/[name]:[version]`