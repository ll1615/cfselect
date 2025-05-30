FROM golang:1.24.3-alpine as builder_go
WORKDIR /app
RUN apk update && apk add git
RUN git clone https://github.com/XIU2/CloudflareSpeedTest.git
RUN cd CloudflareSpeedTest && go build .

FROM rust:1.86.0 as builder_rust
RUN cargo new --bin cfselect
WORKDIR /cfselect
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release && rm src/*.rs && rm ./target/release/deps/cfselect*
# 项目代码变动会重新构建
COPY src ./src
RUN cargo build --release
RUN mv ./target/release/cfselect ./cfselect

FROM rust:1.86.0-slim-bookworm
WORKDIR /app
ENV PATH="$PATH:/app"
COPY assets ./assets
COPY config.toml .
COPY --from=builder_go /app/CloudflareSpeedTest/CloudflareSpeedTest .
COPY --from=builder_rust /cfselect/cfselect .
CMD ["/app/cfselect"]