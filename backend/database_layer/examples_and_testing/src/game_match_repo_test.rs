mod create;
mod delete;
mod get;
mod get_all;
mod get_info;
mod get_ratios;
mod set_ratios;
mod update_status;
use create::create_game_match;
use delete::delete_game_match;
use get::get;
use get_all::get_all;
use get_info::get_show_info;
use get_ratios::get_ratios;
use set_ratios::set_ratios;
use update_status::update_status;

/// run user testing options
pub async fn run(argument: Option<&str>) -> anyhow::Result<()> {
    match argument {
        Some(arg) => match arg {
            "create" => {
                create_game_match().await?;
            }
            "delete" => {
                delete_game_match().await?;
            }
            "get" => {
                get().await?;
            }
            "get-all" => {
                get_all().await?;
            }
            "get-ratios" => {
                get_ratios().await?;
            }
            "get-show-info" => {
                get_show_info().await?;
            }
            "set-ratios" => {
                set_ratios().await?;
            }
            "update-status" => {
                update_status().await?;
            }
            _ => anyhow::bail!("Wrong argument specified"),
        },
        None => anyhow::bail!("NO ARGUMENT SPECIFIED"),
    }

    Ok(())
}
