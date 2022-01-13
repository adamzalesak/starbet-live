use tonic::{Request, Response, Status};

use crate::ticket::ticket_service_server::TicketService;
use crate::ticket::{
    CreateTicketReply, CreateTicketRequest, DeleteTicketRequest, ListTicketsReply,
    ListTicketsRequest,
};

pub struct MyTicketService {}

impl MyTicketService {
    pub fn new() -> MyTicketService {
        MyTicketService {}
    }
}

#[tonic::async_trait]
impl TicketService for MyTicketService {
    async fn list_tickets(
        &self,
        request: Request<ListTicketsRequest>,
    ) -> Result<Response<ListTicketsReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = ListTicketsReply { tickets: vec![] };
        Ok(Response::new(reply))
    }

    async fn create_ticket(
        &self,
        request: Request<CreateTicketRequest>,
    ) -> Result<Response<CreateTicketReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = CreateTicketReply { id: 0 };
        Ok(Response::new(reply))
    }

    async fn delete_ticket(
        &self,
        request: Request<DeleteTicketRequest>,
    ) -> Result<Response<()>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        Ok(Response::new(()))
    }
}
