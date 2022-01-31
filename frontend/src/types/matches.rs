use serde::{Deserialize, Serialize};

// Creating new match

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateMatchForm {
    pub game_id: (u32, bool),
    pub team_one_id: (u32, bool),
    pub team_two_id: (u32, bool),
    pub team_one_ratio: (String, bool),
    pub team_two_ratio: (String, bool),
    pub supposed_start_at: (String, bool),
}

impl CreateMatchForm {
    pub fn new() -> Self {
        Self {
            game_id: (0, false),
            team_one_id: (0, false),
            team_two_id: (0, false),
            team_one_ratio: (String::new(), false),
            team_two_ratio: (String::new(), false),
            supposed_start_at: (String::new(), false),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.game_id.1
            && self.team_one_id.1
            && self.team_two_id.1
            && self.team_one_ratio.1
            && self.team_two_ratio.1
            && self.supposed_start_at.1
    }
}

// Creating new game

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateGameFormData {
    pub name: (String, bool),
    pub logo_url: (String, bool),
}

impl CreateGameFormData {
    pub fn new() -> Self {
        Self {
            name: (String::new(), false),
            logo_url: (String::new(), false),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.name.1 && self.logo_url.1
    }
}

// Creating new team

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CreateTeamFormData {
    pub name: (String, bool),
    pub description: (String, bool),
    pub logo_url: (String, bool),
}

impl CreateTeamFormData {
    pub fn new() -> Self {
        Self {
            name: (String::new(), false),
            description: (String::new(), false),
            logo_url: (String::new(), false),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.name.1 && self.logo_url.1 && self.description.1
    }
}
