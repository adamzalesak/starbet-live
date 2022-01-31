use serde::{Deserialize, Serialize};

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
