use tonic::{Code, Request, Response, Status};

use crate::ticket::ticket_service_server::TicketService;
use crate::ticket::{
    GetCurrentTicketReply, GetCurrentTicketRequest, ListTicketsReply, ListTicketsRequest,
    SubmitTicketReply, SubmitTicketRequest,
};

pub struct MyTicketService {}

impl MyTicketService {
    pub fn new() -> MyTicketService {
        MyTicketService {}
    }
}

#[tonic::async_trait]
impl TicketService for MyTicketService {
    async fn get_current_ticket(
        &self,
        request: Request<GetCurrentTicketRequest>,
    ) -> Result<Response<GetCurrentTicketReply>, Status> {
        Err(Status::new(Code::from_i32(13), "Todo"))
    }

    async fn list_tickets(
        &self,
        request: Request<ListTicketsRequest>,
    ) -> Result<Response<ListTicketsReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = ListTicketsReply { tickets: vec![] };
        Ok(Response::new(reply))
    }

    async fn submit_ticket(
        &self,
        request: Request<SubmitTicketRequest>,
    ) -> Result<Response<SubmitTicketReply>, Status> {
        Err(Status::new(Code::from_i32(13), "Todo"))
    }
}
