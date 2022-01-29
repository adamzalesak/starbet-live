use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

use crate::user::user_service_server::UserService;
use crate::user::{
    Address, CreateUserReply, CreateUserRequest, EditUserReply, EditUserRequest, GetUserReply,
    GetUserRequest, User,
};

use database_layer::{
    connection::PgPool,
    db_access::{
        repo::Repo,
        user::{PgUserRepo, UserRepo},
    },
    db_models::{user::CreateUser, user_address::CreateUserAddress},
};

pub struct MyUserService {
    repo: PgUserRepo,
}

impl MyUserService {
    pub fn new(pool: &Arc<PgPool>) -> MyUserService {
        MyUserService {
            repo: PgUserRepo::new(pool),
        }
    }
}

#[tonic::async_trait]
impl UserService for MyUserService {
    async fn get_user(
        &self,
        request: Request<GetUserRequest>,
    ) -> Result<Response<GetUserReply>, Status> {
        let request = request.into_inner();
        match self.repo.get(request.id).await {
            Ok(user) => match self.repo.get_current_address(user.id).await {
                Ok(address) => Ok(Response::new(GetUserReply {
                    user: Some(User {
                        first_name: user.first_name,
                        last_name: user.last_name,
                        civil_id_number: user.civil_id_number,
                        email: user.email,
                        phone_number: user.phone_number,
                        photo: user.photo,
                        address: Some(Address {
                            street_name: address.street_name,
                            street_number: address.street_number,
                            city: address.city,
                            area: address.area,
                            postal_code: address.postal_code,
                            country: address.country,
                            valid_from: address.valid_from,
                        }),
                    }),
                })),
                Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
            },
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn create_user(
        &self,
        request: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserReply>, Status> {
        let request = request.into_inner();
        if let None = request.user {
            return Err(Status::new(Code::from_i32(13), "user is None"));
        }
        let user = request.user.unwrap();
        if let None = user.address {
            return Err(Status::new(Code::from_i32(13), "address is None"));
        }
        let address = user.address.unwrap();

        let create_user = CreateUser::new(
            &user.first_name,
            &user.last_name,
            &user.civil_id_number,
            &user.email,
            &user.phone_number,
            user.photo.as_deref(),
        );
        let create_user_address = CreateUserAddress::new(
            &address.street_name,
            &address.street_number,
            &address.city,
            address.area.as_deref(),
            &address.postal_code,
            &address.country,
        );

        match self.repo.create(create_user, create_user_address).await {
            Ok((user_id, _)) => Ok(Response::new(CreateUserReply { id: user_id })),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn edit_user(
        &self,
        request: Request<EditUserRequest>,
    ) -> Result<Response<EditUserReply>, Status> {
        let request = request.into_inner();
        match self.repo.get(request.id).await {
            Ok(user) => {
                let create_user = user.edit_user(
                    request.first_name.as_deref(),
                    request.last_name.as_deref(),
                    request.civil_id_number.as_deref(),
                    request.email.as_deref(),
                    request.phone_number.as_deref(),
                    Some(request.photo.as_deref()), // TODO
                );
                match self.repo.edit(user.id, create_user).await {
                    Ok(()) => Ok(Response::new(EditUserReply {})),
                    Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
                }
            }
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }
}
