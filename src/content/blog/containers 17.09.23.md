+++
title = "containers"
date = 2017-09-23
weight = 1
+++


Assuming docker is already installed and running on local machine, in order to create an image all is needed is `Dockerfile`. This file is config of image. Here is simple Hello World example: 

`FROM ubuntu:latest`   - Downloading Ubuntu image from docker's website 

`WORKDIR /app`  - Creating working directory for image

`COPY . .`  - copying all the files from current directory into image

`CMD ["./HelloWorld"]`  - Running the binary

Depending on docker's image binaries should be changed accordingly. 

Made makefile and Dockerfile.