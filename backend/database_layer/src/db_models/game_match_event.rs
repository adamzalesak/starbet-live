use std::fmt::Display;

use crate::db_models::game_match::GameMatch;
use crate::schema::game_match_event;
use crate::type_storing::time_handling::CurrentTime;
use chrono::{DateTime, Utc};

/// Read structure, used for data mapping of
/// GameMatchEvent record from the database
#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(GameMatch)]
#[table_name = "game_match_event"]
pub struct GameMatchEvent {
    pub id: i32,
    pub game_match_id: i32,
    pub event_type: String,
    pub created_at: String,
    pub overtime_until: Option<String>,
}

/// Write structure, used for inserting
/// GameMatchEvent records into the database
#[derive(Insertable)]
#[table_name = "game_match_event"]
pub struct CreateGameMatchEvent {
    pub game_match_id: i32,
    pub event_type: String,
    pub created_at: String,
    pub overtime_until: Option<String>,
}

/// Structure capturing possible GameMatch event types
#[derive(PartialEq)]
pub enum GameMatchEventType {
    Upcoming,
    Live,
    Cancelled,
    Overtime(DateTime<Utc>),
    Ended,
}

impl Display for GameMatchEventType {
    /// Write a text representation of own type into whatever buffer it needs to
    ///
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let self_string = match self {
            GameMatchEventType::Upcoming => "Upcoming",
            GameMatchEventType::Live => "Live",
            GameMatchEventType::Cancelled => "Cancelled",
            GameMatchEventType::Overtime(_) => "Overtime",
            GameMatchEventType::Ended => "Ended",
        };

        write!(f, "{}", self_string)
    }
}

impl GameMatchEventType {
    /// Returns a GameMatchEventType if the record in the database
    /// has been correctly stored
    ///
    /// Params
    /// ---
    /// - input: a game match event record, loaded from the database
    ///
    /// Returns
    /// ---
    /// - Ok(type) - GameMatchEventType if the type has been stored successfully
    /// - Err(_) - if an error occurrs while parsing the structure from the database
    pub fn from_record(input: GameMatchEvent) -> anyhow::Result<GameMatchEventType> {
        match input.event_type.as_ref() {
            "Upcoming" => Ok(GameMatchEventType::Upcoming),
            "Live" => Ok(GameMatchEventType::Live),
            "Cancelled" => Ok(GameMatchEventType::Cancelled),
            "Overtime" => Ok(GameMatchEventType::Overtime(CurrentTime::load_timestamp(
                &input
                    .overtime_until
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
    /// Create a new insert structure for game event in the database
    ///
    /// Params
    /// ---
    /// - game_match_id: id of the match we want to create an event for
    /// - event_type: type of the event we want to store
    ///
    /// Returns
    /// ---
    /// - new insert structure for creating the
    pub fn new(game_match_id: i32, event_type: GameMatchEventType) -> Self {
        let overtime_until = match event_type {
            GameMatchEventType::Overtime(until) => Some(until.to_string()),
            _ => None,
        };

        CreateGameMatchEvent {
            game_match_id,
            event_type: event_type.to_string(),
            created_at: CurrentTime::store(),
            overtime_until,
        }
    }
}
