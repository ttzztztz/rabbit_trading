use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Market {
    CN,
    HK,
    US,
}

impl std::string::ToString for Market {
    fn to_string(&self) -> String {
        match self {
            Market::CN => String::from("CN"),
            Market::HK => String::from("HK"),
            Market::US => String::from("US"),
        }
    }
}
