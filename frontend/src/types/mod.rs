pub mod games;
pub mod matches;
pub mod tickets;
pub mod users;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
pub use users::{UserInfo, UserLoginFormData, UserRegistrationFormData};

pub use tickets::{BetInfo, TicketInfo};

/// Conduit api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}
