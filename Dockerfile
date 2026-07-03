# ------------------------------
# Stage 1. Build an app
# ------------------------------
FROM rust:1.96.0 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# ------------------------------
# Stage 2. Build for runtime
# ------------------------------
FROM dhi.io/debian-base:trixie

ARG GIT_REVISION
ARG BUILD_DATE
ARG VERSION

LABEL org.opencontainers.image.title="nls" \
      org.opencontainers.image.description="ls without month abbreviation" \
      org.opencontainers.image.url="https://kato1052.github.io/nls/" \
      org.opencontainers.image.source="https://github.com/Kato1052/nls" \
      org.opencontainers.image.version=${VERSION} \
      org.opencontainers.image.revision=${GIT_REVISION} \
      org.opencontainers.image.created=${BUILD_DATE} \
      org.opencontainers.image.licenses="MIT"

COPY --from=builder /app/target/release/nls /app/nls
WORKDIR /opt

ENTRYPOINT [ "/app/nls" ]
