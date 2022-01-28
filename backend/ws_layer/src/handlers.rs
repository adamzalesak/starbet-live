use crate::{Client, Clients, Result};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::ws::WebSocket;
use warp::Reply;

pub async fn bet_handler(ws: warp::ws::Ws, clients: Clients) -> Result<impl Reply> {
    println!("bet_handler");
    Ok(ws.on_upgrade(move |socket| bet_cb(socket, clients)))
}

pub async fn bet_cb(ws: WebSocket, clients: Clients) {
    println!("establishing client connection... {:?}", ws);
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();
    let client_rcv = UnboundedReceiverStream::new(client_rcv);
    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            println!("error sending websocket msg: {}", e);
        }
    }));
    let uuid = Uuid::new_v4().to_simple().to_string();
    let new_client = Client {
        client_id: uuid.clone(),
        sender: Some(client_sender),
    };
    clients.lock().await.insert(uuid.clone(), new_client);
    println!("cringaci");

    while let Some(_) = client_ws_rcv.next().await {}

    clients.lock().await.remove(&uuid);
    println!("{} disconnected", uuid);
}
