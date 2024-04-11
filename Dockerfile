FROM alpine:3.19

EXPOSE 3000

COPY target/x86_64-unknown-linux-musl/release/lipl-storage-server /usr/bin/
COPY --from=pwa / /pwa/

ENV LIPL_STORAGE_REPO_TYPE fs
ENV RUST_LOG info
ENV WWW_ROOT /pwa

WORKDIR /lipl

ENTRYPOINT [ "lipl-storage-server" ]
