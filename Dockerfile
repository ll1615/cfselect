FROM golang:1.24.3-alpine as builder_go
WORKDIR /app
RUN apk update && apk add git
RUN git clone https://github.com/XIU2/CloudflareSpeedTest.git
RUN cd CloudflareSpeedTest && go build .

FROM rust:1.86.0 as builder_rust
WORKDIR /app
COPY . .
RUN cargo build --release
RUN mv target/release/cfselect /app/cfselect

FROM debian:bookworm-slim
WORKDIR /app
ENV PATH="$PATH:/app"
RUN apt-get update && apt-get install -y openssl
COPY ./assets ./assets
COPY ./config.toml .
COPY --from=builder_go /app/CloudflareSpeedTest/CloudflareSpeedTest .
COPY --from=builder_rust /app/cfselect .
CMD ["/app/cfselect"]