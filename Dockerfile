FROM rust:latest AS builder
WORKDIR /usr/src/aprs-logger
COPY . .
RUN cargo install --path .

FROM debian:stable-slim AS runtime
LABEL authors="olove"

COPY --from=builder /usr/local/cargo/bin/aprs-logger /usr/local/bin/aprs-logger

CMD ["aprs-logger"]