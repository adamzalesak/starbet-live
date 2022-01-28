use std::fmt::Display;

use crate::db_models::game_match::GameMatch;
use crate::schema::game_match_event;
use crate::type_storing::time_handling::TimeHandling;
use chrono::{DateTime, Utc};

/// Read structure, used for data mapping of
/// `game_match_event` record from the database
#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(GameMatch)]
#[table_name = "game_match_event"]
pub struct GameMatchEvent {
    pub id: i32,
    pub game_match_id: i32,
    pub event_type: String,
    pub created_at: String,
    pub until: Option<String>,
}

/// Write structure, used for inserting
/// `game_match_event` records into the database
#[derive(Insertable)]
#[table_name = "game_match_event"]
pub struct CreateGameMatchEvent {
    pub game_match_id: i32,
    pub event_type: String,
    pub created_at: String,
    pub until: Option<String>,
}

/// Structure capturing possible `game_match_event` types
#[derive(PartialEq)]
pub enum GameMatchEventType {
    Upcoming,
    Live(DateTime<Utc>),
    Cancelled,
    Overtime(DateTime<Utc>),
    Ended,
}

impl Display for GameMatchEventType {
    /// Implement the display trait for converting the enum and writing the result to the database
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_string = match self {
            GameMatchEventType::Upcoming => "Upcoming",
            GameMatchEventType::Live(_) => "Live",
            GameMatchEventType::Cancelled => "Cancelled",
            GameMatchEventType::Overtime(_) => "Overtime",
            GameMatchEventType::Ended => "Ended",
        };

        write!(f, "{}", self_string)
    }
}

impl GameMatchEvent {
    /// Convert the string representation of the `game_match_event` state into the enum
    ///
    /// Returns
    /// ---
    /// - Ok(state) - `GameMatchEventType` if the state has been stored and retrieved successfully
    /// - Err(_) - otherwise
    pub fn extract_event(&self) -> anyhow::Result<GameMatchEventType> {
        match self.event_type.as_ref() {
            "Upcoming" => Ok(GameMatchEventType::Upcoming),
            "Live" => Ok(GameMatchEventType::Overtime(TimeHandling::load_timestamp(
                &self
                    .until
                    .clone()
                    .unwrap_or_else(|| String::from("Will not convert")),
            )?)),
            "Cancelled" => Ok(GameMatchEventType::Cancelled),
            "Overtime" => Ok(GameMatchEventType::Overtime(TimeHandling::load_timestamp(
                &self
                    .until
                    .clone()
                    .unwrap_or_else(|| String::from("Will not convert")),
            )?)),
            "Ended" => Ok(GameMatchEventType::Ended),
            _ => anyhow::bail!(
                "Could not convert the database record into a proper game match event!"
            ),
        }
    }
}

impl CreateGameMatchEvent {
    /// Create a new `game_match_event` insert structure
    ///
    /// Params
    /// ---
    /// - game_match_id: id of the match we want to create an event for
    /// - event_type: type of the event we want to store
    ///
    /// Returns
    /// ---
    /// - new `game_match_event` insert structure
    pub fn new(game_match_id: i32, event_type: GameMatchEventType) -> Self {
        CreateGameMatchEvent {
            game_match_id,
            event_type: event_type.to_string(),
            created_at: TimeHandling::store(),
            until: match event_type {
                GameMatchEventType::Live(until) | GameMatchEventType::Overtime(until) => {
                    Some(until.to_string())
                }
                _ => None,
            },
        }
    }
}
