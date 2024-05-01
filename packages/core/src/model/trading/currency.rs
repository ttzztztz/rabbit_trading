use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Hash)]
pub enum Currency {
    CNH,
    CNY,
    HKD,
    JPY,
    USD,
}
