FROM alpine:latest

RUN apk add zola
# Get the version of Zola
RUN apk info -vv | grep -E 'zola'

WORKDIR /app

COPY . /app

CMD ["zola", "build"]

VOLUME /app/public