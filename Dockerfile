FROM rust:1.79.0-alpine3.20 as builder

RUN apk add --no-cache musl-dev \
    && rm -rf /var/cache/apk/*

WORKDIR /build
COPY . /build
RUN cargo build --release

FROM alpine:3.20
ENV GID 1001
ENV UID 1001
ENV USER dockeruser
ENV PATH=/app/bin:${PATH}

RUN mkdir -p /app/bin /app/conf \
    && adduser -D $USER \
    && chown $USER:$USER /app/bin

USER $USER
COPY --from=builder /build/target/release/rust_rational /app/bin
WORKDIR /data

ENTRYPOINT ["/app/bin/rust_rational"]
