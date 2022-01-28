use database_layer::connection::db_connect_create_pool;
use futures::join;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::transport::Server;

mod handlers;

use handlers::bet_handler;
use handlers::game_handler;
use handlers::game_match_handler;
use handlers::ticket_handler;

mod bet {
    tonic::include_proto!("bet");
}
mod ticket {
    tonic::include_proto!("ticket");
}
mod game_match {
    tonic::include_proto!("game_match");
}
mod game {
    tonic::include_proto!("game");
}

use bet::bet_service_server::BetServiceServer;
use game::game_service_server::GameServiceServer;
use game_match::match_service_server::MatchServiceServer;
use ticket::ticket_service_server::TicketServiceServer;

pub async fn run_grpc_server(server_address: &str, database_url: &str) -> anyhow::Result<()> {
    let database_connection_pool = Arc::new(db_connect_create_pool(&database_url).await?);

    let ws_route_clients = Arc::new(Mutex::new(HashMap::new()));
    let bet_clients = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut ws_route_clients_locked = ws_route_clients.lock().await;
        ws_route_clients_locked.insert("bet".into(), bet_clients.clone());
    }

    let bet_service = bet_handler::MyBetService::new();
    let ticket_service = ticket_handler::MyTicketService::new();
    let game_match_service = game_match_handler::MyMatchService::new(&database_connection_pool);
    let game_service = game_handler::MyGameService::new(&database_connection_pool, bet_clients);

    let ws_server_coro = ws_layer::run_ws_server(ws_route_clients.clone());
    let grpc_server_coro = Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(BetServiceServer::new(bet_service)))
        .add_service(tonic_web::enable(TicketServiceServer::new(ticket_service)))
        .add_service(tonic_web::enable(MatchServiceServer::new(
            game_match_service,
        )))
        .add_service(tonic_web::enable(GameServiceServer::new(game_service)))
        .serve(server_address.parse()?);
    join!(ws_server_coro, grpc_server_coro);
    Ok(())
}
