use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountLedger {
    #[serde(rename = "commoditymarketvalue")]
    pub commodity_market_value: Decimal,
    #[serde(rename = "futuremarketvalue")]
    pub future_market_value: Decimal,
    #[serde(rename = "settledcash")]
    pub settled_cash: Decimal,
    #[serde(rename = "exchangerate")]
    pub exchange_rate: Decimal,
    #[serde(rename = "sessionid")]
    pub session_id: Decimal,
    #[serde(rename = "cashbalance")]
    pub cash_balance: Decimal,
    #[serde(rename = "corporatebondsmarketvalue")]
    pub corporate_bonds_market_value: Decimal,
    #[serde(rename = "warrantsmarketvalue")]
    pub warrants_market_value: Decimal,
    #[serde(rename = "netliquidationvalue")]
    pub net_liquidation_value: Decimal,
    pub interest: Decimal,
    #[serde(rename = "unrealizedpnl")]
    pub unrealized_pnl: Decimal,
    #[serde(rename = "stockmarketvalue")]
    pub stock_market_value: Decimal,
    #[serde(rename = "moneyfunds")]
    pub money_funds: Decimal,
    pub currency: String,
    #[serde(rename = "realizedpnl")]
    pub realized_pnl: Decimal,
    pub funds: Decimal,
    #[serde(rename = "acctcode")]
    pub acct_code: String,
    #[serde(rename = "issueroptionsmarketvalue")]
    pub issuer_options_market_value: Decimal,
    pub key: String,
    pub timestamp: u64,
    pub severity: Decimal,
}
