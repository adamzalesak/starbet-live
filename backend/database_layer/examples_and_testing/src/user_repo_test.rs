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
