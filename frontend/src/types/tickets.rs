// use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BetInfo {
    pub id: u32,
    pub team1: String,
    pub team2: String,
    pub bet_ratio: f32,
    pub bet_team: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct BetInfoWrapper {
    pub bet: BetInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TicketInfo {
    pub id: u32,
    pub ticket_ratio: f32,
    pub ticket_value: f32,
    pub bets: Vec<BetInfo>,
    // pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TicketInfoWrapper {
    pub ticket: TicketInfo,
}
