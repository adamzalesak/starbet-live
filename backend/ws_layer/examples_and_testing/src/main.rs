use ws_layer;

use futures::join;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

async fn send_loop(clients: ws_layer::Clients) {
    loop {
        for client in clients.lock().await.values() {
            if let Some(sender) = &client.sender {
                sender.send(Ok(ws_layer::Msg::text("pong")));
            }
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let route_clients = Arc::new(Mutex::new(HashMap::new()));
    {
        let mut route_clients_locked = route_clients.lock().await;
        route_clients_locked.insert("bet".into(), Arc::new(Mutex::new(HashMap::new())));
    }
    let server_coro = ws_layer::run_ws_server(route_clients.clone());
    let send_coro = send_loop(route_clients.lock().await.get("bet").unwrap().clone());
    join!(server_coro, send_coro);
    Ok(())
}
