use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

use crate::team::team_service_server::TeamService;
use crate::team::{
    AddTeamToGameReply, AddTeamToGameRequest, CreateTeamReply, CreateTeamRequest, GetTeamReply,
    GetTeamRequest, RemoveTeamFromGameReply, RemoveTeamFromGameRequest, Team,
};

use database_layer::{
    connection::PgPool,
    db_access::{
        repo::Repo,
        team::{PgTeamRepo, TeamRepo},
    },
    db_models::team::CreateTeam,
};

pub struct MyTeamService {
    repo: PgTeamRepo,
}

impl MyTeamService {
    pub fn new(pool: &Arc<PgPool>) -> MyTeamService {
        MyTeamService {
            repo: PgTeamRepo::new(pool),
        }
    }
}

#[tonic::async_trait]
impl TeamService for MyTeamService {
    async fn get_team(
        &self,
        request: Request<GetTeamRequest>,
    ) -> Result<Response<GetTeamReply>, Status> {
        let request = request.into_inner();
        match self.repo.get(request.id).await {
            Ok(team) => Ok(Response::new(GetTeamReply {
                team: Some(Team {
                    name: team.name,
                    description: team.description,
                    logo: team.logo,
                }),
            })),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn create_team(
        &self,
        request: Request<CreateTeamRequest>,
    ) -> Result<Response<CreateTeamReply>, Status> {
        let request = request.into_inner();
        if let None = request.team {
            return Err(Status::new(Code::from_i32(13), "team is None"));
        }
        let team = request.team.unwrap();

        let create_team = CreateTeam::new(&team.name, &team.description, &team.logo);

        match self.repo.create(create_team).await {
            Ok(team_id) => Ok(Response::new(CreateTeamReply { id: team_id })),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn add_team_to_game(
        &self,
        request: Request<AddTeamToGameRequest>,
    ) -> Result<Response<AddTeamToGameReply>, Status> {
        let request = request.into_inner();
        match self
            .repo
            .add_to_game(request.team_id, request.game_id)
            .await
        {
            Ok(()) => Ok(Response::new(AddTeamToGameReply {})),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn remove_team_from_game(
        &self,
        request: Request<RemoveTeamFromGameRequest>,
    ) -> Result<Response<RemoveTeamFromGameReply>, Status> {
        let request = request.into_inner();
        match self
            .repo
            .remove_from_game(request.team_id, request.game_id)
            .await
        {
            Ok(()) => Ok(Response::new(RemoveTeamFromGameReply {})),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }
}
