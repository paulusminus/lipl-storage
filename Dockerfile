FROM alpine:latest

EXPOSE 3000

COPY target/x86_64-unknown-linux-musl/release/lipl-storage-server /usr/bin/

RUN mkdir -p /pwa && wget -qO- https://github.com/paulusminus/lipl-control/releases/download/v1.13.1/lipl-pwa.tar.gz | tar xzv -C /pwa

ENV LIPL_STORAGE_REPO_TYPE=fs
ENV RUST_LOG=info
ENV WWW_ROOT=/pwa

WORKDIR /lipl

ENTRYPOINT [ "lipl-storage-server" ]
