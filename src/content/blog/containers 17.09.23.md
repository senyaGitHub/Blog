+++
title = "containers"
date = 2023-08-17
weight = 1
+++

Docker is a platform and toolset that allows you to create, deploy, and manage applications within lightweight, portable containers. It can be used to compile something without installing dependencies. This allows for testing in different environments without using virtual machines


# Example
Assuming docker is already installed and running on local machine, in order to create an image all is needed is `Dockerfile`. This file is config of image. Here is simple Hello World example: 

`FROM ubuntu:latest`   - Downloading Ubuntu image from docker's website 

`WORKDIR /app`  - Creating working directory for image

`COPY . .`  - copying all the files from current directory into image

`CMD ["./HelloWorld"]`  - Running the binary

Depending on docker's image binaries should be changed accordingly. 

Build docker image `docker build -t getting-started .`