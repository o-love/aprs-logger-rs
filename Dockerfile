FROM rust:latest AS builder
WORKDIR /usr/src/aprs-logger-rs
COPY . .
RUN cargo install --path .

FROM debian:stable-slim AS runtime
LABEL authors="olove"

COPY --from=builder /usr/local/cargo/bin/aprs-logger-rs /usr/local/bin/aprs-logger-rs

CMD ["aprs-logger-rs"]