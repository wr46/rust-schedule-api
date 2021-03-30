FROM rustlang/rust:nightly AS builder

WORKDIR /app
COPY src ./src
COPY Cargo.toml Cargo.lock Rocket.toml ./

RUN rustup target add x86_64-unknown-linux-musl
RUN cargo test
RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/rust-schedule-api .

USER 1000
CMD ["./rust-schedule-api"]
