fn main() {
    rust_grpc_web::configure()
        .compile(&["../proto/team.proto"], &["../proto/"])
        .unwrap();
    rust_grpc_web::configure()
        .compile(&["../proto/game.proto"], &["../proto/"])
        .unwrap();
    rust_grpc_web::configure()
        .compile(&["../proto/game_match.proto"], &["../proto/"])
        .unwrap()
}
