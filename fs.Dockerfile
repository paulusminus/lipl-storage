FROM alpine:3.18

EXPOSE 3000

COPY target/x86_64-unknown-linux-musl/release/lipl-server-axum-fs /usr/bin/

ENV LIPL_STORAGE_REPO_TYPE fs
ENV LIPL_STORAGE_FS_DIR lipl
ENV RUST_LOG trace

RUN mkdir -p lipl

ENTRYPOINT [ "lipl-server-axum-fs" ]
