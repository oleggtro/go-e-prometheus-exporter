FROM docker.io/library/rust:1.90-alpine AS builder

WORKDIR /build
RUN apk add build-base
COPY Cargo.toml Cargo.lock ./
COPY src src/
RUN cargo build --release

#FROM docker.io/library/rust:1.90-alpine
FROM scratch AS runner
ARG BIN_NAME=go-e-prometheus-exporter

# link image to repo
LABEL org.opencontainers.image.source="https://github.com/oleggtro/go-e-prometheus-exporter"
LABEL org.opencontainers.image.authors="ole@oleggtro.com"
LABEL org.opencontainers.image.description="a prometheus exporter for the go-e home solar controller"


COPY --from=builder /build/target/release/$BIN_NAME /usr/local/bin/$BIN_NAME
EXPOSE 9186
CMD ["go-e-prometheus-exporter"]
