use super::region::Region;

#[derive(Clone, Debug)]
pub struct Symbol {
    pub region: Region,
    pub identifier: String,
}

impl std::string::ToString for Symbol {
    fn to_string(&self) -> String {
        format!("{}.{}", self.identifier, self.region.to_string())
    }
}
