FROM frolvlad/alpine-glibc:latest

ADD ./target/release/isumi-server /usr/local/bin/isumi-server

ENTRYPOINT isumi-server

EXPOSE 8080
