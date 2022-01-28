use serde::{Deserialize, Serialize};

// structs for collecting data from the registration or login form and its correctness

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserRegistrationFormData {
    pub first_name: (String, bool),
    pub last_name: (String, bool),
    pub password: (String, bool),
    pub password_confirmation: (String, bool),
    pub civil_id_number: (String, bool),
    pub date_of_birth: (String, bool),
    pub email: (String, bool),
    pub phone_number: (String, bool),
    pub address: UserAddressRegistrationFormData,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserAddressRegistrationFormData {
    pub street_name: (String, bool),
    pub street_number: (String, bool),
    pub city: (String, bool),
    pub area: (Option<String>, bool),
    pub postal_code: (String, bool),
    pub country: (String, bool),
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserLoginFormData {
    pub email: String,
    pub password: String,
}

// struct represents user's data stored in the app
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct UserStorage {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub token: String,
    pub current_balance: String,
}

// Enum represents individual fields in user's forms
#[derive(Debug, Clone, PartialEq)]
pub enum Field {
    FirstName,
    LastName,
    Password,
    PasswordConfirmation,
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
            first_name: (String::new(), false),
            last_name: (String::new(), false),
            password: (String::new(), false),
            password_confirmation: (String::new(), false),
            civil_id_number: (String::new(), false),
            date_of_birth: (String::new(), false),
            email: (String::new(), false),
            phone_number: (String::new(), false),
            address: UserAddressRegistrationFormData::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.first_name.1
            && self.last_name.1
            && self.password.1
            && self.password_confirmation.1
            && self.civil_id_number.1
            && self.date_of_birth.1
            && self.email.1
            && self.phone_number.1
            && self.address.is_valid()
    }
}

impl UserAddressRegistrationFormData {
    fn new() -> Self {
        Self {
            street_name: (String::new(), false),
            street_number: (String::new(), false),
            city: (String::new(), false),
            area: (Some(String::new()), false),
            postal_code: (String::new(), false),
            country: (String::new(), false),
        }
    }

    fn is_valid(&self) -> bool {
        self.street_name.1
            && self.street_number.1
            && self.city.1
            && self.postal_code.1
            && self.country.1
    }
}

impl UserLoginFormData {
    pub fn new() -> Self {
        Self {
            email: String::new(),
            password: String::new(),
        }
    }
}

impl UserStorage {
    pub fn new() -> Self {
        Self {
            id: 0,
            first_name: String::new(),
            last_name: String::new(),
            token: String::new(),
            current_balance: String::new(),
        }
    }
}
