use tonic::{Request, Response, Status};

use crate::bet::bet_service_server::BetService;
use crate::bet::{
    Bet, CreateBetReply, CreateBetRequest, DeleteBetReply, DeleteBetRequest, ListBetsReply,
    ListBetsRequest, ListTicketBetsRequest, StreamBetsRequest,
};

use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct MyBetService {}

impl MyBetService {
    pub fn new() -> MyBetService {
        MyBetService {}
    }
}

type ResponseStream = ReceiverStream<Result<Bet, Status>>;

#[tonic::async_trait]
impl BetService for MyBetService {
    async fn list_bets(
        &self,
        request: Request<ListBetsRequest>,
    ) -> Result<Response<ListBetsReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = ListBetsReply { bets: vec![] };
        Ok(Response::new(reply))
    }

    async fn create_bet(
        &self,
        request: Request<CreateBetRequest>,
    ) -> Result<Response<CreateBetReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = CreateBetReply { id: 0 };
        Ok(Response::new(reply))
    }

    async fn delete_bet(
        &self,
        request: Request<DeleteBetRequest>,
    ) -> Result<Response<DeleteBetReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        Ok(Response::new(DeleteBetReply {}))
    }

    type StreamBetsStream = ResponseStream;

    async fn stream_bets(
        &self,
        request: Request<StreamBetsRequest>,
    ) -> Result<Response<Self::StreamBetsStream>, Status> {
        println!("Client connected from: {:?}", request.remote_addr());

        let (tx, rx) = mpsc::channel(4);

        let bets: Vec<Bet> = vec![Bet {
            id: 1,
            match_id: 1,
            rate: 1.0,
            team_id: 1,
            ticket_id: 1,
        }];

        tokio::spawn(async move {
            for bet in &bets[..] {
                tx.send(Ok(bet.clone())).await.unwrap();
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    async fn list_ticket_bets(
        &self,
        request: Request<ListTicketBetsRequest>,
    ) -> Result<Response<ListBetsReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = ListBetsReply { bets: vec![] };
        Ok(Response::new(reply))
    }
}
