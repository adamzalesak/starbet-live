fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../../proto/bet.proto")?;
    tonic_build::compile_protos("../../proto/ticket.proto")?;
    tonic_build::compile_protos("../../proto/game_match.proto")?;
    tonic_build::compile_protos("../../proto/game.proto")?;
    Ok(())
}
