use tonic::transport::Server;

mod grpc_handlers;

use grpc_handlers::bet_handler;
use grpc_handlers::game_handler;
use grpc_handlers::game_match_handler;
use grpc_handlers::ticket_handler;

pub mod bet {
    tonic::include_proto!("bet");
}
pub mod ticket {
    tonic::include_proto!("ticket");
}
pub mod game_match {
    tonic::include_proto!("game_match");
}
pub mod game {
    tonic::include_proto!("game");
}

use bet::bet_service_server::BetServiceServer;
use game::game_service_server::GameServiceServer;
use game_match::game_match_service_server::GameMatchServiceServer;
use ticket::ticket_service_server::TicketServiceServer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address = "127.0.0.1:50051".parse()?;

    let bet_service = bet_handler::MyBetService::new();
    let ticket_service = ticket_handler::MyTicketService::new();
    let game_match_service = game_match_handler::MyGameMatchService::new();
    let game_service = game_handler::MyGameService::new();

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(BetServiceServer::new(bet_service)))
        .add_service(tonic_web::enable(TicketServiceServer::new(ticket_service)))
        .add_service(tonic_web::enable(GameMatchServiceServer::new(game_match_service)))
        .add_service(tonic_web::enable(GameServiceServer::new(game_service)))
        .serve(address)
        .await?;

    Ok(())
}
