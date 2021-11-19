FROM rust:latest AS builder
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR frontend
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine
COPY --from=builder /frontend/target/x86_64-unknown-linux-musl/release/frontend /frontend
ENTRYPOINT ["/frontend"]
