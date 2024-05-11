use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use super::session::ServerInfo;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
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
    pub session_id: Option<String>,
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
    pub severity: Option<i64>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccountProperties {
    /// Returns whether or not child accounts exist for the account.
    #[serde(rename = "hasChildAccounts")]
    pub has_child_accounts: Option<bool>,
    /// Returns whether or not the account can use Cash Quantity for trading.
    #[serde(rename = "supportsCashQty")]
    pub supports_cash_quantity: Option<bool>,
    #[serde(rename = "liteUnderPro")]
    pub lite_under_pro: Option<bool>,
    #[serde(rename = "noFXConv")]
    pub no_fx_conversion_fee: Option<bool>,
    #[serde(rename = "isProp")]
    pub is_prop: Option<bool>,
    /// Returns whether or not the account can submit fractional share orders.
    #[serde(rename = "supportsFractions")]
    pub supports_fractions: Option<bool>,
    #[serde(rename = "allowCustomerTime")]
    pub allow_customer_time: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccountAllowFeatures {
    /// Returns if the account can access market data.
    #[serde(rename = "showGFIS")]
    pub show_gfis: Option<bool>,
    /// Returns if the account can view the EU Cost Report
    #[serde(rename = "showEUCostReport")]
    pub show_eu_cost_report: Option<bool>,
    #[serde(rename = "allowEventContract")]
    pub allow_event_contract: Option<bool>,
    /// Returns if the account can convert currencies.
    #[serde(rename = "allowFXConv")]
    pub allow_fx_conversion: Option<bool>,
    /// Returns if the account can access the financial lens.
    #[serde(rename = "allowFinancialLens")]
    pub allow_financial_lens: Option<bool>,
    /// Returns if the account can use mobile trading alerts.
    #[serde(rename = "allowMTA")]
    pub allow_mobile_trading_alerts: Option<bool>,
    /// Returns if the account can use Type-Ahead support for Client Portal.
    #[serde(rename = "allowTypeAhead")]
    pub allow_type_ahead: Option<bool>,
    /// Returns if the account can use Event Trader.
    #[serde(rename = "allowEventTrading")]
    pub allow_event_trading: Option<bool>,
    /// Returns the snapshot refresh timeout window for new data.
    #[serde(rename = "snapshotRefreshTimeout")]
    pub snapshot_refresh_timeout: i64,
    /// Returns if the account is an IBKR Lite user.
    #[serde(rename = "liteUser")]
    pub lite_user: Option<bool>,
    /// Returns if the account can use News feeds via the web.
    #[serde(rename = "showWebNews")]
    pub show_web_news: Option<bool>,
    pub research: Option<bool>,
    /// Returns if the account can use the debugPnl endpoint.
    #[serde(rename = "debugPnl")]
    pub debug_pnl: Option<bool>,
    /// Returns if the account can use the Tax Optimizer tool
    #[serde(rename = "showTaxOpt")]
    pub show_tax_optimizer_tool: Option<bool>,
    /// Returns if the account can view the Impact Dashboard.
    #[serde(rename = "showImpactDashboard")]
    pub show_impact_dashboard: Option<bool>,
    /// Returns if the account can use dynamic account changes.
    #[serde(rename = "allowDynAccount")]
    pub allow_dynamic_account: Option<bool>,
    /// Returns if the account can trade crypto currencies.
    #[serde(rename = "allowCrypto")]
    pub allow_crypto: Option<bool>,
    #[serde(rename = "allowFA")]
    pub allow_fa: Option<bool>,
    #[serde(rename = "allowLiteUnderPro")]
    pub allow_lite_under_pro: Option<bool>,
    /// Returns a list of asset types the account can trade.
    #[serde(rename = "allowedAssetTypes")]
    pub allowed_asset_types: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetAccountsResponse {
    /// Unique account id
    pub accounts: Vec<String>,
    /// Account Id and its alias
    pub aliases: HashMap<String, String>,
    /// Returns an json object for each accessible account’s properties.
    #[serde(rename = "acctProps")]
    pub account_properties: Option<HashMap<String, AccountProperties>>,
    /// JSON of allowed features for the account.
    #[serde(rename = "allowFeatures")]
    pub allow_features: Option<AccountAllowFeatures>,
    /// Returns available trading times for all available security types.
    #[serde(rename = "chartPeriods")]
    pub chart_periods: Option<FinancialDerivatives<Vec<String>>>,
    /// Returns an array of affiliated groups.
    pub groups: Option<Vec<String>>,
    /// Returns an array of affiliated profiles.
    pub profiles: Option<Vec<String>>,
    /// Returns currently selected account. See Switch Account for more details.
    #[serde(rename = "selectedAccount")]
    pub selected_account: String,
    /// Returns information about the IBKR session. Unrelated to Client Portal Gateway.
    #[serde(rename = "serverInfo")]
    pub server_info: Option<ServerInfo>,
    /// Returns current session ID.
    #[serde(rename = "sessionId")]
    pub session_id: Option<String>,
    /// Returns fractional trading access.
    #[serde(rename = "isFT")]
    pub is_fractional_trading: Option<bool>,
    /// Returns account type status.
    #[serde(rename = "isPaper")]
    pub is_paper: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SwitchAccountRequest {
    #[serde(rename = "acctId")]
    pub account_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SwitchAccountResponse {
    pub set: bool,
    #[serde(rename = "acctId")]
    pub account_id: String,
}

pub type GetAccountLedgerResponse = HashMap<String, AccountLedger>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccountParent {
    #[serde(rename = "mmc")]
    mmc: Option<Vec<String>>,
    /// Account Number for Money Manager Client
    #[serde(rename = "accountId")]
    account_id: Option<String>,
    /// Is MM a Parent Account
    #[serde(rename = "isMParent")]
    is_m_parent: Option<bool>,
    /// Is MM a Child Account
    #[serde(rename = "isMChild")]
    is_m_child: Option<bool>,
    /// Is a Multiplex Account. These are account models with individual account being parent and managed account being child.
    #[serde(rename = "isMultiplex")]
    is_multiplex: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Account {
    /// The account identification value
    #[serde(rename = "id")]
    id: Option<String>,
    /// The account number
    #[serde(rename = "accountId")]
    account_id: Option<String>,
    /// The accountAlias
    #[serde(rename = "accountVan")]
    account_van: Option<String>,
    /// Title of the account
    #[serde(rename = "accountTitle")]
    account_title: Option<String>,
    /// Whichever value is not null in this priority
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    /// User customizable account alias. Refer to [Configure Account Alias](https://guides.interactivebrokers.com/cp/cp.htm#am/settings/accountalias.htm) for details.
    #[serde(rename = "accountAlias")]
    account_alias: Option<String>,
    /// When the account was opened in unix time.
    #[serde(rename = "accountStatus")]
    account_status: Option<i64>,
    /// Base currency of the account.
    #[serde(rename = "currency")]
    currency: Option<String>,
    /// Account Type
    #[serde(rename = "type")]
    _type: Option<String>,
    /// UNI - Deprecated property
    #[serde(rename = "tradingType")]
    trading_type: Option<String>,
    /// If an account is a sub-account to a Financial Advisor.
    #[serde(rename = "faclient")]
    faclient: Option<bool>,
    /// Status of the Account   * O = Open   * P or N = Pending   * A = Abandoned   * R = Rejected   * C = Closed
    #[serde(rename = "clearingStatus")]
    clearing_status: Option<String>,
    /// Is a Covestor Account
    #[serde(rename = "covestor")]
    covestor: Option<bool>,
    #[serde(rename = "parent")]
    parent: Option<AccountParent>,
    /// Formatted \"accountId - accountAlias\"
    #[serde(rename = "desc")]
    desc: Option<String>,
}

pub type GetPortfolioAccountsResponse = Vec<Account>;
pub type GetAccountMetadataResponse = Account;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetSubAccountsV2Request {
    pub page: i32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetAccountMetadataRequest {
    pub account_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetAccountSummaryRequest {
    pub account_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetAccountAllocationRequest {
    pub account_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SubAccount {
    /// The account identification value
    #[serde(rename = "id")]
    pub id: Option<String>,
    /// The account number
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    /// The accountAlias
    #[serde(rename = "accountVan")]
    pub account_van: Option<String>,
    /// Title of the account
    #[serde(rename = "accountTitle")]
    pub account_title: Option<String>,
    /// Whichever value is not null in this priority
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    /// User customizable account alias. Refer to [Configure Account Alias](https://guides.interactivebrokers.com/cp/cp.htm#am/settings/accountalias.htm) for details.
    #[serde(rename = "accountAlias")]
    pub account_alias: Option<String>,
    /// When the account was opened in unix time.
    #[serde(rename = "accountStatus")]
    pub account_status: Option<i64>,
    /// Base currency of the account.
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    /// Account Type
    #[serde(rename = "type")]
    pub _type: Option<String>,
    /// UNI - Deprecated property
    #[serde(rename = "tradingType")]
    pub trading_type: Option<String>,
    /// If an account is a sub-account to a Financial Advisor.
    #[serde(rename = "faclient")]
    pub financial_advisor_client: Option<bool>,
    /// Status of the Account   * O = Open   * P or N = Pending   * A = Abandoned   * R = Rejected   * C = Closed   covestor:     type: boolean     description: Is a Covestor Account   parent:     type: object     properties:       mmc:         type: array         items:           type: string           description: Money Manager Client (MMC) Account       accountId:         type: string         description: Account Number for Money Manager Client       isMParent:         type: boolean         description: Is MM a Parent Account       isMChild:         type: boolean         description: Is MM a Child Account       isMultiplex:         type: boolean         description: Is a Multiplex Account. These are account models with individual account being parent and managed account being child.   desc:     type: string     description: Formatted \"accountId - accountAlias\"
    #[serde(rename = "clearingStatus")]
    pub clearing_status: Option<String>,
}

pub type GetSubAccountsResponse = Vec<Account>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetSubAccountsV2Response {
    #[serde(rename = "metadata")]
    pub metadata: Option<SubAccount>,
    #[serde(rename = "subaccounts")]
    pub sub_accounts: Option<Vec<Account>>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Summary {
    #[serde(rename = "amount")]
    pub amount: Option<Decimal>,
    #[serde(rename = "currency")]
    pub currency: Option<String>,
    #[serde(rename = "isNull")]
    pub is_null: Option<bool>,
    #[serde(rename = "timestamp")]
    pub timestamp: Option<i64>,
    #[serde(rename = "value")]
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GetAccountSummaryResponse {
    #[serde(rename = "accountready")]
    pub account_ready: Option<Summary>,
    #[serde(rename = "accounttype")]
    pub account_type: Option<Summary>,
    #[serde(rename = "accruedcash")]
    pub accrued_cash: Option<Summary>,
    #[serde(rename = "accruedcash-c")]
    pub accrued_cash_c: Option<Summary>,
    #[serde(rename = "accruedcash-f")]
    pub accrued_cash_f: Option<Summary>,
    #[serde(rename = "accruedcash-s")]
    pub accrued_cash_s: Option<Summary>,
    #[serde(rename = "accrueddividend")]
    pub accrued_dividend: Option<Summary>,
    #[serde(rename = "accrueddividend-c")]
    pub accrued_dividend_c: Option<Summary>,
    #[serde(rename = "accrueddividend-f")]
    pub accrued_dividend_f: Option<Summary>,
    #[serde(rename = "accrueddividend-s")]
    pub accrued_dividend_s: Option<Summary>,
    #[serde(rename = "availablefunds")]
    pub available_funds: Option<Summary>,
    #[serde(rename = "availablefunds-c")]
    pub available_funds_c: Option<Summary>,
    #[serde(rename = "availablefunds-f")]
    pub available_funds_f: Option<Summary>,
    #[serde(rename = "availablefunds-s")]
    pub available_funds_s: Option<Summary>,
    #[serde(rename = "billable")]
    pub billable: Option<Summary>,
    #[serde(rename = "billable-c")]
    pub billable_c: Option<Summary>,
    #[serde(rename = "billable-f")]
    pub billable_f: Option<Summary>,
    #[serde(rename = "billable-s")]
    pub billable_s: Option<Summary>,
    #[serde(rename = "buyingpower")]
    pub buying_power: Option<Summary>,
    #[serde(rename = "cushion")]
    pub cushion: Option<Summary>,
    #[serde(rename = "daytradesremaining")]
    pub day_trades_remaining: Option<Summary>,
    #[serde(rename = "daytradesremainingt+1")]
    pub day_trades_remaining_t1: Option<Summary>,
    #[serde(rename = "daytradesremainingt+2")]
    pub day_trades_remaining_t2: Option<Summary>,
    #[serde(rename = "daytradesremainingt+3")]
    pub day_trades_remaining_t3: Option<Summary>,
    #[serde(rename = "daytradesremainingt+4")]
    pub day_trades_remaining_t4: Option<Summary>,
    #[serde(rename = "equitywithloanvalue")]
    pub equity_with_loan_value: Option<Summary>,
    #[serde(rename = "equitywithloanvalue-c")]
    pub equity_with_loan_value_c: Option<Summary>,
    #[serde(rename = "equitywithloanvalue-f")]
    pub equity_with_loan_value_f: Option<Summary>,
    #[serde(rename = "equitywithloanvalue-s")]
    pub equity_with_loan_value_s: Option<Summary>,
    #[serde(rename = "excessliquidity")]
    pub excess_liquidity: Option<Summary>,
    #[serde(rename = "excessliquidity-c")]
    pub excess_liquidity_c: Option<Summary>,
    #[serde(rename = "excessliquidity-f")]
    pub excess_liquidity_f: Option<Summary>,
    #[serde(rename = "excessliquidity-s")]
    pub excess_liquidity_s: Option<Summary>,
    #[serde(rename = "fullavailablefunds")]
    pub full_available_funds: Option<Summary>,
    #[serde(rename = "fullavailablefunds-c")]
    pub full_available_funds_c: Option<Summary>,
    #[serde(rename = "fullavailablefunds-f")]
    pub full_available_funds_f: Option<Summary>,
    #[serde(rename = "fullavailablefunds-s")]
    pub full_available_funds_s: Option<Summary>,
    #[serde(rename = "fullexcessliquidity")]
    pub full_excess_liquidity: Option<Summary>,
    #[serde(rename = "fullexcessliquidity-c")]
    pub full_excess_liquidity_c: Option<Summary>,
    #[serde(rename = "fullexcessliquidity-f")]
    pub full_excess_liquidity_f: Option<Summary>,
    #[serde(rename = "fullexcessliquidity-s")]
    pub full_excess_liquidity_s: Option<Summary>,
    #[serde(rename = "fullinitmarginreq")]
    pub full_init_margin_req: Option<Summary>,
    #[serde(rename = "fullinitmarginreq-c")]
    pub full_init_margin_req_c: Option<Summary>,
    #[serde(rename = "fullinitmarginreq-f")]
    pub full_init_margin_req_f: Option<Summary>,
    #[serde(rename = "fullinitmarginreq-s")]
    pub full_init_margin_req_s: Option<Summary>,
    #[serde(rename = "fullmaintmarginreq")]
    pub full_maintenance_margin_req: Option<Summary>,
    #[serde(rename = "fullmaintmarginreq-c")]
    pub full_maintenance_margin_req_c: Option<Summary>,
    #[serde(rename = "fullmaintmarginreq-f")]
    pub full_maintenance_margin_req_f: Option<Summary>,
    #[serde(rename = "fullmaintmarginreq-s")]
    pub full_maintenance_margin_req_s: Option<Summary>,
    #[serde(rename = "grosspositionvalue")]
    pub gross_position_value: Option<Summary>,
    #[serde(rename = "grosspositionvalue-c")]
    pub gross_position_value_c: Option<Summary>,
    #[serde(rename = "grosspositionvalue-f")]
    pub gross_position_value_f: Option<Summary>,
    #[serde(rename = "grosspositionvalue-s")]
    pub gross_position_value_s: Option<Summary>,
    #[serde(rename = "guarantee")]
    pub guarantee: Option<Summary>,
    #[serde(rename = "guarantee-c")]
    pub guarantee_c: Option<Summary>,
    #[serde(rename = "guarantee-f")]
    pub guarantee_f: Option<Summary>,
    #[serde(rename = "guarantee-s")]
    pub guarantee_s: Option<Summary>,
    #[serde(rename = "highestseverity")]
    pub highest_severity: Option<Summary>,
    #[serde(rename = "highestseverity-c")]
    pub highest_severity_c: Option<Summary>,
    #[serde(rename = "highestseverity-f")]
    pub highest_severity_f: Option<Summary>,
    #[serde(rename = "highestseverity-s")]
    pub highest_severity_s: Option<Summary>,
    #[serde(rename = "indianstockhaircut")]
    pub indian_stock_haircut: Option<Summary>,
    #[serde(rename = "indianstockhaircut-c")]
    pub indian_stock_haircut_c: Option<Summary>,
    #[serde(rename = "indianstockhaircut-f")]
    pub indian_stock_haircut_f: Option<Summary>,
    #[serde(rename = "indianstockhaircut-s")]
    pub indian_stock_haircut_s: Option<Summary>,
    #[serde(rename = "initmarginreq")]
    pub init_margin_req: Option<Summary>,
    #[serde(rename = "initmarginreq-c")]
    pub init_margin_req_c: Option<Summary>,
    #[serde(rename = "initmarginreq-f")]
    pub init_margin_req_f: Option<Summary>,
    #[serde(rename = "initmarginreq-s")]
    pub init_margin_req_s: Option<Summary>,
    #[serde(rename = "leverage")]
    pub leverage: Option<Summary>,
    #[serde(rename = "leverage-c")]
    pub leverage_c: Option<Summary>,
    #[serde(rename = "leverage-f")]
    pub leverage_f: Option<Summary>,
    #[serde(rename = "leverage-s")]
    pub leverage_s: Option<Summary>,
    #[serde(rename = "lookaheadavailablefunds")]
    pub look_ahead_available_funds: Option<Summary>,
    #[serde(rename = "lookaheadavailablefunds-c")]
    pub look_ahead_available_funds_c: Option<Summary>,
    #[serde(rename = "lookaheadavailablefunds-f")]
    pub look_ahead_available_funds_f: Option<Summary>,
    #[serde(rename = "lookaheadavailablefunds-s")]
    pub look_ahead_available_funds_s: Option<Summary>,
    #[serde(rename = "lookaheadexcessliquidity")]
    pub look_ahead_excess_liquidity: Option<Summary>,
    #[serde(rename = "lookaheadexcessliquidity-c")]
    pub look_ahead_excess_liquidity_c: Option<Summary>,
    #[serde(rename = "lookaheadexcessliquidity-f")]
    pub look_ahead_excess_liquidity_f: Option<Summary>,
    #[serde(rename = "lookaheadexcessliquidity-s")]
    pub look_ahead_excess_liquidity_s: Option<Summary>,
    #[serde(rename = "lookaheadinitmarginreq")]
    pub look_ahead_init_marginreq: Option<Summary>,
    #[serde(rename = "lookaheadinitmarginreq-c")]
    pub look_ahead_init_marginreq_c: Option<Summary>,
    #[serde(rename = "lookaheadinitmarginreq-f")]
    pub look_ahead_init_marginreq_f: Option<Summary>,
    #[serde(rename = "lookaheadinitmarginreq-s")]
    pub look_ahead_init_marginreq_s: Option<Summary>,
    #[serde(rename = "lookaheadmaintmarginreq")]
    pub look_ahead_maintenance_marginreq: Option<Summary>,
    #[serde(rename = "lookaheadmaintmarginreq-c")]
    pub look_ahead_maintenance_marginreq_c: Option<Summary>,
    #[serde(rename = "lookaheadmaintmarginreq-f")]
    pub look_ahead_maintenance_margin_req_f: Option<Summary>,
    #[serde(rename = "lookaheadmaintmarginreq-s")]
    pub look_ahead_maintenance_margin_req_s: Option<Summary>,
    #[serde(rename = "lookaheadnextchange")]
    pub look_ahead_next_change: Option<Summary>,
    #[serde(rename = "maintmarginreq")]
    pub maintenance_marginreq: Option<Summary>,
    #[serde(rename = "maintmarginreq-c")]
    pub maintenance_marginreq_c: Option<Summary>,
    #[serde(rename = "maintmarginreq-f")]
    pub maintenance_marginreq_f: Option<Summary>,
    #[serde(rename = "maintmarginreq-s")]
    pub maintenance_marginreq_s: Option<Summary>,
    #[serde(rename = "netliquidation")]
    pub net_liquidation: Option<Summary>,
    #[serde(rename = "netliquidation-c")]
    pub net_liquidation_c: Option<Summary>,
    #[serde(rename = "netliquidation-f")]
    pub net_liquidation_f: Option<Summary>,
    #[serde(rename = "netliquidation-s")]
    pub net_liquidation_s: Option<Summary>,
    #[serde(rename = "netliquidationuncertainty")]
    pub net_liquidation_uncertainty: Option<Summary>,
    #[serde(rename = "nlvandmargininreview")]
    pub nlvandmargininreview: Option<Summary>,
    #[serde(rename = "pasharesvalue")]
    pub pashares_value: Option<Summary>,
    #[serde(rename = "pasharesvalue-c")]
    pub pashares_value_c: Option<Summary>,
    #[serde(rename = "pasharesvalue-f")]
    pub pashares_value_f: Option<Summary>,
    #[serde(rename = "pasharesvalue-s")]
    pub pashares_value_s: Option<Summary>,
    #[serde(rename = "postexpirationexcess")]
    pub post_expiration_excess: Option<Summary>,
    #[serde(rename = "postexpirationexcess-c")]
    pub post_expiration_excess_c: Option<Summary>,
    #[serde(rename = "postexpirationexcess-f")]
    pub post_expiration_excess_f: Option<Summary>,
    #[serde(rename = "postexpirationexcess-s")]
    pub post_expiration_excess_s: Option<Summary>,
    #[serde(rename = "postexpirationmargin")]
    pub post_expiration_margin: Option<Summary>,
    #[serde(rename = "postexpirationmargin-c")]
    pub post_expiration_margin_c: Option<Summary>,
    #[serde(rename = "postexpirationmargin-f")]
    pub post_expiration_margin_f: Option<Summary>,
    #[serde(rename = "postexpirationmargin-s")]
    pub post_expiration_margin_s: Option<Summary>,
    #[serde(rename = "previousdayequitywithloanvalue")]
    pub previous_day_equity_with_loan_value: Option<Summary>,
    #[serde(rename = "previousdayequitywithloanvalue-c")]
    pub previous_day_equity_with_loan_value_c: Option<Summary>,
    #[serde(rename = "previousdayequitywithloanvalue-f")]
    pub previous_day_equity_with_loan_value_f: Option<Summary>,
    #[serde(rename = "previousdayequitywithloanvalue-s")]
    pub previous_day_equity_with_loan_value_s: Option<Summary>,
    #[serde(rename = "segmenttitle-c")]
    pub segment_title_c: Option<Summary>,
    #[serde(rename = "segmenttitle-f")]
    pub segment_title_f: Option<Summary>,
    #[serde(rename = "segmenttitle-s")]
    pub segment_title_s: Option<Summary>,
    #[serde(rename = "totalcashvalue")]
    pub total_cashvalue: Option<Summary>,
    #[serde(rename = "totalcashvalue-c")]
    pub total_cashvalue_c: Option<Summary>,
    #[serde(rename = "totalcashvalue-f")]
    pub total_cashvalue_f: Option<Summary>,
    #[serde(rename = "totalcashvalue-s")]
    pub total_cashvalue_s: Option<Summary>,
    #[serde(rename = "totaldebitcardpendingcharges")]
    pub total_debit_card_pending_charges: Option<Summary>,
    #[serde(rename = "totaldebitcardpendingcharges-c")]
    pub total_debit_card_pending_charges_c: Option<Summary>,
    #[serde(rename = "totaldebitcardpendingcharges-f")]
    pub total_debit_card_pending_charges_f: Option<Summary>,
    #[serde(rename = "totaldebitcardpendingcharges-s")]
    pub total_debit_card_pending_charges_s: Option<Summary>,
    #[serde(rename = "tradingtype-f")]
    pub trading_type_f: Option<Summary>,
    #[serde(rename = "tradingtype-s")]
    pub trading_type_s: Option<Summary>,
}

pub type AllocationAssetClassLong = FinancialDerivatives<Decimal>;
pub type AllocationAssetClassShort = FinancialDerivatives<Decimal>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FinancialDerivatives<T> {
    #[serde(rename = "STK")]
    pub stock: Option<T>,
    #[serde(rename = "CFD")]
    pub cfd: Option<T>,
    #[serde(rename = "OPT")]
    pub options: Option<T>,
    #[serde(rename = "FOP")]
    pub fop: Option<T>,
    #[serde(rename = "WAR")]
    pub warrants: Option<T>,
    #[serde(rename = "FUT")]
    pub futures: Option<T>,
    #[serde(rename = "CASH")]
    pub cash: Option<T>,
    #[serde(rename = "IND")]
    pub ind: Option<T>,
    #[serde(rename = "BOND")]
    pub bonds: Option<T>,
    #[serde(rename = "FUND")]
    pub fund: Option<T>,
    #[serde(rename = "CMDTY")]
    pub cmdty: Option<T>,
    #[serde(rename = "PHYSS")]
    pub physs: Option<T>,
    #[serde(rename = "CRYPTO")]
    pub crypto: Option<T>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AllocationAssetClass {
    #[serde(rename = "long")]
    pub long: Option<AllocationAssetClassLong>,
    #[serde(rename = "short")]
    pub short: Option<AllocationAssetClassShort>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AllocationSectorLong {
    #[serde(rename = "Others")]
    pub others: Option<Decimal>,
    #[serde(rename = "Utilities")]
    pub utilities: Option<Decimal>,
    #[serde(rename = "Energy")]
    pub energy: Option<Decimal>,
    #[serde(rename = "Technology")]
    pub technology: Option<Decimal>,
    #[serde(rename = "Financial")]
    pub financial: Option<Decimal>,
    #[serde(rename = "Communications")]
    pub communications: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AllocationSectorShort {
    #[serde(rename = "Industrial")]
    pub industrial: Option<Decimal>,
    #[serde(rename = "Consumer")]
    pub consumer: Option<Decimal>,
    #[serde(rename = "Diversified")]
    pub diversified: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AllocationSector {
    #[serde(rename = "long")]
    pub long: Option<AllocationSectorLong>,
    #[serde(rename = "short")]
    pub short: Option<AllocationSectorShort>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AllocationGroupLong {
    #[serde(rename = "Computers")]
    pub computers: Option<Decimal>,
    #[serde(rename = "Semiconductors")]
    pub semiconductors: Option<Decimal>,
    #[serde(rename = "Others")]
    pub others: Option<Decimal>,
    #[serde(rename = "Chemicals")]
    pub chemicals: Option<Decimal>,
    #[serde(rename = "Apparel")]
    pub apparel: Option<Decimal>,
    #[serde(rename = "Communications")]
    pub communications: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AllocationInnerGroupShort {
    #[serde(rename = "Banks")]
    pub banks: Option<Decimal>,
    #[serde(rename = "Airlines")]
    pub airlines: Option<Decimal>,
    #[serde(rename = "Internet")]
    pub internet: Option<Decimal>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AllocationGroup {
    #[serde(rename = "long")]
    pub long: Option<AllocationGroupLong>,
    #[serde(rename = "short")]
    pub short: Option<AllocationInnerGroupShort>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Allocation {
    #[serde(rename = "assetClass")]
    pub asset_class: Option<AllocationAssetClass>,
    #[serde(rename = "sector")]
    pub sector: Option<AllocationSector>,
    #[serde(rename = "group")]
    pub group: Option<AllocationGroup>,
}

// todo: add data definitions for this response -- missing in the official docs
pub type GetAccountPnLPartitionedResponse = HashMap<String, Value>;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct AccountTrade {
    /// execution identifier for the order
    #[serde(rename = "execution_id")]
    pub execution_id: Option<String>,
    /// Underlying Symbol
    #[serde(rename = "symbol")]
    pub symbol: Option<String>,
    /// The side of the market of the order.   * B - Buy contract near posted ask price   * S - Sell contract near posted bid price   * X - Option expired
    #[serde(rename = "side")]
    pub side: Option<String>,
    /// Formatted description of the order \"%side% %size% @ %price% on %exchange%\".
    #[serde(rename = "order_description")]
    pub order_description: Option<String>,
    /// Time of Status update in format \"YYYYMMDD-hh:mm:ss\".
    #[serde(rename = "trade_time")]
    pub trade_time: Option<String>,
    /// Time of status update in format unix time.
    #[serde(rename = "trade_time_r")]
    pub trade_time_r: Option<i64>,
    /// Quantity of the order
    #[serde(rename = "size")]
    pub size: Option<String>,
    /// Average Price
    #[serde(rename = "price")]
    pub price: Option<String>,
    /// User defined string used to identify the order. Value is set using \"cOID\" field while placing an order.
    #[serde(rename = "order_ref")]
    pub order_ref: Option<String>,
    /// User that submitted order
    #[serde(rename = "submitter")]
    pub submitter: Option<String>,
    /// Exchange or venue of order
    #[serde(rename = "exchange")]
    pub exchange: Option<String>,
    /// Commission of the order
    #[serde(rename = "commission")]
    pub commission: Option<Decimal>,
    /// Net cost of the order, including contract multiplier and quantity.
    #[serde(rename = "net_amount")]
    pub net_amount: Option<Decimal>,
    /// accountCode
    #[serde(rename = "account")]
    pub account: Option<String>,
    /// Account Number
    #[serde(rename = "acountCode")]
    pub acount_code: Option<String>,
    /// Contracts company name
    #[serde(rename = "company_name")]
    pub company_name: Option<String>,
    /// Format contract name
    #[serde(rename = "contract_description_1")]
    pub contract_description_1: Option<String>,
    /// Asset class
    #[serde(rename = "sec_type")]
    pub sec_type: Option<String>,
    /// IBKR's contract identifier
    #[serde(rename = "conid")]
    pub conid: Option<i64>,
    /// conid and exchange. Format supports conid or conid@exchange
    #[serde(rename = "conidex")]
    pub conidex: Option<String>,
    /// Total quantity owned for this contract
    #[serde(rename = "position")]
    pub position: Option<String>,
    /// Firm which will settle the trade. For IBExecution customers only.
    #[serde(rename = "clearing_id")]
    pub clearing_id: Option<String>,
    /// Specifies the true beneficiary of the order. For IBExecution customers only.
    #[serde(rename = "clearing_name")]
    pub clearing_name: Option<String>,
    /// If order adds liquidity to the market.
    #[serde(rename = "liquidation_trade")]
    pub liquidation_trade: Option<i64>,
}

pub type GetAccountTradesResponse = Vec<AccountTrade>;
