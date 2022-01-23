fn main() {
    rust_grpc_web::configure()
        .support_streaming(true)
        .compile(&["../proto/game.proto"], &["../proto/"])
        .unwrap();
}
