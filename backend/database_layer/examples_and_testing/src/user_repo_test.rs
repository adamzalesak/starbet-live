mod add_balance;
mod add_new_address;
mod create_user;
mod edit_user;
mod get_balance;
mod get_current_address;
mod get_user;
mod spend_balance;
use add_balance::add_balance;
use add_new_address::new_address;
use create_user::create_user;
use edit_user::edit_user;
use get_balance::get_balance;
use get_current_address::get_current_address;
use get_user::get_user;
use spend_balance::spend_balance;

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
            "edit" => {
                edit_user().await?;
            }
            "new-address" => {
                new_address().await?;
            }
            "current-address" => {
                get_current_address().await?;
            }
            "balance" => {
                get_balance().await?;
            }
            "add-balance" => {
                add_balance().await?;
            }
            "spend-balance" => {
                spend_balance().await?;
            }
            _ => anyhow::bail!("Wrong argument specified"),
        },
        None => anyhow::bail!("NO ARGUMENT SPECIFIED"),
    }

    Ok(())
}
