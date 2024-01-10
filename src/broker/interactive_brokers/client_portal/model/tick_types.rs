use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u32)]
pub enum TickType {
    ///Last Price - The last price at which the contract traded. May contain one of the following prefixes:
    ///  * C - Previous day's closing price.
    ///  * H - Trading has halted.
    LastPrice = 31,
    ///Symbol
    Symbol = 55,
    ///Text
    Text = 58,
    ///High - Current day high price
    High = 70,
    ///Low - Current day low price
    Low = 71,
    ///Market Value - The current market value of  your position in the security. Market Value is calculated with real time market data (even when not subscribed to market data).
    MarketValue = 73,
    ///Avg Price - The average price of the position.
    AvgPrice = 74,
    ///Unrealized PnL - Unrealized profit or loss. Unrealized PnL is calculated with real time market data (even when not subscribed to market data).
    UnrealizedPnL = 75,
    ///Formatted position
    Formattedposition = 76,
    ///Formatted Unrealized PnL
    FormattedUnrealizedPnL = 77,
    ///Daily PnL - Your profit or loss of the day since prior close. Daily PnL is calculated with real time market data (even when not subscribed to market data).
    DailyPnL = 78,
    ///Realized PnL - Realized profit or loss. Realized PnL is calculated with real time market data (even when not subscribed to market data).
    RealizedPnL = 79,
    ///Unrealized PnL % - Unrealized profit or loss expressed in percentage.
    UnrealizedPnLPct = 80,
    ///Change - The difference between the last price and the close on the previous trading day
    Change = 82,
    ///Change % - The difference between the last price and the close on the previous trading day in percentage.
    ChangePct = 83,
    ///Bid Price - The highest-priced bid on the contract.
    BidPrice = 84,
    ///Ask Size - The number of contracts or shares offered at the ask price. For US stocks, the number displayed is divided by 100.
    AskSize = 85,
    ///Ask Price - The lowest-priced offer on the contract.
    AskPrice = 86,
    ///Volume - Volume for the day, formatted with 'K' for thousands or 'M' for millions. For higher precision volume refer to field 7762.
    Volume = 87,
    ///Bid Size - The number of contracts or shares bid for at the bid price. For US stocks, the number displayed is divided by 100.
    BidSize = 88,
    ///Exchange
    Exchange = 6004,
    ///Conid - Contract identifier from IBKR's database.
    Conid = 6008,
    ///SecType - The asset class of the instrument.
    SecType = 6070,
    ///Months
    Months = 6072,
    ///Regular Expiry
    RegularExpiry = 6073,
    ///Marker for market data delivery method (similar to request id)
    Markerformarketdatadeliverymethodsimilartorequestid = 6119,
    ///Underlying Conid. Use /trsrv/secdef to get more information about the security
    UnderlyingConidUsetrsrvsecdeftogetmoreinformationaboutthesecurity = 6457,
    ///Service Params.
    ServiceParams = 6508,
    ///Market Data Availability. The field may contain three chars. First char defines: R = RealTime, D = Delayed,
    ///Z = Frozen, Y = Frozen Delayed, N = Not Subscribed. Second char defines: P = Snapshot, p = Consolidated.
    ///hird char defines: B = Book
    ///  * RealTime - Data is relayed back in real time without delay, market data subscription(s) are required.
    ///  * Delayed - Data is relayed back 15-20 min delayed.
    ///  * Frozen - Last recorded data at market close, relayed back in real time.
    ///  * Frozen Delayed - Last recorded data at market close, relayed back delayed.
    ///  * Not Subscribed - User does not have the required market data subscription(s) to relay back either real time or delayed data.
    ///  * Snapshot - Snapshot request is available for contract.
    ///  * Consolidated - Market data is aggregated across multiple exchanges or venues.
    ///  * Book - Top of the book data is available for contract.
    MarketDataAvailability = 6509,
    ///Company name
    Companyname = 7051,
    ///Ask Exch - Displays the exchange(s) offering the SMART price. A=AMEX, C=CBOE, I=ISE, X=PHLX, N=PSE, B=BOX, Q=NASDAQOM, Z=BATS, W=CBOE2, T=NASDAQBX, M=MIAX, H=GEMINI, E=EDGX, J=MERCURY
    AskExch = 7057,
    ///Last Exch - Displays the exchange(s) offering the SMART price. A=AMEX, C=CBOE, I=ISE, X=PHLX, N=PSE, B=BOX, Q=NASDAQOM, Z=BATS, W=CBOE2, T=NASDAQBX, M=MIAX, H=GEMINI, E=EDGX, J=MERCURY
    LastExch = 7058,
    ///Last Size - The number of unites traded at the last price
    LastSize = 7059,
    ///Bid Exch - Displays the exchange(s) offering the SMART price. A=AMEX, C=CBOE, I=ISE, X=PHLX, N=PSE, B=BOX, Q=NASDAQOM, Z=BATS, W=CBOE2, T=NASDAQBX, M=MIAX, H=GEMINI, E=EDGX, J=MERCURY
    BidExch = 7068,
    ///Implied Vol./Hist. Vol % - The ratio of the implied volatility over the historical volatility, expressed as a percentage.
    ImpliedVolHistVol = 7084,
    ///Put/Call Interest - Put option open interest/call option open interest for the trading day.
    PutCallInterest = 7085,
    ///Put/Call Volume - Put option volume/call option volume for the trading day.
    PutCallVolume = 7086,
    ///Hist. Vol. % - 30-day real-time historical volatility.
    HistVol = 7087,
    ///Hist. Vol. Close % - Shows the historical volatility based on previous close price.
    HistVolClose = 7088,
    ///Opt. Volume - Option Volume
    OptVolume = 7089,
    ///Conid + Exchange
    ConidExchange = 7094,
    ///canBeTraded - If contract is a trade-able instrument. Returns 1(true) or 0(false).
    CanBeTraded = 7184,
    ///Contract Description
    ContractDescription = 7219,
    ///Contract Description
    ContractDescription2 = 7220,
    ///Listing Exchange
    ListingExchange = 7221,
    ///Industry - Displays the type of industry under which the underlying company can be categorized.
    Industry = 7280,
    ///Category - Displays a more detailed level of description within the industry under which the underlying company can be categorized.
    Category = 7281,
    ///Average Volume - The average daily trading volume over 90 days.
    AverageVolume = 7282,
    ///Option Implied Vol. % - A prediction of how volatile an underlying will be in the future.
    ///At the market volatility estimated for a maturity thirty calendar days forward of the current trading day, and based on option prices from two consecutive expiration months.
    ///To query the Implied Vol. % of a specific strike refer to field 7633.
    OptionImpliedVol = 7283,
    ///Historic Volume (30d)
    HistoricVolume30d = 7284,
    ///Put/Call Ratio
    PutCallRatio = 7285,
    ///Dividend Amount - Displays the amount of the next dividend.
    DividendAmount = 7286,
    ///Dividend Yield % - This value is the toal of the expected dividend payments over the next twelve months per share divided by the Current Price and is expressed as a percentage.
    ///For derivatives, this displays the total of the expected dividend payments over the expiry date.
    DividendYield = 7287,
    ///Ex-date of the dividend
    Ex = 7288,
    ///Market Cap
    MarketCap = 7289,
    ///P/E
    PE = 7290,
    ///EPS
    EPS = 7291,
    ///Cost Basis - Your current position in this security multiplied by the average price and multiplier.
    CostBasis = 7292,
    ///52 Week High - The highest price for the past 52 weeks.
    FiftyTwoWeekHigh = 7293,
    ///52 Week Low - The lowest price for the past 52 weeks.
    FiftyTwoWeekLow = 7294,
    ///Open - Today's opening price.
    Open = 7295,
    ///Close - Today's closing price.
    Close = 7296,
    ///Delta - The ratio of the change in the price of the option to the corresponding change in the price of the underlying.
    Delta = 7308,
    ///Gamma - The rate of change for the delta with respect to the underlying asset's price.
    Gamma = 7309,
    ///Theta - A measure of the rate of decline the value of an option due to the passage of time.
    Theta = 7310,
    ///Vega - The amount that the price of an option changes compared to a 1% change in the volatility.
    Vega = 7311,
    ///Opt. Volume Change % - Today's option volume as a percentage of the average option volume.
    OptVolumeChange = 7607,
    ///Implied Vol. % - The implied volatility for the specific strike of the option in percentage. To query the Option Implied Vol. % from the underlying refer to field 7283.
    ImpliedVol = 7633,
    ///Mark - The mark price is, the ask price if ask is less than last price, the bid price if bid is more than the last price, otherwise it's equal to last price.
    Mark = 7635,
    ///Shortable Shares - Number of shares available for shorting.
    ShortableShares = 7636,
    ///Fee Rate - Interest rate charged on borrowed shares.
    FeeRate = 7637,
    ///Option Open Interest
    OptionOpenInterest = 7638,
    ///% of Mark Value - Displays the market value of the contract as a percentage of the total market value of the account.
    //Mark Value is calculated with real time market data (even when not subscribed to market data).
    PctOfMarkValue = 7639,
    ///Shortable - Describes the level of difficulty with which the security can be sold short.
    Shortable = 7644,
    ///Morningstar Rating - Displays Morningstar Rating provided value. Requires [Morningstar](https://www.interactivebrokers.com/en/index.php?f=14262) subscription.
    MorningstarRating = 7655,
    ///Dividends - This value is the total of the expected dividend payments over the next twelve months per share.
    Dividends = 7671,
    ///Dividends TTM - This value is the total of the expected dividend payments over the last twelve months per share.
    DividendsTTM = 7672,
    ///EMA(200) - Exponential moving average (N=200).
    EMA200 = 7674,
    ///EMA(100) - Exponential moving average (N=100).
    EMA100 = 7675,
    ///EMA(50) - Exponential moving average (N=50).
    EMA50 = 7676,
    ///EMA(20) - Exponential moving average (N=20).
    EMA20 = 7677,
    ///Price/EMA(200) - Price to Exponential moving average (N=200) ratio -1, displayed in percents.
    PriceEMA200 = 7678,
    ///Price/EMA(100) - Price to Exponential moving average (N=100) ratio -1, displayed in percents.
    PriceEMA100 = 7679,
    ///Price/EMA(50) - Price to Exponential moving average (N=50) ratio -1, displayed in percents.
    PriceEMA50 = 7680,
    ///Price/EMA(20) - Price to Exponential moving average (N=20) ratio -1, displayed in percents.
    PriceEMA20 = 7681,
    ///Change Since Open - The difference between the last price and the open price.
    ChangeSinceOpen = 7682,
    ///Upcoming Event - Shows the next major company event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    UpcomingEvent = 7683,
    ///Upcoming Event Date - The date of the next major company event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    UpcomingEventDate = 7684,
    ///Upcoming Analyst Meeting - The date and time of the next scheduled analyst meeting. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    UpcomingAnalystMeeting = 7685,
    ///Upcoming Earnings - The date and time of the next scheduled earnings/earnings call event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    UpcomingEarnings = 7686,
    ///Upcoming Misc Event - The date and time of the next shareholder meeting, presentation or other event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    UpcomingMiscEvent = 7687,
    ///Recent Analyst Meeting - The date and time of the most recent analyst meeting. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    RecentAnalystMeeting = 7688,
    ///Recent Earnings - The date and time of the most recent earnings/earning call event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    RecentEarnings = 7689,
    ///Recent Misc Event - The date and time of the most recent shareholder meeting, presentation or other event. Requires [Wall Street Horizon](https://www.interactivebrokers.com/en/index.php?f=24674) subscription.
    RecentMiscEvent = 7690,
    ///Probability of Max Return - Customer implied probability of maximum potential gain.
    ProbabilityofMaxReturn = 7694,
    ///Break Even - Break even points
    BreakEven = 7695,
    ///SPX Delta - Beta Weighted Delta is calculated using the formula; Delta x dollar adjusted beta, where adjusted beta is adjusted by the ratio of the close price.
    SPXDelta = 7696,
    ///Futures Open Interest - Total number of outstanding futures contracts
    FuturesOpenInterest = 7697,
    ///Last Yield - Implied yield of the bond if it is purchased at the current last price. Last yield is calculated using the Last price on all possible call dates.
    ///It is assumed that prepayment occurs if the bond has call or put provisions and the issuer can offer a lower coupon rate based on current market rates.
    ///The yield to worst will be the lowest of the yield to maturity or yield to call (if the bond has prepayment provisions). Yield to worse may be the same as yield to maturity but never higher.
    LastYield = 7698,
    ///Bid Yield - Implied yield of the bond if it is purchased at the current bid price. Bid yield is calculated using the Ask on all possible call dates.
    ///It is assumed that prepayment occurs if the bond has call or put provisions and the issuer can offer a lower coupon rate based on current market rates.
    ///The yield to worst will be the lowest of the yield to maturity or yield to call (if the bond has prepayment provisions). Yield to worse may be the same as yield to maturity but never higher.
    BidYield = 7699,
    ///Probability of Max Return - Customer implied probability of maximum potential gain.
    ProbabilityofMaxReturn2 = 7700,
    ///Probability of Max Loss - Customer implied probability of maximum potential loss.
    ProbabilityofMaxLoss = 7702,
    ///Profit Probability - Customer implied probability of any gain.
    ProfitProbability = 7703,
    ///Organization Type
    OrganizationType = 7704,
    ///Debt Class
    DebtClass = 7705,
    ///Ratings - Ratings issued for bond contract.
    Ratings = 7706,
    ///Bond State Code
    BondStateCode = 7707,
    ///Bond Type
    BondType = 7708,
    ///Last Trading Date
    LastTradingDate = 7714,
    ///Issue Date
    IssueDate = 7715,
    ///Beta - Beta is against standard index.
    Beta = 7718,
    ///Ask Yield - Implied yield of the bond if it is purchased at the current offer. Ask yield is calculated using the Bid on all possible call dates.
    ///It is assumed that prepayment occurs if the bond has call or put provisions and the issuer can offer a lower coupon rate based on current market rates.
    ///The yield to worst will be the lowest of the yield to maturity or yield to call (if the bond has prepayment provisions). Yield to worse may be the same as yield to maturity but never higher.
    AskYield = 7720,
    ///Prior Close - Yesterday's closing price
    PriorClose = 7741,
    ///Volume Long - High precision volume for the day. For formatted volume refer to field 87.
    VolumeLong = 7762,
    ///hasTradingPermissions - if user has trading permissions for specified contract. Returns 1(true) or 0(false).
    HasTradingPermissions = 7768,
}
