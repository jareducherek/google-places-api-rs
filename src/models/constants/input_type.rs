use serde::{Deserialize};

#[derive(Debug, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum InputType {
    TextQuery,
    PhoneNumber,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputType::TextQuery => "textquery",
            InputType::PhoneNumber => "phonenumber",
        }
    }
}
