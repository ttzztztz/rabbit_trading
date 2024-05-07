use reqwest_retry::policies::ExponentialBackoff;
use serial_test::serial;

use crate::{
    client::IBClientPortal,
    model::{
        definition::TickType,
        market_data::{
            GetMarketDataHistoryBetaRequest, GetMarketDataHistoryRequest, GetMarketDataRequest,
            GetMarketDataSnapshotRequest,
        },
    },
    test::{
        session::once_init_brokerage_session,
        utils::{get_test_account, CONTRACT_ID_AAPL, TEST_HOST},
    },
};

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_market_data() {
    once_init_brokerage_session().await;
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let request = GetMarketDataRequest {
        conid_list: vec![CONTRACT_ID_AAPL],
        since: Option::None,
        fields: Option::Some(vec![
            TickType::LastPrice,
            TickType::Low,
            TickType::High,
            TickType::Open,
            TickType::Change,
            TickType::ChangePct,
            TickType::ChangeSinceOpen,
            TickType::Volume,
            TickType::PriorClose,
            TickType::AskPrice,
            TickType::BidPrice,
            TickType::AskSize,
            TickType::BidSize,
        ]),
    };
    let first_response_result = ib_cp_client.get_market_data(request.clone()).await;
    assert!(first_response_result.is_ok());

    // first response won't return anything
    let second_response_result = ib_cp_client.get_market_data(request).await;

    assert!(second_response_result.is_ok());
    let second_response_result = second_response_result.unwrap();
    assert!(second_response_result.len() > 0);
    let body = &second_response_result[0];
    assert_eq!(Option::Some(CONTRACT_ID_AAPL.to_string()), body.conidex);
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_market_data_history() {
    once_init_brokerage_session().await;
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let response_result = ib_cp_client
        .get_market_data_history(GetMarketDataHistoryRequest {
            conid: CONTRACT_ID_AAPL,
            exchange: Option::None,
            period: "1d".to_owned(),
            bar: Option::Some("15min".to_owned()),
            outside_regular_trading_hours: Option::Some(false),
            start_time: Option::Some("20240101-00:00:00".to_owned()),
        })
        .await;

    assert!(response_result.is_ok());
    let response = response_result.unwrap();
    assert!(response.data.is_some());
    assert!(response.data.unwrap().len() > 0);
}

// todo: test unsubscribe_all_market_data, unsubscribe_market_data

#[tokio::test]
#[serial]
#[cfg_attr(not(feature = "flaky_test_cases"), ignore)]
async fn test_get_market_data_history_beta() {
    once_init_brokerage_session().await;
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let response_result = ib_cp_client
        .get_market_data_history_beta(GetMarketDataHistoryBetaRequest {
            conid: CONTRACT_ID_AAPL,
            period: "30d".to_owned(),
            bar: Option::Some("1d".to_owned()),
            outside_regular_trading_hours: Option::Some(false),
        })
        .await;

    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_market_data_snapshot_beta() {
    once_init_brokerage_session().await;
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let response_result = ib_cp_client
        .get_market_data_snapshot_beta(GetMarketDataSnapshotRequest {
            conid_list: vec![CONTRACT_ID_AAPL],
            field_list: vec![TickType::LastPrice],
        })
        .await;

    assert!(response_result.is_ok());
}

#[tokio::test]
#[serial]
#[cfg_attr(feature = "ci", ignore)]
async fn test_get_market_data_all_tick_types() {
    once_init_brokerage_session().await;
    let ib_cp_client = IBClientPortal::new(
        get_test_account(),
        TEST_HOST.to_owned(),
        false,
        ExponentialBackoff::builder().build_with_max_retries(3),
    );
    let request = GetMarketDataRequest {
        conid_list: vec![CONTRACT_ID_AAPL],
        since: Option::None,
        fields: Option::Some(vec![
            TickType::LastPrice,
            TickType::Symbol,
            TickType::Text,
            TickType::High,
            TickType::Low,
            TickType::MarketValue,
            TickType::AvgPrice,
            TickType::UnrealizedPnL,
            TickType::FormattedPosition,
            TickType::FormattedUnrealizedPnL,
            TickType::DailyPnL,
            TickType::RealizedPnL,
            TickType::UnrealizedPnLPct,
            TickType::Change,
            TickType::ChangePct,
            TickType::BidPrice,
            TickType::AskSize,
            TickType::AskPrice,
            TickType::Volume,
            TickType::BidSize,
            TickType::Exchange,
            TickType::ConId,
            TickType::SecType,
            TickType::Months,
            TickType::RegularExpiry,
            TickType::MarkerForMarketDataDeliveryMethodSimilarToRequestId,
            TickType::UnderlyingConIdUseTrsrvsecdefToGetMoreInformationAboutTheSecurity,
            TickType::ServiceParams,
            TickType::MarketDataAvailability,
            TickType::CompanyName,
            TickType::AskExch,
            TickType::LastExch,
            TickType::LastSize,
            TickType::BidExch,
            TickType::ImpliedVolHistVol,
            TickType::PutCallInterest,
            TickType::PutCallVolume,
            TickType::HistVol,
            TickType::HistVolClose,
            TickType::OptVolume,
            TickType::ConIdExchange,
            TickType::CanBeTraded,
            TickType::IVRank7195,
            TickType::IVRank7196,
            TickType::IVRank7197,
            TickType::IVPercentile7198,
            TickType::IVPercentile7199,
            TickType::IVPercentile7200,
            TickType::IVHighLow7201,
            TickType::IVHighLow7202,
            TickType::IVHighLow7203,
            TickType::IVHighLow7204,
            TickType::IVHighLow7205,
            TickType::IVHighLow7206,
            TickType::HVRank7207,
            TickType::HVRank7208,
            TickType::HVRank7209,
            TickType::HVPercentile7210,
            TickType::HVPercentile7211,
            TickType::HVPercentile7212,
            TickType::ContractDescription7219,
            TickType::ContractDescription7220,
            TickType::ListingExchange,
            TickType::HVHighLow7245,
            TickType::HVHighLow7246,
            TickType::HVHighLow7247,
            TickType::HVHighLow7248,
            TickType::HVHighLow7249,
            TickType::HVHighLow7263,
            TickType::ESG7264,
            TickType::ESG7265,
            TickType::ESG7266,
            TickType::ESG7267,
            TickType::ESG7268,
            TickType::ESG7269,
            TickType::ESG7271,
            TickType::ESG7272,
            TickType::ESG7273,
            TickType::ESG7274,
            TickType::ESG7275,
            TickType::ESG7276,
            TickType::ESG7277,
            TickType::Industry,
            TickType::Category,
            TickType::AverageVolume,
            TickType::OptionImpliedVol,
            TickType::HistoricVolume30d,
            TickType::PutCallRatio,
            TickType::DividendAmount,
            TickType::DividendYield,
            TickType::Ex,
            TickType::MarketCap,
            TickType::PE,
            TickType::EPS,
            TickType::CostBasis,
            TickType::FiftyTwoWeekHigh,
            TickType::FiftyTwoWeekLow,
            TickType::Open,
            TickType::Close,
            TickType::LastPrice,
            TickType::Symbol,
            TickType::Text,
            TickType::High,
            TickType::Low,
            TickType::MarketValue,
            TickType::AvgPrice,
            TickType::UnrealizedPnL,
            TickType::FormattedPosition,
            TickType::FormattedUnrealizedPnL,
            TickType::DailyPnL,
            TickType::RealizedPnL,
            TickType::UnrealizedPnLPct,
            TickType::Change,
            TickType::ChangePct,
            TickType::BidPrice,
            TickType::AskSize,
            TickType::AskPrice,
            TickType::Volume,
            TickType::BidSize,
            TickType::Exchange,
            TickType::ConId,
            TickType::SecType,
            TickType::Months,
            TickType::RegularExpiry,
            TickType::MarkerForMarketDataDeliveryMethodSimilarToRequestId,
            TickType::UnderlyingConIdUseTrsrvsecdefToGetMoreInformationAboutTheSecurity,
            TickType::ServiceParams,
            TickType::MarketDataAvailability,
            TickType::CompanyName,
            TickType::AskExch,
            TickType::LastExch,
            TickType::LastSize,
            TickType::BidExch,
            TickType::ImpliedVolHistVol,
            TickType::PutCallInterest,
            TickType::PutCallVolume,
            TickType::HistVol,
            TickType::HistVolClose,
            TickType::OptVolume,
            TickType::ConIdExchange,
            TickType::CanBeTraded,
            TickType::IVRank7195,
            TickType::IVRank7196,
            TickType::IVRank7197,
            TickType::IVPercentile7198,
            TickType::IVPercentile7199,
            TickType::IVPercentile7200,
            TickType::IVHighLow7201,
            TickType::IVHighLow7202,
            TickType::IVHighLow7203,
            TickType::IVHighLow7204,
            TickType::IVHighLow7205,
            TickType::IVHighLow7206,
            TickType::HVRank7207,
            TickType::HVRank7208,
            TickType::HVRank7209,
            TickType::HVPercentile7210,
            TickType::HVPercentile7211,
            TickType::HVPercentile7212,
            TickType::ContractDescription7219,
            TickType::ContractDescription7220,
            TickType::ListingExchange,
            TickType::HVHighLow7245,
            TickType::HVHighLow7246,
            TickType::HVHighLow7247,
            TickType::HVHighLow7248,
            TickType::HVHighLow7249,
            TickType::HVHighLow7263,
            TickType::ESG7264,
            TickType::ESG7265,
            TickType::ESG7266,
            TickType::ESG7267,
            TickType::ESG7268,
            TickType::ESG7269,
            TickType::ESG7271,
            TickType::ESG7272,
            TickType::ESG7273,
            TickType::ESG7274,
            TickType::ESG7275,
            TickType::ESG7276,
            TickType::ESG7277,
            TickType::Industry,
            TickType::Category,
            TickType::AverageVolume,
            TickType::OptionImpliedVol,
            TickType::HistoricVolume30d,
            TickType::PutCallRatio,
            TickType::DividendAmount,
            TickType::DividendYield,
            TickType::Ex,
            TickType::MarketCap,
            TickType::PE,
            TickType::EPS,
            TickType::CostBasis,
            TickType::FiftyTwoWeekHigh,
            TickType::FiftyTwoWeekLow,
            TickType::Open,
            TickType::Close,
            TickType::Delta,
            TickType::Gamma,
            TickType::Theta,
            TickType::Vega,
            TickType::ReutersFundamentals,
            TickType::ESG7370,
            TickType::ESG7371,
            TickType::ESG7372,
            TickType::OptVolumeChange,
            TickType::ImpliedVol,
            TickType::Mark,
            TickType::ShortableShares,
            TickType::FeeRate,
            TickType::OptionOpenInterest,
            TickType::PctOfMarkValue,
            TickType::Shortable,
            TickType::MorningstarRating,
            TickType::Dividends,
            TickType::DividendsTTM,
            TickType::EMA200,
            TickType::EMA100,
            TickType::EMA50,
            TickType::EMA20,
            TickType::PriceEMA200,
            TickType::PriceEMA100,
            TickType::PriceEMA50,
            TickType::PriceEMA20,
            TickType::ChangeSinceOpen,
            TickType::UpcomingEvent,
            TickType::UpcomingEventDate,
            TickType::UpcomingAnalystMeeting,
            TickType::UpcomingEarnings,
            TickType::UpcomingMiscEvent,
            TickType::RecentAnalystMeeting,
            TickType::RecentEarnings,
            TickType::RecentMiscEvent,
            TickType::ProbabilityOfMaxReturn,
            TickType::BreakEven,
            TickType::SPXDelta,
            TickType::FuturesOpenInterest,
            TickType::LastYield,
            TickType::BidYield,
            TickType::ProbabilityOfMaxReturn7700,
            TickType::ProbabilityOfMaxLoss,
            TickType::ProfitProbability,
            TickType::OrganizationType,
            TickType::DebtClass,
            TickType::Ratings,
            TickType::BondStateCode,
            TickType::BondType,
            TickType::LastTradingDate,
            TickType::IssueDate,
            TickType::Beta,
            TickType::AskYield,
            TickType::PriorClose,
            TickType::ReutersFundamentals7743,
            TickType::ESG,
            TickType::VolumeLong,
            TickType::HasTradingPermissions,
            TickType::_26WeekHigh,
            TickType::_26WeekLow,
            TickType::_13WeekHigh,
            TickType::_13WeekLow,
            TickType::Vega,
            TickType::ReutersFundamentals,
            TickType::ESG7370,
            TickType::ESG7371,
            TickType::ESG7372,
            TickType::OptVolumeChange,
            TickType::ImpliedVol,
            TickType::Mark,
            TickType::ShortableShares,
            TickType::FeeRate,
            TickType::OptionOpenInterest,
            TickType::Shortable,
            TickType::MorningstarRating,
            TickType::Dividends,
            TickType::DividendsTTM,
            TickType::EMA200,
            TickType::EMA100,
            TickType::EMA50,
            TickType::EMA20,
            TickType::PriceEMA200,
            TickType::PriceEMA100,
            TickType::PriceEMA50,
            TickType::PriceEMA20,
            TickType::ChangeSinceOpen,
            TickType::UpcomingEvent,
            TickType::UpcomingEventDate,
            TickType::UpcomingAnalystMeeting,
            TickType::UpcomingEarnings,
            TickType::UpcomingMiscEvent,
            TickType::RecentAnalystMeeting,
            TickType::RecentEarnings,
            TickType::RecentMiscEvent,
            TickType::ProbabilityOfMaxReturn,
            TickType::BreakEven,
            TickType::SPXDelta,
            TickType::FuturesOpenInterest,
            TickType::LastYield,
            TickType::BidYield,
            TickType::ProbabilityOfMaxReturn7700,
            TickType::ProbabilityOfMaxLoss,
            TickType::ProfitProbability,
            TickType::OrganizationType,
            TickType::DebtClass,
            TickType::Ratings,
            TickType::BondStateCode,
            TickType::BondType,
            TickType::LastTradingDate,
            TickType::IssueDate,
            TickType::Beta,
            TickType::AskYield,
            TickType::PriorClose,
            TickType::ReutersFundamentals7743,
            TickType::ESG,
            TickType::VolumeLong,
            TickType::HasTradingPermissions,
            TickType::_26WeekHigh,
            TickType::_26WeekLow,
            TickType::_13WeekHigh,
            TickType::_13WeekLow,
        ]),
    };
    let first_response_result = ib_cp_client.get_market_data(request.clone()).await;
    assert!(first_response_result.is_ok());

    // first response won't return anything
    let second_response_result = ib_cp_client.get_market_data(request).await;

    assert!(second_response_result.is_ok());
    let second_response_result = second_response_result.unwrap();
    assert!(second_response_result.len() > 0);
    let body = &second_response_result[0];
    assert_eq!(Option::Some(CONTRACT_ID_AAPL.to_string()), body.conidex);
}
