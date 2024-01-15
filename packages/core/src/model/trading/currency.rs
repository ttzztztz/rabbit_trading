use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Currency {
    CNH,
    CNY,
    HKD,
    JPY,
    USD,
}
