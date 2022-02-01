pub mod create_user;
pub mod get_user;
use create_user::create_user;
use get_user::get_user;

/// run user testing options
pub async fn run(argument: Option<&str>) -> anyhow::Result<()> {
    match argument {
        Some(arg) => match arg {
            "get" => {
                get_user().await?;
            }
            "create" => {
                create_user().await?;
            }
            _ => anyhow::bail!("Wrong argument specified"),
        },
        None => anyhow::bail!("NO ARGUMENT SPECIFIED"),
    }

    Ok(())
}

// async fn get_user() -> anyhow::Result<()> {
//     dotenv().ok();
//     let database_url = env::var("DATABASE_URL")?;
//     let database_connection_pool: Arc<PgPool> =
//         Arc::new(db_connect_create_pool(&database_url).await?);
//     let pg_user: PgUserRepo = PgUserRepo::new(&database_connection_pool);

//     println!("Get a desired user:\nUser id:");

//     let mut input = "".to_string();
//     io::stdin().read_line(&mut input)?;
//     let user_id: i32 = input.trim().parse()?;

//     let user = pg_user.get(user_id).await;

//     match user {
//         Ok(user) => println!(
//             "[\n  User id: {}\n  User name: {} {}\n  User's email: {}\n  Date of creation: {}\n]\n",
//             user.id, user.first_name, user.last_name, user.email, user.created_at
//         ),
//         Err(error) => println!("ERROR: {}", error),
//     }

//     Ok(())
// }

// async fn
