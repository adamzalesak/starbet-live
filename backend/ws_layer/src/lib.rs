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
    let ws_route = warp::path("bet")
        .and(warp::ws())
        .and(with_clients(bet_clients))
        .and_then(handlers::bet_handler);

    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
    Ok(())
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
