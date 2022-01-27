use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserRegistrationFormData {
    pub first_name: String,
    pub last_name: String,
    pub password: String,
    pub civil_id_number: String,
    pub date_of_birth: String,
    pub email: String,
    pub phone_number: String,
    pub address: UserAddressRegistrationFormData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserAddressRegistrationFormData {
    pub street_name: String,
    pub street_number: String,
    pub city: String,
    pub area: Option<String>,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    FirstName,
    LastName,
    Password,
    CivilIdNumber,
    DateOfBirth,
    Email,
    PhoneNumber,
    StreetName,
    StreetNumber,
    City,
    Area,
    PostalCode,
    Country,
}

impl UserRegistrationFormData {
    pub fn new() -> Self {
        Self {
            first_name: String::new(),
            last_name: String::new(),
            password: String::new(),
            civil_id_number: String::new(),
            date_of_birth: String::new(),
            email: String::new(),
            phone_number: String::new(),
            address: UserAddressRegistrationFormData::new(),
        }
    }
}

impl UserAddressRegistrationFormData {
    fn new() -> Self {
        Self {
            street_name: String::new(),
            street_number: String::new(),
            city: String::new(),
            area: Some(String::new()),
            postal_code: String::new(),
            country: String::new(),
        }
    }
}
