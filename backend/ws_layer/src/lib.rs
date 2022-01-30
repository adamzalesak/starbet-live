use anyhow::Context;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use warp::{ws::Message, Filter, Rejection};

mod handlers;

#[derive(Debug, Clone)]
pub struct Client {
    pub client_id: String,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

pub type Clients = Arc<Mutex<HashMap<String, Client>>>;
pub type RouteClients = Arc<Mutex<HashMap<String, Clients>>>;
type Result<T> = std::result::Result<T, Rejection>;

pub type Msg = Message;

pub async fn run_ws_server(route_clients: RouteClients) -> anyhow::Result<()> {
    let bet_clients = route_clients
        .lock()
        .await
        .get("bet")
        .context("bet clients are absent")?
        .clone();
    let match_clients = route_clients
        .lock()
        .await
        .get("match")
        .context("match clients are absent")?
        .clone();

    let bet_route = warp::path("bet")
        .and(warp::ws())
        .and(with_clients(bet_clients))
        .and_then(handlers::ws_handler);

    let match_route = warp::path("match")
        .and(warp::ws())
        .and(with_clients(match_clients))
        .and_then(handlers::ws_handler);

    let routes = bet_route
        .or(match_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 50052)).await;
    Ok(())
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
