FROM alpine:3.19

EXPOSE 3000

COPY target/x86_64-unknown-linux-musl/release/lipl-storage-fs /usr/bin/

ENV LIPL_STORAGE_REPO_TYPE fs
ENV RUST_LOG info

WORKDIR /lipl

ENTRYPOINT [ "lipl-storage-fs" ]
