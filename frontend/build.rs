fn main() {
    rust_grpc_web::configure()
        .support_streaming(true)
        .compile(&["../proto/team.proto"], &["../proto/"])
        .unwrap();
    rust_grpc_web::configure()
        .support_streaming(true)
        .compile(&["../proto/game.proto"], &["../proto/"])
        .unwrap();
    rust_grpc_web::configure()
        .support_streaming(true)
        .compile(&["../proto/game_match.proto"], &["../proto/"])
        .unwrap()
}
