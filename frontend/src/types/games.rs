use serde::{Deserialize, Serialize};

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
