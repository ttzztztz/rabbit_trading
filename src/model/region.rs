#[derive(Clone, Debug)]
pub enum Region {
    CN,
    HK,
    US,
}

impl std::string::ToString for Region {
    fn to_string(&self) -> String {
        match self {
            Region::CN => String::from("CN"),
            Region::HK => String::from("HK"),
            Region::US => String::from("US"),
        }
    }
}
