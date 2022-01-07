use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum MatchEventType {
    MatchUpcoming,
    MatchLive,
    MatchEnd,
    MatchOvertime,
    MatchCancelled,
}
