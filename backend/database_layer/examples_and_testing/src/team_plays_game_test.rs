pub mod team_in_game;
use team_in_game::{add_team_to_the_game, remove_team_from_game};

pub async fn run(argument: Option<&str>) -> anyhow::Result<()> {
    match argument {
        Some(arg) => match arg {
            "add" => {
                add_team_to_the_game().await?;
            }
            "remove" => {
                remove_team_from_game().await?;
            }
            _ => anyhow::bail!("Wrong argument specified"),
        },
        None => anyhow::bail!("NO ARGUMENT SPECIFIED"),
    }

    Ok(())
}
