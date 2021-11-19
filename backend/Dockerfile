FROM rust:latest AS builder
RUN rustup target add x86_64-unknown-linux-musl
WORKDIR backend
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine
COPY --from=builder /backend/target/x86_64-unknown-linux-musl/release/backend /backend
ENTRYPOINT ["/backend"]
