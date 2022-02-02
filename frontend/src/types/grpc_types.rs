pub mod game {
    include!(concat!(env!("OUT_DIR"), concat!("/game.rs")));
}
pub mod game_match {
    include!(concat!(env!("OUT_DIR"), concat!("/game_match.rs")));
}
pub mod team {
    include!(concat!(env!("OUT_DIR"), concat!("/team.rs")));
}
pub mod ticket {
    include!(concat!(env!("OUT_DIR"), concat!("/ticket.rs")));
}
pub mod bet {
    include!(concat!(env!("OUT_DIR"), concat!("/bet.rs")));
}
pub mod user {
    include!(concat!(env!("OUT_DIR"), concat!("/user.rs")));
}