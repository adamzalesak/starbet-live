use anyhow::Context;
use database_layer::connection::db_connect_create_pool;
use futures::try_join;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;
use tonic::transport::Server;

use ws_layer::RouteClients;

mod handlers;
mod repos;
mod transform;

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
mod user {
    tonic::include_proto!("user");
}
mod team {
    tonic::include_proto!("team");
}

use bet::bet_service_server::BetServiceServer;
use game::game_service_server::GameServiceServer;
use game_match::match_service_server::MatchServiceServer;
use team::team_service_server::TeamServiceServer;
use ticket::ticket_service_server::TicketServiceServer;
use user::user_service_server::UserServiceServer;

async fn serve_grpc_server(
    server_address: &str,
    database_url: &str,
    ws_route_clients: RouteClients,
) -> anyhow::Result<()> {
    let db_conn_pool = Arc::new(db_connect_create_pool(&database_url).await?);
    let bet_clients = ws_route_clients
        .lock()
        .await
        .get("bet")
        .context("bet clients are absent")?
        .clone();
    let match_clients = ws_route_clients
        .lock()
        .await
        .get("match")
        .context("match clients are absent")?
        .clone();

    let bet_service = handlers::bet::MyBetService::new(&db_conn_pool, bet_clients);
    let ticket_service =
        handlers::ticket::MyTicketService::new(&db_conn_pool, match_clients.clone());
    let game_match_service =
        handlers::game_match::MyMatchService::new(&db_conn_pool, match_clients);
    let game_service = handlers::game::MyGameService::new(&db_conn_pool);
    let user_service = handlers::user::MyUserService::new(&db_conn_pool);
    let team_service = handlers::team::MyTeamService::new(&db_conn_pool);

    Server::builder()
        .accept_http1(true)
        .add_service(tonic_web::enable(BetServiceServer::new(bet_service)))
        .add_service(tonic_web::enable(TicketServiceServer::new(ticket_service)))
        .add_service(tonic_web::enable(MatchServiceServer::new(
            game_match_service,
        )))
        .add_service(tonic_web::enable(GameServiceServer::new(game_service)))
        .add_service(tonic_web::enable(UserServiceServer::new(user_service)))
        .add_service(tonic_web::enable(TeamServiceServer::new(team_service)))
        .serve(server_address.parse()?)
        .await?;
    Ok(())
}

pub async fn run_grpc_server(server_address: &str, database_url: &str) -> anyhow::Result<()> {
    let ws_route_clients = Arc::new(Mutex::new(HashMap::new()));
    {
        let bet_clients = Arc::new(Mutex::new(HashMap::new()));
        let match_clients = Arc::new(Mutex::new(HashMap::new()));
        let mut ws_route_clients_locked = ws_route_clients.lock().await;
        ws_route_clients_locked.insert("bet".into(), bet_clients);
        ws_route_clients_locked.insert("match".into(), match_clients);
    }

    let ws_server_coro = ws_layer::run_ws_server(ws_route_clients.clone());
    let grpc_server_coro = serve_grpc_server(server_address, database_url, ws_route_clients);
    try_join!(ws_server_coro, grpc_server_coro)?;
    Ok(())
}
