use dotenv::dotenv;
use grpc_layer;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let server_address = env::var("SERVER_ADDRESS")?;
    grpc_layer::run_grpc_server(&server_address, &database_url).await?;
    Ok(())
}
