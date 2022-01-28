use crate::schema::team;

/// Read structure, used for data mapping of
/// `team` record from the database
#[derive(Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub logo: String,
}

/// Write structure, used for inserting
/// `team` records into the database
#[derive(Insertable, AsChangeset)]
#[table_name = "team"]
pub struct CreateTeam {
    pub name: String,
    pub description: String,
    pub logo: String,
}

impl CreateTeam {
    /// Create a new `team` insert structure
    ///
    /// Params
    /// ---
    /// - name: name of the team we wish to create
    /// - description: team description
    /// - logo: path to the team's logo
    ///
    /// Returns
    /// ---
    /// - new `team` insert structure
    pub fn new(name: &str, description: &str, logo: &str) -> Self {
        Self {
            name: String::from(name),
            description: String::from(description),
            logo: String::from(logo),
        }
    }
}
