/// Денежная сумма в определенной валюте
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoneyValue {
    /// строковый ISO-код валюты
    #[prost(string, tag = "1")]
    pub currency: ::prost::alloc::string::String,
    /// целая часть суммы, может быть отрицательным числом
    #[prost(int64, tag = "2")]
    pub units: i64,
    /// дробная часть суммы, может быть отрицательным числом
    #[prost(int32, tag = "3")]
    pub nano: i32,
}
/// Котировка - денежная сумма без указания валюты
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Quotation {
    /// целая часть суммы, может быть отрицательным числом
    #[prost(int64, tag = "1")]
    pub units: i64,
    /// дробная часть суммы, может быть отрицательным числом
    #[prost(int32, tag = "2")]
    pub nano: i32,
}
/// Проверка активности стрима.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ping {
    /// Время проверки.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Тип инструмента.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentType {
    Unspecified = 0,
    /// Облигация.
    Bond = 1,
    /// Акция.
    Share = 2,
    /// Валюта.
    Currency = 3,
    /// Exchange-traded fund. Фонд.
    Etf = 4,
    /// Фьючерс.
    Futures = 5,
    /// Структурная нота.
    Sp = 6,
    /// Опцион.
    Option = 7,
    /// Clearing certificate.
    ClearingCertificate = 8,
}
impl InstrumentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstrumentType::Unspecified => "INSTRUMENT_TYPE_UNSPECIFIED",
            InstrumentType::Bond => "INSTRUMENT_TYPE_BOND",
            InstrumentType::Share => "INSTRUMENT_TYPE_SHARE",
            InstrumentType::Currency => "INSTRUMENT_TYPE_CURRENCY",
            InstrumentType::Etf => "INSTRUMENT_TYPE_ETF",
            InstrumentType::Futures => "INSTRUMENT_TYPE_FUTURES",
            InstrumentType::Sp => "INSTRUMENT_TYPE_SP",
            InstrumentType::Option => "INSTRUMENT_TYPE_OPTION",
            InstrumentType::ClearingCertificate => "INSTRUMENT_TYPE_CLEARING_CERTIFICATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTRUMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "INSTRUMENT_TYPE_BOND" => Some(Self::Bond),
            "INSTRUMENT_TYPE_SHARE" => Some(Self::Share),
            "INSTRUMENT_TYPE_CURRENCY" => Some(Self::Currency),
            "INSTRUMENT_TYPE_ETF" => Some(Self::Etf),
            "INSTRUMENT_TYPE_FUTURES" => Some(Self::Futures),
            "INSTRUMENT_TYPE_SP" => Some(Self::Sp),
            "INSTRUMENT_TYPE_OPTION" => Some(Self::Option),
            "INSTRUMENT_TYPE_CLEARING_CERTIFICATE" => Some(Self::ClearingCertificate),
            _ => None,
        }
    }
}
/// Режим торгов инструмента
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SecurityTradingStatus {
    /// Торговый статус не определён
    Unspecified = 0,
    /// Недоступен для торгов
    NotAvailableForTrading = 1,
    /// Период открытия торгов
    OpeningPeriod = 2,
    /// Период закрытия торгов
    ClosingPeriod = 3,
    /// Перерыв в торговле
    BreakInTrading = 4,
    /// Нормальная торговля
    NormalTrading = 5,
    /// Аукцион закрытия
    ClosingAuction = 6,
    /// Аукцион крупных пакетов
    DarkPoolAuction = 7,
    /// Дискретный аукцион
    DiscreteAuction = 8,
    /// Аукцион открытия
    OpeningAuctionPeriod = 9,
    /// Период торгов по цене аукциона закрытия
    TradingAtClosingAuctionPrice = 10,
    /// Сессия назначена
    SessionAssigned = 11,
    /// Сессия закрыта
    SessionClose = 12,
    /// Сессия открыта
    SessionOpen = 13,
    /// Доступна торговля в режиме внутренней ликвидности брокера
    DealerNormalTrading = 14,
    /// Перерыв торговли в режиме внутренней ликвидности брокера
    DealerBreakInTrading = 15,
    /// Недоступна торговля в режиме внутренней ликвидности брокера
    DealerNotAvailableForTrading = 16,
}
impl SecurityTradingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SecurityTradingStatus::Unspecified => "SECURITY_TRADING_STATUS_UNSPECIFIED",
            SecurityTradingStatus::NotAvailableForTrading => {
                "SECURITY_TRADING_STATUS_NOT_AVAILABLE_FOR_TRADING"
            }
            SecurityTradingStatus::OpeningPeriod => {
                "SECURITY_TRADING_STATUS_OPENING_PERIOD"
            }
            SecurityTradingStatus::ClosingPeriod => {
                "SECURITY_TRADING_STATUS_CLOSING_PERIOD"
            }
            SecurityTradingStatus::BreakInTrading => {
                "SECURITY_TRADING_STATUS_BREAK_IN_TRADING"
            }
            SecurityTradingStatus::NormalTrading => {
                "SECURITY_TRADING_STATUS_NORMAL_TRADING"
            }
            SecurityTradingStatus::ClosingAuction => {
                "SECURITY_TRADING_STATUS_CLOSING_AUCTION"
            }
            SecurityTradingStatus::DarkPoolAuction => {
                "SECURITY_TRADING_STATUS_DARK_POOL_AUCTION"
            }
            SecurityTradingStatus::DiscreteAuction => {
                "SECURITY_TRADING_STATUS_DISCRETE_AUCTION"
            }
            SecurityTradingStatus::OpeningAuctionPeriod => {
                "SECURITY_TRADING_STATUS_OPENING_AUCTION_PERIOD"
            }
            SecurityTradingStatus::TradingAtClosingAuctionPrice => {
                "SECURITY_TRADING_STATUS_TRADING_AT_CLOSING_AUCTION_PRICE"
            }
            SecurityTradingStatus::SessionAssigned => {
                "SECURITY_TRADING_STATUS_SESSION_ASSIGNED"
            }
            SecurityTradingStatus::SessionClose => {
                "SECURITY_TRADING_STATUS_SESSION_CLOSE"
            }
            SecurityTradingStatus::SessionOpen => "SECURITY_TRADING_STATUS_SESSION_OPEN",
            SecurityTradingStatus::DealerNormalTrading => {
                "SECURITY_TRADING_STATUS_DEALER_NORMAL_TRADING"
            }
            SecurityTradingStatus::DealerBreakInTrading => {
                "SECURITY_TRADING_STATUS_DEALER_BREAK_IN_TRADING"
            }
            SecurityTradingStatus::DealerNotAvailableForTrading => {
                "SECURITY_TRADING_STATUS_DEALER_NOT_AVAILABLE_FOR_TRADING"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SECURITY_TRADING_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SECURITY_TRADING_STATUS_NOT_AVAILABLE_FOR_TRADING" => {
                Some(Self::NotAvailableForTrading)
            }
            "SECURITY_TRADING_STATUS_OPENING_PERIOD" => Some(Self::OpeningPeriod),
            "SECURITY_TRADING_STATUS_CLOSING_PERIOD" => Some(Self::ClosingPeriod),
            "SECURITY_TRADING_STATUS_BREAK_IN_TRADING" => Some(Self::BreakInTrading),
            "SECURITY_TRADING_STATUS_NORMAL_TRADING" => Some(Self::NormalTrading),
            "SECURITY_TRADING_STATUS_CLOSING_AUCTION" => Some(Self::ClosingAuction),
            "SECURITY_TRADING_STATUS_DARK_POOL_AUCTION" => Some(Self::DarkPoolAuction),
            "SECURITY_TRADING_STATUS_DISCRETE_AUCTION" => Some(Self::DiscreteAuction),
            "SECURITY_TRADING_STATUS_OPENING_AUCTION_PERIOD" => {
                Some(Self::OpeningAuctionPeriod)
            }
            "SECURITY_TRADING_STATUS_TRADING_AT_CLOSING_AUCTION_PRICE" => {
                Some(Self::TradingAtClosingAuctionPrice)
            }
            "SECURITY_TRADING_STATUS_SESSION_ASSIGNED" => Some(Self::SessionAssigned),
            "SECURITY_TRADING_STATUS_SESSION_CLOSE" => Some(Self::SessionClose),
            "SECURITY_TRADING_STATUS_SESSION_OPEN" => Some(Self::SessionOpen),
            "SECURITY_TRADING_STATUS_DEALER_NORMAL_TRADING" => {
                Some(Self::DealerNormalTrading)
            }
            "SECURITY_TRADING_STATUS_DEALER_BREAK_IN_TRADING" => {
                Some(Self::DealerBreakInTrading)
            }
            "SECURITY_TRADING_STATUS_DEALER_NOT_AVAILABLE_FOR_TRADING" => {
                Some(Self::DealerNotAvailableForTrading)
            }
            _ => None,
        }
    }
}
/// Запрос расписания торгов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedulesRequest {
    /// Наименование биржи или расчетного календаря. </br>Если не передаётся, возвращается информация по всем доступным торговым площадкам.
    #[prost(string, tag = "1")]
    pub exchange: ::prost::alloc::string::String,
    /// Начало периода по часовому поясу UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода по часовому поясу UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Список торговых площадок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedulesResponse {
    /// Список торговых площадок и режимов торгов.
    #[prost(message, repeated, tag = "1")]
    pub exchanges: ::prost::alloc::vec::Vec<TradingSchedule>,
}
/// Данные по торговой площадке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingSchedule {
    /// Наименование торговой площадки.
    #[prost(string, tag = "1")]
    pub exchange: ::prost::alloc::string::String,
    /// Массив с торговыми и неторговыми днями.
    #[prost(message, repeated, tag = "2")]
    pub days: ::prost::alloc::vec::Vec<TradingDay>,
}
/// Информация о времени торгов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingDay {
    /// Дата.
    #[prost(message, optional, tag = "1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак торгового дня на бирже.
    #[prost(bool, tag = "2")]
    pub is_trading_day: bool,
    /// Время начала торгов по часовому поясу UTC.
    #[prost(message, optional, tag = "3")]
    pub start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания торгов по часовому поясу UTC.
    #[prost(message, optional, tag = "4")]
    pub end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала аукциона открытия в часовом поясе UTC.
    #[prost(message, optional, tag = "7")]
    pub opening_auction_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания аукциона закрытия в часовом поясе UTC.
    #[prost(message, optional, tag = "8")]
    pub closing_auction_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала аукциона открытия вечерней сессии в часовом поясе UTC.
    #[prost(message, optional, tag = "9")]
    pub evening_opening_auction_start_time: ::core::option::Option<
        ::prost_types::Timestamp,
    >,
    /// Время начала вечерней сессии в часовом поясе UTC.
    #[prost(message, optional, tag = "10")]
    pub evening_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания вечерней сессии в часовом поясе UTC.
    #[prost(message, optional, tag = "11")]
    pub evening_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала основного клиринга в часовом поясе UTC.
    #[prost(message, optional, tag = "12")]
    pub clearing_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания основного клиринга в часовом поясе UTC.
    #[prost(message, optional, tag = "13")]
    pub clearing_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала премаркета в часовом поясе UTC.
    #[prost(message, optional, tag = "14")]
    pub premarket_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания премаркета в часовом поясе UTC.
    #[prost(message, optional, tag = "15")]
    pub premarket_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время начала аукциона закрытия в часовом поясе UTC.
    #[prost(message, optional, tag = "16")]
    pub closing_auction_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время окончания аукциона открытия в часовом поясе UTC.
    #[prost(message, optional, tag = "17")]
    pub opening_auction_end_time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос получения инструмента по идентификатору.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentRequest {
    /// Тип идентификатора инструмента. Возможные значения: figi, ticker. Подробнее об идентификации инструментов: [Идентификация инструментов](<https://tinkoff.github.io/investAPI/faq_identification/>)
    #[prost(enumeration = "InstrumentIdType", tag = "1")]
    pub id_type: i32,
    /// Идентификатор class_code. Обязателен при id_type = ticker.
    #[prost(string, tag = "2")]
    pub class_code: ::prost::alloc::string::String,
    /// Идентификатор запрашиваемого инструмента.
    #[prost(string, tag = "3")]
    pub id: ::prost::alloc::string::String,
}
/// Запрос получения инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentsRequest {
    /// Статус запрашиваемых инструментов. Возможные значения: \[InstrumentStatus\](#instrumentstatus)
    #[prost(enumeration = "InstrumentStatus", tag = "1")]
    pub instrument_status: i32,
}
/// Параметры фильтрации опционов
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FilterOptionsRequest {
    /// Идентификатор базового актива опциона.  Обязательный параметр.
    #[prost(string, tag = "1")]
    pub basic_asset_uid: ::prost::alloc::string::String,
    /// Идентификатор позиции базового актива опциона
    #[prost(string, tag = "2")]
    pub basic_asset_position_uid: ::prost::alloc::string::String,
}
/// Информация об облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondResponse {
    /// Информация об облигации.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Bond>,
}
/// Список облигаций.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BondsResponse {
    /// Массив облигаций.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Bond>,
}
/// Запрос купонов по облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBondCouponsRequest {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC. Фильтрация по coupon_date (дата выплаты купона)
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC. Фильтрация по coupon_date (дата выплаты купона)
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Купоны по облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBondCouponsResponse {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<Coupon>,
}
/// Объект передачи информации о купоне облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coupon {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Дата выплаты купона.
    #[prost(message, optional, tag = "2")]
    pub coupon_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Номер купона.
    #[prost(int64, tag = "3")]
    pub coupon_number: i64,
    /// (Опционально) Дата фиксации реестра для выплаты купона.
    #[prost(message, optional, tag = "4")]
    pub fix_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Выплата на одну облигацию.
    #[prost(message, optional, tag = "5")]
    pub pay_one_bond: ::core::option::Option<MoneyValue>,
    /// Тип купона.
    #[prost(enumeration = "CouponType", tag = "6")]
    pub coupon_type: i32,
    /// Начало купонного периода.
    #[prost(message, optional, tag = "7")]
    pub coupon_start_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание купонного периода.
    #[prost(message, optional, tag = "8")]
    pub coupon_end_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Купонный период в днях.
    #[prost(int32, tag = "9")]
    pub coupon_period: i32,
}
/// Данные по валюте.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrencyResponse {
    /// Информация о валюте.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Currency>,
}
/// Данные по валютам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CurrenciesResponse {
    /// Массив валют.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Currency>,
}
/// Данные по фонду.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtfResponse {
    /// Информация о фонде.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Etf>,
}
/// Данные по фондам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtfsResponse {
    /// Массив фондов.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Etf>,
}
/// Данные по фьючерсу.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FutureResponse {
    /// Информация о фьючерсу.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Future>,
}
/// Данные по фьючерсам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FuturesResponse {
    /// Массив фьючерсов.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Future>,
}
/// Данные по опциону.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionResponse {
    /// Информация по опциону.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Option>,
}
/// Данные по опционам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptionsResponse {
    /// Массив данных по опциону.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Option>,
}
/// Опцион.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Option {
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции.
    #[prost(string, tag = "2")]
    pub position_uid: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "3")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код.
    #[prost(string, tag = "4")]
    pub class_code: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции основного инструмента.
    #[prost(string, tag = "5")]
    pub basic_asset_position_uid: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "21")]
    pub trading_status: i32,
    /// Реальная площадка исполнения расчётов. Допустимые значения: [REAL_EXCHANGE_MOEX, REAL_EXCHANGE_RTS]
    #[prost(enumeration = "RealExchange", tag = "31")]
    pub real_exchange: i32,
    /// Направление опциона.
    #[prost(enumeration = "OptionDirection", tag = "41")]
    pub direction: i32,
    /// Тип расчетов по опциону.
    #[prost(enumeration = "OptionPaymentType", tag = "42")]
    pub payment_type: i32,
    /// Стиль опциона.
    #[prost(enumeration = "OptionStyle", tag = "43")]
    pub style: i32,
    /// Способ исполнения опциона.
    #[prost(enumeration = "OptionSettlementType", tag = "44")]
    pub settlement_type: i32,
    /// Название инструмента.
    #[prost(string, tag = "101")]
    pub name: ::prost::alloc::string::String,
    /// Валюта.
    #[prost(string, tag = "111")]
    pub currency: ::prost::alloc::string::String,
    /// Валюта, в которой оценивается контракт.
    #[prost(string, tag = "112")]
    pub settlement_currency: ::prost::alloc::string::String,
    /// Тип актива.
    #[prost(string, tag = "131")]
    pub asset_type: ::prost::alloc::string::String,
    /// Основной актив.
    #[prost(string, tag = "132")]
    pub basic_asset: ::prost::alloc::string::String,
    /// Биржа.
    #[prost(string, tag = "141")]
    pub exchange: ::prost::alloc::string::String,
    /// Код страны рисков.
    #[prost(string, tag = "151")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны рисков.
    #[prost(string, tag = "152")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag = "161")]
    pub sector: ::prost::alloc::string::String,
    /// Количество бумаг в лоте.
    #[prost(int32, tag = "201")]
    pub lot: i32,
    /// Размер основного актива.
    #[prost(message, optional, tag = "211")]
    pub basic_asset_size: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска длинной позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР). 1 – клиент с повышенным уровнем риска (КПУР)
    #[prost(message, optional, tag = "221")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по клиенту. 2 – клиент со стандартным уровнем риска (КСУР). 1 – клиент с повышенным уровнем риска (КПУР)
    #[prost(message, optional, tag = "222")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР лонг.
    #[prost(message, optional, tag = "223")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КСУР шорт.
    #[prost(message, optional, tag = "224")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР лонг.
    #[prost(message, optional, tag = "225")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи для КПУР шорт.
    #[prost(message, optional, tag = "226")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Минимальный шаг цены.
    #[prost(message, optional, tag = "231")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Цена страйка.
    #[prost(message, optional, tag = "241")]
    pub strike_price: ::core::option::Option<MoneyValue>,
    /// Дата истечения срока в формате UTC.
    #[prost(message, optional, tag = "301")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата начала обращения контракта в формате UTC.
    #[prost(message, optional, tag = "311")]
    pub first_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата исполнения в формате UTC.
    #[prost(message, optional, tag = "312")]
    pub last_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой минутной свечи в формате UTC.
    #[prost(message, optional, tag = "321")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи в формате UTC.
    #[prost(message, optional, tag = "322")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак доступности для операций шорт.
    #[prost(bool, tag = "401")]
    pub short_enabled_flag: bool,
    /// Возможность покупки/продажи на ИИС.
    #[prost(bool, tag = "402")]
    pub for_iis_flag: bool,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "403")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "404")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "405")]
    pub sell_available_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "406")]
    pub for_qual_investor_flag: bool,
    /// Флаг отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "407")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "408")]
    pub blocked_tca_flag: bool,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "409")]
    pub api_trade_available_flag: bool,
}
/// Данные по акции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShareResponse {
    /// Информация об акции.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Share>,
}
/// Данные по акциям.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SharesResponse {
    /// Массив акций.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<Share>,
}
/// Объект передачи информации об облигации.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bond {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    /// Количество выплат по купонам в год.
    #[prost(int32, tag = "17")]
    pub coupon_quantity_per_year: i32,
    /// Дата погашения облигации в часовом поясе UTC.
    #[prost(message, optional, tag = "18")]
    pub maturity_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Номинал облигации.
    #[prost(message, optional, tag = "19")]
    pub nominal: ::core::option::Option<MoneyValue>,
    /// Первоначальный номинал облигации.
    #[prost(message, optional, tag = "20")]
    pub initial_nominal: ::core::option::Option<MoneyValue>,
    /// Дата выпуска облигации в часовом поясе UTC.
    #[prost(message, optional, tag = "21")]
    pub state_reg_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата размещения в часовом поясе UTC.
    #[prost(message, optional, tag = "22")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена размещения.
    #[prost(message, optional, tag = "23")]
    pub placement_price: ::core::option::Option<MoneyValue>,
    /// Значение НКД (накопленного купонного дохода) на дату.
    #[prost(message, optional, tag = "24")]
    pub aci_value: ::core::option::Option<MoneyValue>,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "25")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "26")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag = "27")]
    pub sector: ::prost::alloc::string::String,
    /// Форма выпуска. Возможные значения: </br>**documentary** — документарная; </br>**non_documentary** — бездокументарная.
    #[prost(string, tag = "28")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Размер выпуска.
    #[prost(int64, tag = "29")]
    pub issue_size: i64,
    /// Плановый размер выпуска.
    #[prost(int64, tag = "30")]
    pub issue_size_plan: i64,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "31")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "32")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "33")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "34")]
    pub sell_available_flag: bool,
    /// Признак облигации с плавающим купоном.
    #[prost(bool, tag = "35")]
    pub floating_coupon_flag: bool,
    /// Признак бессрочной облигации.
    #[prost(bool, tag = "36")]
    pub perpetual_flag: bool,
    /// Признак облигации с амортизацией долга.
    #[prost(bool, tag = "37")]
    pub amortization_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag = "38")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "39")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "40")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration = "RealExchange", tag = "41")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "42")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "51")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "52")]
    pub for_qual_investor_flag: bool,
    /// Флаг отображающий доступность торговли инструментом по выходным
    #[prost(bool, tag = "53")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС
    #[prost(bool, tag = "54")]
    pub blocked_tca_flag: bool,
    /// Признак субординированной облигации.
    #[prost(bool, tag = "55")]
    pub subordinated_flag: bool,
    /// Флаг достаточной ликвидности
    #[prost(bool, tag = "56")]
    pub liquidity_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "61")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "62")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Уровень риска.
    #[prost(enumeration = "RiskLevel", tag = "63")]
    pub risk_level: i32,
}
/// Объект передачи информации о валюте.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Currency {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    /// Номинал.
    #[prost(message, optional, tag = "17")]
    pub nominal: ::core::option::Option<MoneyValue>,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "18")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "19")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "20")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "21")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "22")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "23")]
    pub sell_available_flag: bool,
    /// Строковый ISO-код валюты.
    #[prost(string, tag = "24")]
    pub iso_currency_name: ::prost::alloc::string::String,
    /// Шаг цены.
    #[prost(message, optional, tag = "25")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "26")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "27")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration = "RealExchange", tag = "28")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "29")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "41")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "52")]
    pub for_qual_investor_flag: bool,
    /// Флаг отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "53")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "54")]
    pub blocked_tca_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Объект передачи информации об инвестиционном фонде.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Etf {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    /// Размер фиксированной комиссии фонда.
    #[prost(message, optional, tag = "17")]
    pub fixed_commission: ::core::option::Option<Quotation>,
    /// Возможные значения: </br>**equity** — акции;</br>**fixed_income** — облигации;</br>**mixed_allocation** — смешанный;</br>**money_market** — денежный рынок;</br>**real_estate** — недвижимость;</br>**commodity** — товары;</br>**specialty** — специальный;</br>**private_equity** — private equity;</br>**alternative_investment** — альтернативные инвестиции.
    #[prost(string, tag = "18")]
    pub focus_type: ::prost::alloc::string::String,
    /// Дата выпуска в часовом поясе UTC.
    #[prost(message, optional, tag = "19")]
    pub released_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Количество акций фонда в обращении.
    #[prost(message, optional, tag = "20")]
    pub num_shares: ::core::option::Option<Quotation>,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "21")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "22")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag = "23")]
    pub sector: ::prost::alloc::string::String,
    /// Частота ребалансировки.
    #[prost(string, tag = "24")]
    pub rebalancing_freq: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "25")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "26")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "27")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "28")]
    pub sell_available_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag = "29")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "30")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "31")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration = "RealExchange", tag = "32")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "33")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "41")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "42")]
    pub for_qual_investor_flag: bool,
    /// Флаг отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "43")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "44")]
    pub blocked_tca_flag: bool,
    /// Флаг достаточной ликвидности
    #[prost(bool, tag = "45")]
    pub liquidity_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Объект передачи информации о фьючерсе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Future {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "4")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "5")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по клиенту.
    #[prost(message, optional, tag = "6")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по клиенту.
    #[prost(message, optional, tag = "7")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "8")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций шорт.
    #[prost(bool, tag = "12")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "13")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag = "14")]
    pub exchange: ::prost::alloc::string::String,
    /// Дата начала обращения контракта в часовом поясе UTC.
    #[prost(message, optional, tag = "15")]
    pub first_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата в часовом поясе UTC, до которой возможно проведение операций с фьючерсом.
    #[prost(message, optional, tag = "16")]
    pub last_trade_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Тип фьючерса. Возможные значения: </br>**physical_delivery** — физические поставки; </br>**cash_settlement** — денежный эквивалент.
    #[prost(string, tag = "17")]
    pub futures_type: ::prost::alloc::string::String,
    /// Тип актива. Возможные значения: </br>**commodity** — товар; </br>**currency** — валюта; </br>**security** — ценная бумага; </br>**index** — индекс.
    #[prost(string, tag = "18")]
    pub asset_type: ::prost::alloc::string::String,
    /// Основной актив.
    #[prost(string, tag = "19")]
    pub basic_asset: ::prost::alloc::string::String,
    /// Размер основного актива.
    #[prost(message, optional, tag = "20")]
    pub basic_asset_size: ::core::option::Option<Quotation>,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "21")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "22")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag = "23")]
    pub sector: ::prost::alloc::string::String,
    /// Дата истечения срока в часов поясе UTC.
    #[prost(message, optional, tag = "24")]
    pub expiration_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "25")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "26")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "27")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "28")]
    pub sell_available_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag = "29")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "30")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "31")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration = "RealExchange", tag = "32")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "33")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции основного инструмента.
    #[prost(string, tag = "34")]
    pub basic_asset_position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "41")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "42")]
    pub for_qual_investor_flag: bool,
    /// Флаг отображающий доступность торговли инструментом по выходным.
    #[prost(bool, tag = "43")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС.
    #[prost(bool, tag = "44")]
    pub blocked_tca_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Объект передачи информации об акции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Share {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "15")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag = "16")]
    pub exchange: ::prost::alloc::string::String,
    /// Дата IPO акции в часовом поясе UTC.
    #[prost(message, optional, tag = "17")]
    pub ipo_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Размер выпуска.
    #[prost(int64, tag = "18")]
    pub issue_size: i64,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "19")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "20")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Сектор экономики.
    #[prost(string, tag = "21")]
    pub sector: ::prost::alloc::string::String,
    /// Плановый размер выпуска.
    #[prost(int64, tag = "22")]
    pub issue_size_plan: i64,
    /// Номинал.
    #[prost(message, optional, tag = "23")]
    pub nominal: ::core::option::Option<MoneyValue>,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "25")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "26")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "27")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "28")]
    pub sell_available_flag: bool,
    /// Признак наличия дивидендной доходности.
    #[prost(bool, tag = "29")]
    pub div_yield_flag: bool,
    /// Тип акции. Возможные значения: \[ShareType\](<https://tinkoff.github.io/investAPI/instruments#sharetype>)
    #[prost(enumeration = "ShareType", tag = "30")]
    pub share_type: i32,
    /// Шаг цены.
    #[prost(message, optional, tag = "31")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "32")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "33")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration = "RealExchange", tag = "34")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "35")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "46")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "47")]
    pub for_qual_investor_flag: bool,
    /// Флаг отображающий доступность торговли инструментом по выходным
    #[prost(bool, tag = "48")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС
    #[prost(bool, tag = "49")]
    pub blocked_tca_flag: bool,
    /// Флаг достаточной ликвидности
    #[prost(bool, tag = "50")]
    pub liquidity_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос НКД по облигации
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccruedInterestsRequest {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// НКД облигации
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccruedInterestsResponse {
    /// Массив операций начисления купонов.
    #[prost(message, repeated, tag = "1")]
    pub accrued_interests: ::prost::alloc::vec::Vec<AccruedInterest>,
}
/// Операция начисления купонов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccruedInterest {
    /// Дата и время выплаты в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Величина выплаты.
    #[prost(message, optional, tag = "2")]
    pub value: ::core::option::Option<Quotation>,
    /// Величина выплаты в процентах от номинала.
    #[prost(message, optional, tag = "3")]
    pub value_percent: ::core::option::Option<Quotation>,
    /// Номинал облигации.
    #[prost(message, optional, tag = "4")]
    pub nominal: ::core::option::Option<Quotation>,
}
/// Запрос информации о фьючерсе
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFuturesMarginRequest {
    /// Идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
}
/// Данные по фьючерсу
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFuturesMarginResponse {
    /// Гарантийное обеспечение при покупке.
    #[prost(message, optional, tag = "1")]
    pub initial_margin_on_buy: ::core::option::Option<MoneyValue>,
    /// Гарантийное обеспечение при продаже.
    #[prost(message, optional, tag = "2")]
    pub initial_margin_on_sell: ::core::option::Option<MoneyValue>,
    /// Шаг цены.
    #[prost(message, optional, tag = "3")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Стоимость шага цены.
    #[prost(message, optional, tag = "4")]
    pub min_price_increment_amount: ::core::option::Option<Quotation>,
}
/// Данные по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentResponse {
    /// Основная информация об инструменте.
    #[prost(message, optional, tag = "1")]
    pub instrument: ::core::option::Option<Instrument>,
}
/// Объект передачи основной информации об инструменте.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instrument {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код инструмента.
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Лотность инструмента. Возможно совершение операций только на количества ценной бумаги, кратные параметру *lot*. Подробнее: \[лот\](<https://tinkoff.github.io/investAPI/glossary#lot>)
    #[prost(int32, tag = "5")]
    pub lot: i32,
    /// Валюта расчётов.
    #[prost(string, tag = "6")]
    pub currency: ::prost::alloc::string::String,
    /// Коэффициент ставки риска длинной позиции по инструменту.
    #[prost(message, optional, tag = "7")]
    pub klong: ::core::option::Option<Quotation>,
    /// Коэффициент ставки риска короткой позиции по инструменту.
    #[prost(message, optional, tag = "8")]
    pub kshort: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "9")]
    pub dlong: ::core::option::Option<Quotation>,
    /// Ставка риска минимальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "10")]
    pub dshort: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в лонг. Подробнее: [ставка риска в лонг](<https://help.tinkoff.ru/margin-trade/long/risk-rate/>)
    #[prost(message, optional, tag = "11")]
    pub dlong_min: ::core::option::Option<Quotation>,
    /// Ставка риска начальной маржи в шорт. Подробнее: [ставка риска в шорт](<https://help.tinkoff.ru/margin-trade/short/risk-rate/>)
    #[prost(message, optional, tag = "12")]
    pub dshort_min: ::core::option::Option<Quotation>,
    /// Признак доступности для операций в шорт.
    #[prost(bool, tag = "13")]
    pub short_enabled_flag: bool,
    /// Название инструмента.
    #[prost(string, tag = "14")]
    pub name: ::prost::alloc::string::String,
    /// Торговая площадка.
    #[prost(string, tag = "15")]
    pub exchange: ::prost::alloc::string::String,
    /// Код страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "16")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска, т.е. страны, в которой компания ведёт основной бизнес.
    #[prost(string, tag = "17")]
    pub country_of_risk_name: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "18")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Текущий режим торгов инструмента.
    #[prost(enumeration = "SecurityTradingStatus", tag = "19")]
    pub trading_status: i32,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "20")]
    pub otc_flag: bool,
    /// Признак доступности для покупки.
    #[prost(bool, tag = "21")]
    pub buy_available_flag: bool,
    /// Признак доступности для продажи.
    #[prost(bool, tag = "22")]
    pub sell_available_flag: bool,
    /// Шаг цены.
    #[prost(message, optional, tag = "23")]
    pub min_price_increment: ::core::option::Option<Quotation>,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "24")]
    pub api_trade_available_flag: bool,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "25")]
    pub uid: ::prost::alloc::string::String,
    /// Реальная площадка исполнения расчётов.
    #[prost(enumeration = "RealExchange", tag = "26")]
    pub real_exchange: i32,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "27")]
    pub position_uid: ::prost::alloc::string::String,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "36")]
    pub for_iis_flag: bool,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "37")]
    pub for_qual_investor_flag: bool,
    /// Флаг отображающий доступность торговли инструментом по выходным
    #[prost(bool, tag = "38")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС
    #[prost(bool, tag = "39")]
    pub blocked_tca_flag: bool,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "40")]
    pub instrument_kind: i32,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "56")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "57")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос дивидендов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsRequest {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC. Фильтрация происходит по параметру *record_date* (дата фиксации реестра).
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC. Фильтрация происходит по параметру *record_date* (дата фиксации реестра).
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Дивиденды.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsResponse {
    #[prost(message, repeated, tag = "1")]
    pub dividends: ::prost::alloc::vec::Vec<Dividend>,
}
/// Информация о выплате.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Dividend {
    /// Величина дивиденда на 1 ценную бумагу (включая валюту).
    #[prost(message, optional, tag = "1")]
    pub dividend_net: ::core::option::Option<MoneyValue>,
    /// Дата фактических выплат в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub payment_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата объявления дивидендов в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub declared_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Последний день (включительно) покупки для получения выплаты в часовом поясе UTC.
    #[prost(message, optional, tag = "4")]
    pub last_buy_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Тип выплаты. Возможные значения: Regular Cash – регулярные выплаты, Cancelled – выплата отменена, Daily Accrual – ежедневное начисление, Return of Capital – возврат капитала, прочие типы выплат.
    #[prost(string, tag = "5")]
    pub dividend_type: ::prost::alloc::string::String,
    /// Дата фиксации реестра в часовом поясе UTC.
    #[prost(message, optional, tag = "6")]
    pub record_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Регулярность выплаты. Возможные значения: Annual – ежегодная, Semi-Anl – каждые полгода, прочие типы выплат.
    #[prost(string, tag = "7")]
    pub regularity: ::prost::alloc::string::String,
    /// Цена закрытия инструмента на момент ex_dividend_date.
    #[prost(message, optional, tag = "8")]
    pub close_price: ::core::option::Option<MoneyValue>,
    /// Величина доходности.
    #[prost(message, optional, tag = "9")]
    pub yield_value: ::core::option::Option<Quotation>,
    /// Дата и время создания записи в часовом поясе UTC.
    #[prost(message, optional, tag = "10")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос актива по идентификатору.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetRequest {
    /// uid-идентификатор актива.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Данные по активу.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetResponse {
    /// Актив.
    #[prost(message, optional, tag = "1")]
    pub asset: ::core::option::Option<AssetFull>,
}
/// Запрос списка активов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsRequest {
    #[prost(enumeration = "InstrumentType", tag = "1")]
    pub instrument_type: i32,
}
/// Список активов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetsResponse {
    /// Активы.
    #[prost(message, repeated, tag = "1")]
    pub assets: ::prost::alloc::vec::Vec<Asset>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetFull {
    /// Уникальный идентификатор актива.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    /// Тип актива.
    #[prost(enumeration = "AssetType", tag = "2")]
    pub r#type: i32,
    /// Наименование актива.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Короткое наименование актива.
    #[prost(string, tag = "4")]
    pub name_brief: ::prost::alloc::string::String,
    /// Описание актива.
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    /// Дата и время удаления актива.
    #[prost(message, optional, tag = "6")]
    pub deleted_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Тестирование клиентов.
    #[prost(string, repeated, tag = "7")]
    pub required_tests: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Номер государственной регистрации.
    #[prost(string, tag = "10")]
    pub gos_reg_code: ::prost::alloc::string::String,
    /// Код CFI.
    #[prost(string, tag = "11")]
    pub cfi: ::prost::alloc::string::String,
    /// Код НРД инструмента.
    #[prost(string, tag = "12")]
    pub code_nsd: ::prost::alloc::string::String,
    /// Статус актива.
    #[prost(string, tag = "13")]
    pub status: ::prost::alloc::string::String,
    /// Бренд.
    #[prost(message, optional, tag = "14")]
    pub brand: ::core::option::Option<Brand>,
    /// Дата и время последнего обновления записи.
    #[prost(message, optional, tag = "15")]
    pub updated_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Код типа ц.б. по классификации Банка России.
    #[prost(string, tag = "16")]
    pub br_code: ::prost::alloc::string::String,
    /// Наименование кода типа ц.б. по классификации Банка России.
    #[prost(string, tag = "17")]
    pub br_code_name: ::prost::alloc::string::String,
    /// Массив идентификаторов инструментов.
    #[prost(message, repeated, tag = "18")]
    pub instruments: ::prost::alloc::vec::Vec<AssetInstrument>,
    #[prost(oneof = "asset_full::Ext", tags = "8, 9")]
    pub ext: ::core::option::Option<asset_full::Ext>,
}
/// Nested message and enum types in `AssetFull`.
pub mod asset_full {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ext {
        /// Валюта. Обязательно и заполняется только для type = "ASSET_TYPE_CURRENCY".
        #[prost(message, tag = "8")]
        Currency(super::AssetCurrency),
        /// Ценная бумага. Обязательно и заполняется только для type = "ASSET_TYPE_SECURITY".
        #[prost(message, tag = "9")]
        Security(super::AssetSecurity),
    }
}
/// Информация об активе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// Уникальный идентификатор актива.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    /// Тип актива.
    #[prost(enumeration = "AssetType", tag = "2")]
    pub r#type: i32,
    /// Наименование актива.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Массив идентификаторов инструментов.
    #[prost(message, repeated, tag = "4")]
    pub instruments: ::prost::alloc::vec::Vec<AssetInstrument>,
}
/// Валюта.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetCurrency {
    /// ISO-код валюты.
    #[prost(string, tag = "1")]
    pub base_currency: ::prost::alloc::string::String,
}
/// Ценная бумага.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetSecurity {
    /// ISIN-идентификатор ценной бумаги.
    #[prost(string, tag = "1")]
    pub isin: ::prost::alloc::string::String,
    /// Тип ценной бумаги.
    #[prost(string, tag = "2")]
    pub r#type: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "10")]
    pub instrument_kind: i32,
    #[prost(oneof = "asset_security::Ext", tags = "3, 4, 5, 6, 7")]
    pub ext: ::core::option::Option<asset_security::Ext>,
}
/// Nested message and enum types in `AssetSecurity`.
pub mod asset_security {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Ext {
        /// Акция. Заполняется только для акций (тип актива asset.type = "ASSET_TYPE_SECURITY" и security.type = share).
        #[prost(message, tag = "3")]
        Share(super::AssetShare),
        /// Облигация. Заполняется только для облигаций (тип актива asset.type = "ASSET_TYPE_SECURITY" и security.type = bond).
        #[prost(message, tag = "4")]
        Bond(super::AssetBond),
        /// Структурная нота. Заполняется только для структурных продуктов (тип актива asset.type = "ASSET_TYPE_SECURITY" и security.type = sp).
        #[prost(message, tag = "5")]
        Sp(super::AssetStructuredProduct),
        /// Фонд. Заполняется только для фондов (тип актива asset.type = "ASSET_TYPE_SECURITY" и security.type = etf).
        #[prost(message, tag = "6")]
        Etf(super::AssetEtf),
        /// Клиринговый сертификат участия. Заполняется только для клиринговых сертификатов (тип актива asset.type = "ASSET_TYPE_SECURITY" и security.type = clearing_certificate).
        #[prost(message, tag = "7")]
        ClearingCertificate(super::AssetClearingCertificate),
    }
}
/// Акция.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetShare {
    /// Тип акции.
    #[prost(enumeration = "ShareType", tag = "1")]
    pub r#type: i32,
    /// Объем выпуска (шт.).
    #[prost(message, optional, tag = "2")]
    pub issue_size: ::core::option::Option<Quotation>,
    /// Номинал.
    #[prost(message, optional, tag = "3")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag = "4")]
    pub nominal_currency: ::prost::alloc::string::String,
    /// Индекс (Bloomberg).
    #[prost(string, tag = "5")]
    pub primary_index: ::prost::alloc::string::String,
    /// Ставка дивиденда (для привилегированных акций).
    #[prost(message, optional, tag = "6")]
    pub dividend_rate: ::core::option::Option<Quotation>,
    /// Тип привилегированных акций.
    #[prost(string, tag = "7")]
    pub preferred_share_type: ::prost::alloc::string::String,
    /// Дата IPO.
    #[prost(message, optional, tag = "8")]
    pub ipo_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата регистрации.
    #[prost(message, optional, tag = "9")]
    pub registry_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак наличия дивидендной доходности.
    #[prost(bool, tag = "10")]
    pub div_yield_flag: bool,
    /// Форма выпуска ФИ.
    #[prost(string, tag = "11")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Дата размещения акции.
    #[prost(message, optional, tag = "12")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// ISIN базового актива.
    #[prost(string, tag = "13")]
    pub repres_isin: ::prost::alloc::string::String,
    /// Объявленное количество шт.
    #[prost(message, optional, tag = "14")]
    pub issue_size_plan: ::core::option::Option<Quotation>,
    /// Количество акций в свободном обращении.
    #[prost(message, optional, tag = "15")]
    pub total_float: ::core::option::Option<Quotation>,
}
/// Облигация.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetBond {
    /// Текущий номинал.
    #[prost(message, optional, tag = "1")]
    pub current_nominal: ::core::option::Option<Quotation>,
    /// Наименование заемщика.
    #[prost(string, tag = "2")]
    pub borrow_name: ::prost::alloc::string::String,
    /// Объем эмиссии облигации (стоимость).
    #[prost(message, optional, tag = "3")]
    pub issue_size: ::core::option::Option<Quotation>,
    /// Номинал облигации.
    #[prost(message, optional, tag = "4")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag = "5")]
    pub nominal_currency: ::prost::alloc::string::String,
    /// Форма выпуска облигации.
    #[prost(string, tag = "6")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Форма дохода облигации.
    #[prost(string, tag = "7")]
    pub interest_kind: ::prost::alloc::string::String,
    /// Количество выплат в год.
    #[prost(int32, tag = "8")]
    pub coupon_quantity_per_year: i32,
    /// Признак облигации с индексируемым номиналом.
    #[prost(bool, tag = "9")]
    pub indexed_nominal_flag: bool,
    /// Признак субординированной облигации.
    #[prost(bool, tag = "10")]
    pub subordinated_flag: bool,
    /// Признак обеспеченной облигации.
    #[prost(bool, tag = "11")]
    pub collateral_flag: bool,
    /// Признак показывает, что купоны облигации не облагаются налогом (для mass market).
    #[prost(bool, tag = "12")]
    pub tax_free_flag: bool,
    /// Признак облигации с амортизацией долга.
    #[prost(bool, tag = "13")]
    pub amortization_flag: bool,
    /// Признак облигации с плавающим купоном.
    #[prost(bool, tag = "14")]
    pub floating_coupon_flag: bool,
    /// Признак бессрочной облигации.
    #[prost(bool, tag = "15")]
    pub perpetual_flag: bool,
    /// Дата погашения облигации.
    #[prost(message, optional, tag = "16")]
    pub maturity_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Описание и условия получения дополнительного дохода.
    #[prost(string, tag = "17")]
    pub return_condition: ::prost::alloc::string::String,
    /// Дата выпуска облигации.
    #[prost(message, optional, tag = "18")]
    pub state_reg_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата размещения облигации.
    #[prost(message, optional, tag = "19")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена размещения облигации.
    #[prost(message, optional, tag = "20")]
    pub placement_price: ::core::option::Option<Quotation>,
    /// Объявленное количество шт.
    #[prost(message, optional, tag = "21")]
    pub issue_size_plan: ::core::option::Option<Quotation>,
}
/// Структурная нота.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetStructuredProduct {
    /// Наименование заемщика.
    #[prost(string, tag = "1")]
    pub borrow_name: ::prost::alloc::string::String,
    /// Номинал.
    #[prost(message, optional, tag = "2")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag = "3")]
    pub nominal_currency: ::prost::alloc::string::String,
    /// Тип структурной ноты.
    #[prost(enumeration = "StructuredProductType", tag = "4")]
    pub r#type: i32,
    /// Стратегия портфеля.
    #[prost(string, tag = "5")]
    pub logic_portfolio: ::prost::alloc::string::String,
    /// Тип базового актива.
    #[prost(enumeration = "AssetType", tag = "6")]
    pub asset_type: i32,
    /// Вид базового актива в зависимости от типа базового актива.
    #[prost(string, tag = "7")]
    pub basic_asset: ::prost::alloc::string::String,
    /// Барьер сохранности (в процентах).
    #[prost(message, optional, tag = "8")]
    pub safety_barrier: ::core::option::Option<Quotation>,
    /// Дата погашения.
    #[prost(message, optional, tag = "9")]
    pub maturity_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Объявленное количество шт.
    #[prost(message, optional, tag = "10")]
    pub issue_size_plan: ::core::option::Option<Quotation>,
    /// Объем размещения.
    #[prost(message, optional, tag = "11")]
    pub issue_size: ::core::option::Option<Quotation>,
    /// Дата размещения ноты.
    #[prost(message, optional, tag = "12")]
    pub placement_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Форма выпуска.
    #[prost(string, tag = "13")]
    pub issue_kind: ::prost::alloc::string::String,
}
/// Фонд.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetEtf {
    /// Суммарные расходы фонда (в %).
    #[prost(message, optional, tag = "1")]
    pub total_expense: ::core::option::Option<Quotation>,
    /// Барьерная ставка доходности после которой фонд имеет право на perfomance fee (в процентах).
    #[prost(message, optional, tag = "2")]
    pub hurdle_rate: ::core::option::Option<Quotation>,
    /// Комиссия за успешные результаты фонда (в процентах).
    #[prost(message, optional, tag = "3")]
    pub performance_fee: ::core::option::Option<Quotation>,
    /// Фиксированная комиссия за управление (в процентах).
    #[prost(message, optional, tag = "4")]
    pub fixed_commission: ::core::option::Option<Quotation>,
    /// Тип распределения доходов от выплат по бумагам.
    #[prost(string, tag = "5")]
    pub payment_type: ::prost::alloc::string::String,
    /// Признак необходимости выхода фонда в плюс для получения комиссии.
    #[prost(bool, tag = "6")]
    pub watermark_flag: bool,
    /// Премия (надбавка к цене) при покупке доли в фонде (в процентах).
    #[prost(message, optional, tag = "7")]
    pub buy_premium: ::core::option::Option<Quotation>,
    /// Ставка дисконта (вычет из цены) при продаже доли в фонде (в процентах).
    #[prost(message, optional, tag = "8")]
    pub sell_discount: ::core::option::Option<Quotation>,
    /// Признак ребалансируемости портфеля фонда.
    #[prost(bool, tag = "9")]
    pub rebalancing_flag: bool,
    /// Периодичность ребалансировки.
    #[prost(string, tag = "10")]
    pub rebalancing_freq: ::prost::alloc::string::String,
    /// Тип управления.
    #[prost(string, tag = "11")]
    pub management_type: ::prost::alloc::string::String,
    /// Индекс, который реплицирует (старается копировать) фонд.
    #[prost(string, tag = "12")]
    pub primary_index: ::prost::alloc::string::String,
    /// База ETF.
    #[prost(string, tag = "13")]
    pub focus_type: ::prost::alloc::string::String,
    /// Признак использования заемных активов (плечо).
    #[prost(bool, tag = "14")]
    pub leveraged_flag: bool,
    /// Количество акций в обращении.
    #[prost(message, optional, tag = "15")]
    pub num_share: ::core::option::Option<Quotation>,
    /// Признак обязательства по отчетности перед регулятором.
    #[prost(bool, tag = "16")]
    pub ucits_flag: bool,
    /// Дата выпуска.
    #[prost(message, optional, tag = "17")]
    pub released_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Описание фонда.
    #[prost(string, tag = "18")]
    pub description: ::prost::alloc::string::String,
    /// Описание индекса, за которым следует фонд.
    #[prost(string, tag = "19")]
    pub primary_index_description: ::prost::alloc::string::String,
    /// Основные компании, в которые вкладывается фонд.
    #[prost(string, tag = "20")]
    pub primary_index_company: ::prost::alloc::string::String,
    /// Срок восстановления индекса (после просадки).
    #[prost(message, optional, tag = "21")]
    pub index_recovery_period: ::core::option::Option<Quotation>,
    /// IVAV-код.
    #[prost(string, tag = "22")]
    pub inav_code: ::prost::alloc::string::String,
    /// Признак наличия дивидендной доходности.
    #[prost(bool, tag = "23")]
    pub div_yield_flag: bool,
    /// Комиссия на покрытие расходов фонда (в процентах).
    #[prost(message, optional, tag = "24")]
    pub expense_commission: ::core::option::Option<Quotation>,
    /// Ошибка следования за индексом (в процентах).
    #[prost(message, optional, tag = "25")]
    pub primary_index_tracking_error: ::core::option::Option<Quotation>,
    /// Плановая ребалансировка портфеля.
    #[prost(string, tag = "26")]
    pub rebalancing_plan: ::prost::alloc::string::String,
    /// Ставки налогообложения дивидендов и купонов.
    #[prost(string, tag = "27")]
    pub tax_rate: ::prost::alloc::string::String,
    /// Даты ребалансировок.
    #[prost(message, repeated, tag = "28")]
    pub rebalancing_dates: ::prost::alloc::vec::Vec<::prost_types::Timestamp>,
    /// Форма выпуска.
    #[prost(string, tag = "29")]
    pub issue_kind: ::prost::alloc::string::String,
    /// Номинал.
    #[prost(message, optional, tag = "30")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag = "31")]
    pub nominal_currency: ::prost::alloc::string::String,
}
/// Клиринговый сертификат участия.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetClearingCertificate {
    /// Номинал.
    #[prost(message, optional, tag = "1")]
    pub nominal: ::core::option::Option<Quotation>,
    /// Валюта номинала.
    #[prost(string, tag = "2")]
    pub nominal_currency: ::prost::alloc::string::String,
}
/// Бренд.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Brand {
    /// uid идентификатор бренда.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    /// Наименование бренда.
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// Описание.
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    /// Информация о бренде.
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    /// Компания.
    #[prost(string, tag = "5")]
    pub company: ::prost::alloc::string::String,
    /// Сектор.
    #[prost(string, tag = "6")]
    pub sector: ::prost::alloc::string::String,
    /// Код страны риска.
    #[prost(string, tag = "7")]
    pub country_of_risk: ::prost::alloc::string::String,
    /// Наименование страны риска.
    #[prost(string, tag = "8")]
    pub country_of_risk_name: ::prost::alloc::string::String,
}
/// Идентификаторы инструмента.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetInstrument {
    /// uid идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub uid: ::prost::alloc::string::String,
    /// figi идентификатор инструмента.
    #[prost(string, tag = "2")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "3")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "4")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код (секция торгов).
    #[prost(string, tag = "5")]
    pub class_code: ::prost::alloc::string::String,
    /// Массив связанных инструментов.
    #[prost(message, repeated, tag = "6")]
    pub links: ::prost::alloc::vec::Vec<InstrumentLink>,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "10")]
    pub instrument_kind: i32,
    /// id позиции.
    #[prost(string, tag = "11")]
    pub position_uid: ::prost::alloc::string::String,
}
/// Связь с другим инструментом.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentLink {
    /// Тип связи.
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// uid идентификатор связанного инструмента.
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос списка избранных инструментов, входные параметры не требуются.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFavoritesRequest {}
/// В ответ передаётся список избранных инструментов в качестве массива.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetFavoritesResponse {
    /// Массив инструментов
    #[prost(message, repeated, tag = "1")]
    pub favorite_instruments: ::prost::alloc::vec::Vec<FavoriteInstrument>,
}
/// Массив избранных инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FavoriteInstrument {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тикер инструмента.
    #[prost(string, tag = "2")]
    pub ticker: ::prost::alloc::string::String,
    /// Класс-код инструмента.
    #[prost(string, tag = "3")]
    pub class_code: ::prost::alloc::string::String,
    /// Isin-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "11")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Признак внебиржевой ценной бумаги.
    #[prost(bool, tag = "16")]
    pub otc_flag: bool,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "17")]
    pub api_trade_available_flag: bool,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "18")]
    pub instrument_kind: i32,
}
/// Запрос редактирования списка избранных инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFavoritesRequest {
    /// Массив инструментов.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<EditFavoritesRequestInstrument>,
    /// Тип действия со списком.
    #[prost(enumeration = "EditFavoritesActionType", tag = "6")]
    pub action_type: i32,
}
/// Массив инструментов для редактирования списка избранных инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFavoritesRequestInstrument {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
}
/// Результат редактирования списка избранных инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFavoritesResponse {
    /// Массив инструментов
    #[prost(message, repeated, tag = "1")]
    pub favorite_instruments: ::prost::alloc::vec::Vec<FavoriteInstrument>,
}
/// Запрос справочника стран.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCountriesRequest {}
/// Справочник стран.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCountriesResponse {
    /// Массив стран.
    #[prost(message, repeated, tag = "1")]
    pub countries: ::prost::alloc::vec::Vec<CountryResponse>,
}
/// Данные о стране.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountryResponse {
    /// Двухбуквенный код страны.
    #[prost(string, tag = "1")]
    pub alfa_two: ::prost::alloc::string::String,
    /// Трёхбуквенный код страны.
    #[prost(string, tag = "2")]
    pub alfa_three: ::prost::alloc::string::String,
    /// Наименование страны.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Краткое наименование страны.
    #[prost(string, tag = "4")]
    pub name_brief: ::prost::alloc::string::String,
}
/// Запрос на поиск инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindInstrumentRequest {
    /// Строка поиска.
    #[prost(string, tag = "1")]
    pub query: ::prost::alloc::string::String,
    /// Фильтр по типу инструмента.
    #[prost(enumeration = "InstrumentType", tag = "2")]
    pub instrument_kind: i32,
    /// Фильтр для отображения только торговых инструментов.
    #[prost(bool, tag = "3")]
    pub api_trade_available_flag: bool,
}
/// Результат поиска инструментов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FindInstrumentResponse {
    /// Массив инструментов, удовлетворяющих условиям поиска.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<InstrumentShort>,
}
/// Краткая информация об инструменте.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentShort {
    /// Isin инструмента.
    #[prost(string, tag = "1")]
    pub isin: ::prost::alloc::string::String,
    /// Figi инструмента.
    #[prost(string, tag = "2")]
    pub figi: ::prost::alloc::string::String,
    /// Ticker инструмента.
    #[prost(string, tag = "3")]
    pub ticker: ::prost::alloc::string::String,
    /// ClassCode инструмента.
    #[prost(string, tag = "4")]
    pub class_code: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "5")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Название инструмента.
    #[prost(string, tag = "6")]
    pub name: ::prost::alloc::string::String,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "7")]
    pub uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор позиции инструмента.
    #[prost(string, tag = "8")]
    pub position_uid: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "10")]
    pub instrument_kind: i32,
    /// Параметр указывает на возможность торговать инструментом через API.
    #[prost(bool, tag = "11")]
    pub api_trade_available_flag: bool,
    /// Признак доступности для ИИС.
    #[prost(bool, tag = "12")]
    pub for_iis_flag: bool,
    /// Дата первой минутной свечи.
    #[prost(message, optional, tag = "26")]
    pub first_1min_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата первой дневной свечи.
    #[prost(message, optional, tag = "27")]
    pub first_1day_candle_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Флаг отображающий доступность торговли инструментом только для квалифицированных инвесторов.
    #[prost(bool, tag = "28")]
    pub for_qual_investor_flag: bool,
    /// Флаг отображающий доступность торговли инструментом по выходным
    #[prost(bool, tag = "29")]
    pub weekend_flag: bool,
    /// Флаг заблокированного ТКС
    #[prost(bool, tag = "30")]
    pub blocked_tca_flag: bool,
}
/// Запрос списка брендов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandsRequest {}
/// Запрос бренда.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandRequest {
    /// Uid-идентификатор бренда.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
/// Список брендов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrandsResponse {
    /// Массив брендов.
    #[prost(message, repeated, tag = "1")]
    pub brands: ::prost::alloc::vec::Vec<Brand>,
}
/// Тип купонов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CouponType {
    /// Неопределенное значение
    Unspecified = 0,
    /// Постоянный
    Constant = 1,
    /// Плавающий
    Floating = 2,
    /// Дисконт
    Discount = 3,
    /// Ипотечный
    Mortgage = 4,
    /// Фиксированный
    Fix = 5,
    /// Переменный
    Variable = 6,
    /// Прочее
    Other = 7,
}
impl CouponType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CouponType::Unspecified => "COUPON_TYPE_UNSPECIFIED",
            CouponType::Constant => "COUPON_TYPE_CONSTANT",
            CouponType::Floating => "COUPON_TYPE_FLOATING",
            CouponType::Discount => "COUPON_TYPE_DISCOUNT",
            CouponType::Mortgage => "COUPON_TYPE_MORTGAGE",
            CouponType::Fix => "COUPON_TYPE_FIX",
            CouponType::Variable => "COUPON_TYPE_VARIABLE",
            CouponType::Other => "COUPON_TYPE_OTHER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COUPON_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "COUPON_TYPE_CONSTANT" => Some(Self::Constant),
            "COUPON_TYPE_FLOATING" => Some(Self::Floating),
            "COUPON_TYPE_DISCOUNT" => Some(Self::Discount),
            "COUPON_TYPE_MORTGAGE" => Some(Self::Mortgage),
            "COUPON_TYPE_FIX" => Some(Self::Fix),
            "COUPON_TYPE_VARIABLE" => Some(Self::Variable),
            "COUPON_TYPE_OTHER" => Some(Self::Other),
            _ => None,
        }
    }
}
/// Тип опциона по направлению сделки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionDirection {
    /// Тип не определен.
    Unspecified = 0,
    /// Опцион на продажу.
    Put = 1,
    /// Опцион на покупку.
    Call = 2,
}
impl OptionDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionDirection::Unspecified => "OPTION_DIRECTION_UNSPECIFIED",
            OptionDirection::Put => "OPTION_DIRECTION_PUT",
            OptionDirection::Call => "OPTION_DIRECTION_CALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPTION_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "OPTION_DIRECTION_PUT" => Some(Self::Put),
            "OPTION_DIRECTION_CALL" => Some(Self::Call),
            _ => None,
        }
    }
}
/// Тип расчетов по опциону.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionPaymentType {
    /// Тип не определен.
    Unspecified = 0,
    /// Опционы с использованием премии в расчетах.
    Premium = 1,
    /// Маржируемые опционы.
    Marginal = 2,
}
impl OptionPaymentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionPaymentType::Unspecified => "OPTION_PAYMENT_TYPE_UNSPECIFIED",
            OptionPaymentType::Premium => "OPTION_PAYMENT_TYPE_PREMIUM",
            OptionPaymentType::Marginal => "OPTION_PAYMENT_TYPE_MARGINAL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPTION_PAYMENT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPTION_PAYMENT_TYPE_PREMIUM" => Some(Self::Premium),
            "OPTION_PAYMENT_TYPE_MARGINAL" => Some(Self::Marginal),
            _ => None,
        }
    }
}
/// Тип опциона по стилю.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionStyle {
    /// Тип не определен.
    Unspecified = 0,
    /// Американский опцион.
    American = 1,
    /// Европейский опцион.
    European = 2,
}
impl OptionStyle {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionStyle::Unspecified => "OPTION_STYLE_UNSPECIFIED",
            OptionStyle::American => "OPTION_STYLE_AMERICAN",
            OptionStyle::European => "OPTION_STYLE_EUROPEAN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPTION_STYLE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPTION_STYLE_AMERICAN" => Some(Self::American),
            "OPTION_STYLE_EUROPEAN" => Some(Self::European),
            _ => None,
        }
    }
}
/// Тип опциона по способу исполнения.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OptionSettlementType {
    /// Тип не определен.
    OptionExecutionTypeUnspecified = 0,
    /// Поставочный тип опциона.
    OptionExecutionTypePhysicalDelivery = 1,
    /// Расчетный тип опциона.
    OptionExecutionTypeCashSettlement = 2,
}
impl OptionSettlementType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OptionSettlementType::OptionExecutionTypeUnspecified => {
                "OPTION_EXECUTION_TYPE_UNSPECIFIED"
            }
            OptionSettlementType::OptionExecutionTypePhysicalDelivery => {
                "OPTION_EXECUTION_TYPE_PHYSICAL_DELIVERY"
            }
            OptionSettlementType::OptionExecutionTypeCashSettlement => {
                "OPTION_EXECUTION_TYPE_CASH_SETTLEMENT"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPTION_EXECUTION_TYPE_UNSPECIFIED" => {
                Some(Self::OptionExecutionTypeUnspecified)
            }
            "OPTION_EXECUTION_TYPE_PHYSICAL_DELIVERY" => {
                Some(Self::OptionExecutionTypePhysicalDelivery)
            }
            "OPTION_EXECUTION_TYPE_CASH_SETTLEMENT" => {
                Some(Self::OptionExecutionTypeCashSettlement)
            }
            _ => None,
        }
    }
}
/// Тип идентификатора инструмента. Подробнее об идентификации инструментов: [Идентификация инструментов](<https://tinkoff.github.io/investAPI/faq_identification/>)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentIdType {
    /// Значение не определено.
    InstrumentIdUnspecified = 0,
    /// Figi.
    Figi = 1,
    /// Ticker.
    Ticker = 2,
    /// Уникальный идентификатор.
    Uid = 3,
    /// Идентификатор позиции.
    PositionUid = 4,
}
impl InstrumentIdType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstrumentIdType::InstrumentIdUnspecified => "INSTRUMENT_ID_UNSPECIFIED",
            InstrumentIdType::Figi => "INSTRUMENT_ID_TYPE_FIGI",
            InstrumentIdType::Ticker => "INSTRUMENT_ID_TYPE_TICKER",
            InstrumentIdType::Uid => "INSTRUMENT_ID_TYPE_UID",
            InstrumentIdType::PositionUid => "INSTRUMENT_ID_TYPE_POSITION_UID",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTRUMENT_ID_UNSPECIFIED" => Some(Self::InstrumentIdUnspecified),
            "INSTRUMENT_ID_TYPE_FIGI" => Some(Self::Figi),
            "INSTRUMENT_ID_TYPE_TICKER" => Some(Self::Ticker),
            "INSTRUMENT_ID_TYPE_UID" => Some(Self::Uid),
            "INSTRUMENT_ID_TYPE_POSITION_UID" => Some(Self::PositionUid),
            _ => None,
        }
    }
}
/// Статус запрашиваемых инструментов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InstrumentStatus {
    /// Значение не определено.
    Unspecified = 0,
    /// Базовый список инструментов (по умолчанию). Инструменты доступные для торговли через TINKOFF INVEST API.
    Base = 1,
    /// Список всех инструментов.
    All = 2,
}
impl InstrumentStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstrumentStatus::Unspecified => "INSTRUMENT_STATUS_UNSPECIFIED",
            InstrumentStatus::Base => "INSTRUMENT_STATUS_BASE",
            InstrumentStatus::All => "INSTRUMENT_STATUS_ALL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTRUMENT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "INSTRUMENT_STATUS_BASE" => Some(Self::Base),
            "INSTRUMENT_STATUS_ALL" => Some(Self::All),
            _ => None,
        }
    }
}
/// Тип акций.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ShareType {
    /// Значение не определено.
    Unspecified = 0,
    /// Обыкновенная
    Common = 1,
    /// Привилегированная
    Preferred = 2,
    /// Американские депозитарные расписки
    Adr = 3,
    /// Глобальные депозитарные расписки
    Gdr = 4,
    /// Товарищество с ограниченной ответственностью
    Mlp = 5,
    /// Акции из реестра Нью-Йорка
    NyRegShrs = 6,
    /// Закрытый инвестиционный фонд
    ClosedEndFund = 7,
    /// Траст недвижимости
    Reit = 8,
}
impl ShareType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ShareType::Unspecified => "SHARE_TYPE_UNSPECIFIED",
            ShareType::Common => "SHARE_TYPE_COMMON",
            ShareType::Preferred => "SHARE_TYPE_PREFERRED",
            ShareType::Adr => "SHARE_TYPE_ADR",
            ShareType::Gdr => "SHARE_TYPE_GDR",
            ShareType::Mlp => "SHARE_TYPE_MLP",
            ShareType::NyRegShrs => "SHARE_TYPE_NY_REG_SHRS",
            ShareType::ClosedEndFund => "SHARE_TYPE_CLOSED_END_FUND",
            ShareType::Reit => "SHARE_TYPE_REIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SHARE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "SHARE_TYPE_COMMON" => Some(Self::Common),
            "SHARE_TYPE_PREFERRED" => Some(Self::Preferred),
            "SHARE_TYPE_ADR" => Some(Self::Adr),
            "SHARE_TYPE_GDR" => Some(Self::Gdr),
            "SHARE_TYPE_MLP" => Some(Self::Mlp),
            "SHARE_TYPE_NY_REG_SHRS" => Some(Self::NyRegShrs),
            "SHARE_TYPE_CLOSED_END_FUND" => Some(Self::ClosedEndFund),
            "SHARE_TYPE_REIT" => Some(Self::Reit),
            _ => None,
        }
    }
}
/// Тип актива.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetType {
    /// Тип не определён.
    Unspecified = 0,
    /// Валюта.
    Currency = 1,
    /// Товар.
    Commodity = 2,
    /// Индекс.
    Index = 3,
    /// Ценная бумага.
    Security = 4,
}
impl AssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetType::Unspecified => "ASSET_TYPE_UNSPECIFIED",
            AssetType::Currency => "ASSET_TYPE_CURRENCY",
            AssetType::Commodity => "ASSET_TYPE_COMMODITY",
            AssetType::Index => "ASSET_TYPE_INDEX",
            AssetType::Security => "ASSET_TYPE_SECURITY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASSET_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ASSET_TYPE_CURRENCY" => Some(Self::Currency),
            "ASSET_TYPE_COMMODITY" => Some(Self::Commodity),
            "ASSET_TYPE_INDEX" => Some(Self::Index),
            "ASSET_TYPE_SECURITY" => Some(Self::Security),
            _ => None,
        }
    }
}
/// Тип структурной ноты.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StructuredProductType {
    /// Тип не определён.
    SpTypeUnspecified = 0,
    /// Поставочный.
    SpTypeDeliverable = 1,
    /// Беспоставочный.
    SpTypeNonDeliverable = 2,
}
impl StructuredProductType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StructuredProductType::SpTypeUnspecified => "SP_TYPE_UNSPECIFIED",
            StructuredProductType::SpTypeDeliverable => "SP_TYPE_DELIVERABLE",
            StructuredProductType::SpTypeNonDeliverable => "SP_TYPE_NON_DELIVERABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SP_TYPE_UNSPECIFIED" => Some(Self::SpTypeUnspecified),
            "SP_TYPE_DELIVERABLE" => Some(Self::SpTypeDeliverable),
            "SP_TYPE_NON_DELIVERABLE" => Some(Self::SpTypeNonDeliverable),
            _ => None,
        }
    }
}
/// Тип действия со списком избранных инструментов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EditFavoritesActionType {
    /// Тип не определён.
    Unspecified = 0,
    /// Добавить в список.
    Add = 1,
    /// Удалить из списка.
    Del = 2,
}
impl EditFavoritesActionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EditFavoritesActionType::Unspecified => {
                "EDIT_FAVORITES_ACTION_TYPE_UNSPECIFIED"
            }
            EditFavoritesActionType::Add => "EDIT_FAVORITES_ACTION_TYPE_ADD",
            EditFavoritesActionType::Del => "EDIT_FAVORITES_ACTION_TYPE_DEL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EDIT_FAVORITES_ACTION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "EDIT_FAVORITES_ACTION_TYPE_ADD" => Some(Self::Add),
            "EDIT_FAVORITES_ACTION_TYPE_DEL" => Some(Self::Del),
            _ => None,
        }
    }
}
/// Реальная площадка исполнения расчётов.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RealExchange {
    /// Тип не определён.
    Unspecified = 0,
    /// Московская биржа.
    Moex = 1,
    /// Санкт-Петербургская биржа.
    Rts = 2,
    /// Внебиржевой инструмент.
    Otc = 3,
}
impl RealExchange {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RealExchange::Unspecified => "REAL_EXCHANGE_UNSPECIFIED",
            RealExchange::Moex => "REAL_EXCHANGE_MOEX",
            RealExchange::Rts => "REAL_EXCHANGE_RTS",
            RealExchange::Otc => "REAL_EXCHANGE_OTC",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REAL_EXCHANGE_UNSPECIFIED" => Some(Self::Unspecified),
            "REAL_EXCHANGE_MOEX" => Some(Self::Moex),
            "REAL_EXCHANGE_RTS" => Some(Self::Rts),
            "REAL_EXCHANGE_OTC" => Some(Self::Otc),
            _ => None,
        }
    }
}
/// Уровень риска облигации.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RiskLevel {
    Unspecified = 0,
    /// Низкий уровень риска
    Low = 1,
    /// Средний уровень риска
    Moderate = 2,
    /// Высокий уровень риска
    High = 3,
}
impl RiskLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RiskLevel::Unspecified => "RISK_LEVEL_UNSPECIFIED",
            RiskLevel::Low => "RISK_LEVEL_LOW",
            RiskLevel::Moderate => "RISK_LEVEL_MODERATE",
            RiskLevel::High => "RISK_LEVEL_HIGH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RISK_LEVEL_UNSPECIFIED" => Some(Self::Unspecified),
            "RISK_LEVEL_LOW" => Some(Self::Low),
            "RISK_LEVEL_MODERATE" => Some(Self::Moderate),
            "RISK_LEVEL_HIGH" => Some(Self::High),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod instruments_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct InstrumentsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InstrumentsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InstrumentsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InstrumentsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            InstrumentsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Метод получения расписания торгов торговых площадок.
        pub async fn trading_schedules(
            &mut self,
            request: impl tonic::IntoRequest<super::TradingSchedulesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::TradingSchedulesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/TradingSchedules",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "TradingSchedules",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения облигации по её идентификатору.
        pub async fn bond_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> std::result::Result<tonic::Response<super::BondResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/BondBy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "BondBy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка облигаций.
        pub async fn bonds(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> std::result::Result<tonic::Response<super::BondsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Bonds",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "Bonds",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения графика выплат купонов по облигации.
        pub async fn get_bond_coupons(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBondCouponsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBondCouponsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetBondCoupons",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "GetBondCoupons",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения валюты по её идентификатору.
        pub async fn currency_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CurrencyResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/CurrencyBy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "CurrencyBy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка валют.
        pub async fn currencies(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CurrenciesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Currencies",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "Currencies",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения инвестиционного фонда по его идентификатору.
        pub async fn etf_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> std::result::Result<tonic::Response<super::EtfResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/EtfBy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "EtfBy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка инвестиционных фондов.
        pub async fn etfs(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> std::result::Result<tonic::Response<super::EtfsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Etfs",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "Etfs",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения фьючерса по его идентификатору.
        pub async fn future_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> std::result::Result<tonic::Response<super::FutureResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/FutureBy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "FutureBy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка фьючерсов.
        pub async fn futures(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FuturesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Futures",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "Futures",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения опциона по его идентификатору.
        pub async fn option_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> std::result::Result<tonic::Response<super::OptionResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/OptionBy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "OptionBy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Deprecated Метод получения списка опционов.
        pub async fn options(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OptionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Options",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "Options",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка опционов.
        pub async fn options_by(
            &mut self,
            request: impl tonic::IntoRequest<super::FilterOptionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OptionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/OptionsBy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "OptionsBy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения акции по её идентификатору.
        pub async fn share_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> std::result::Result<tonic::Response<super::ShareResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/ShareBy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "ShareBy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка акций.
        pub async fn shares(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentsRequest>,
        ) -> std::result::Result<tonic::Response<super::SharesResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/Shares",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "Shares",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения накопленного купонного дохода по облигации.
        pub async fn get_accrued_interests(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccruedInterestsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccruedInterestsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAccruedInterests",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "GetAccruedInterests",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения размера гарантийного обеспечения по фьючерсам.
        pub async fn get_futures_margin(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFuturesMarginRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFuturesMarginResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetFuturesMargin",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "GetFuturesMargin",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения основной информации об инструменте.
        pub async fn get_instrument_by(
            &mut self,
            request: impl tonic::IntoRequest<super::InstrumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::InstrumentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetInstrumentBy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "GetInstrumentBy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод для получения событий выплаты дивидендов по инструменту.
        pub async fn get_dividends(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDividendsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDividendsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetDividends",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "GetDividends",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения актива по его идентификатору.
        pub async fn get_asset_by(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetRequest>,
        ) -> std::result::Result<tonic::Response<super::AssetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAssetBy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "GetAssetBy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка активов. Метод работает для всех инструментов, за исключением срочных - опционов и фьючерсов.
        pub async fn get_assets(
            &mut self,
            request: impl tonic::IntoRequest<super::AssetsRequest>,
        ) -> std::result::Result<tonic::Response<super::AssetsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetAssets",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "GetAssets",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка избранных инструментов.
        pub async fn get_favorites(
            &mut self,
            request: impl tonic::IntoRequest<super::GetFavoritesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetFavoritesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetFavorites",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "GetFavorites",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод редактирования списка избранных инструментов.
        pub async fn edit_favorites(
            &mut self,
            request: impl tonic::IntoRequest<super::EditFavoritesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::EditFavoritesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/EditFavorites",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "EditFavorites",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка стран.
        pub async fn get_countries(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCountriesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCountriesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetCountries",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "GetCountries",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод поиска инструмента.
        pub async fn find_instrument(
            &mut self,
            request: impl tonic::IntoRequest<super::FindInstrumentRequest>,
        ) -> std::result::Result<
            tonic::Response<super::FindInstrumentResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/FindInstrument",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "FindInstrument",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка брендов.
        pub async fn get_brands(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBrandsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetBrandsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetBrands",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "GetBrands",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения бренда по его идентификатору.
        pub async fn get_brand_by(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBrandRequest>,
        ) -> std::result::Result<tonic::Response<super::Brand>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.InstrumentsService/GetBrandBy",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.InstrumentsService",
                        "GetBrandBy",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Запрос подписки или отписки на определённые биржевые данные.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataRequest {
    #[prost(oneof = "market_data_request::Payload", tags = "1, 2, 3, 4, 5, 6")]
    pub payload: ::core::option::Option<market_data_request::Payload>,
}
/// Nested message and enum types in `MarketDataRequest`.
pub mod market_data_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Запрос подписки на свечи.
        #[prost(message, tag = "1")]
        SubscribeCandlesRequest(super::SubscribeCandlesRequest),
        /// Запрос подписки на стаканы.
        #[prost(message, tag = "2")]
        SubscribeOrderBookRequest(super::SubscribeOrderBookRequest),
        /// Запрос подписки на ленту обезличенных сделок.
        #[prost(message, tag = "3")]
        SubscribeTradesRequest(super::SubscribeTradesRequest),
        /// Запрос подписки на торговые статусы инструментов.
        #[prost(message, tag = "4")]
        SubscribeInfoRequest(super::SubscribeInfoRequest),
        /// Запрос подписки на цены последних сделок.
        #[prost(message, tag = "5")]
        SubscribeLastPriceRequest(super::SubscribeLastPriceRequest),
        /// Запрос своих подписок.
        #[prost(message, tag = "6")]
        GetMySubscriptions(super::GetMySubscriptions),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataServerSideStreamRequest {
    /// Запрос подписки на свечи.
    #[prost(message, optional, tag = "1")]
    pub subscribe_candles_request: ::core::option::Option<SubscribeCandlesRequest>,
    /// Запрос подписки на стаканы.
    #[prost(message, optional, tag = "2")]
    pub subscribe_order_book_request: ::core::option::Option<SubscribeOrderBookRequest>,
    /// Запрос подписки на ленту обезличенных сделок.
    #[prost(message, optional, tag = "3")]
    pub subscribe_trades_request: ::core::option::Option<SubscribeTradesRequest>,
    /// Запрос подписки на торговые статусы инструментов.
    #[prost(message, optional, tag = "4")]
    pub subscribe_info_request: ::core::option::Option<SubscribeInfoRequest>,
    /// Запрос подписки на цены последних сделок.
    #[prost(message, optional, tag = "5")]
    pub subscribe_last_price_request: ::core::option::Option<SubscribeLastPriceRequest>,
}
/// Пакет биржевой информации по подписке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MarketDataResponse {
    #[prost(
        oneof = "market_data_response::Payload",
        tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11"
    )]
    pub payload: ::core::option::Option<market_data_response::Payload>,
}
/// Nested message and enum types in `MarketDataResponse`.
pub mod market_data_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Результат подписки на свечи.
        #[prost(message, tag = "1")]
        SubscribeCandlesResponse(super::SubscribeCandlesResponse),
        /// Результат подписки на стаканы.
        #[prost(message, tag = "2")]
        SubscribeOrderBookResponse(super::SubscribeOrderBookResponse),
        /// Результат подписки на поток обезличенных сделок.
        #[prost(message, tag = "3")]
        SubscribeTradesResponse(super::SubscribeTradesResponse),
        /// Результат подписки на торговые статусы инструментов.
        #[prost(message, tag = "4")]
        SubscribeInfoResponse(super::SubscribeInfoResponse),
        /// Свеча.
        #[prost(message, tag = "5")]
        Candle(super::Candle),
        /// Сделки.
        #[prost(message, tag = "6")]
        Trade(super::Trade),
        /// Стакан.
        #[prost(message, tag = "7")]
        Orderbook(super::OrderBook),
        /// Торговый статус.
        #[prost(message, tag = "8")]
        TradingStatus(super::TradingStatus),
        /// Проверка активности стрима.
        #[prost(message, tag = "9")]
        Ping(super::Ping),
        /// Результат подписки на цены последние сделок по инструментам.
        #[prost(message, tag = "10")]
        SubscribeLastPriceResponse(super::SubscribeLastPriceResponse),
        /// Цена последней сделки.
        #[prost(message, tag = "11")]
        LastPrice(super::LastPrice),
    }
}
/// subscribeCandles | Изменения статуса подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCandlesRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на свечи.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<CandleInstrument>,
    /// Флаг ожидания закрытия временного интервала для отправки свечи, применяется только для минутных свечей.
    #[prost(bool, tag = "3")]
    pub waiting_close: bool,
}
/// Запрос изменения статус подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandleInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечей.
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "3")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статус подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeCandlesResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на свечи.
    #[prost(message, repeated, tag = "2")]
    pub candles_subscriptions: ::prost::alloc::vec::Vec<CandleSubscription>,
}
/// Статус подписки на свечи.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CandleSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечей.
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "3")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "4")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос на изменение статуса подписки на стаканы.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOrderBookRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на стаканы.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<OrderBookInstrument>,
}
/// Запрос подписки на стаканы.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "3")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на стаканы.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeOrderBookResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на стаканы.
    #[prost(message, repeated, tag = "2")]
    pub order_book_subscriptions: ::prost::alloc::vec::Vec<OrderBookSubscription>,
}
/// Статус подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBookSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "3")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "4")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Изменение статуса подписки на поток обезличенных сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTradesRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на поток обезличенных сделок.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<TradeInstrument>,
}
/// Запрос подписки на поток обезличенных сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на поток обезличенных сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeTradesResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на поток сделок.
    #[prost(message, repeated, tag = "2")]
    pub trade_subscriptions: ::prost::alloc::vec::Vec<TradeSubscription>,
}
/// Статус подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "3")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Изменение статуса подписки на торговый статус инструмента.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInfoRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на торговый статус.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<InfoInstrument>,
}
/// Запрос подписки на торговый статус.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на торговый статус.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeInfoResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на торговый статус.
    #[prost(message, repeated, tag = "2")]
    pub info_subscriptions: ::prost::alloc::vec::Vec<InfoSubscription>,
}
/// Статус подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "3")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Изменение статуса подписки на цену последней сделки по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLastPriceRequest {
    /// Изменение статуса подписки.
    #[prost(enumeration = "SubscriptionAction", tag = "1")]
    pub subscription_action: i32,
    /// Массив инструментов для подписки на цену последней сделки.
    #[prost(message, repeated, tag = "2")]
    pub instruments: ::prost::alloc::vec::Vec<LastPriceInstrument>,
}
/// Запрос подписки на последнюю цену.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceInstrument {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат изменения статуса подписки на цену последней сделки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubscribeLastPriceResponse {
    /// Уникальный идентификатор запроса, подробнее: \[tracking_id\](<https://tinkoff.github.io/investAPI/grpc#tracking-id>).
    #[prost(string, tag = "1")]
    pub tracking_id: ::prost::alloc::string::String,
    /// Массив статусов подписки на цену последней сделки.
    #[prost(message, repeated, tag = "2")]
    pub last_price_subscriptions: ::prost::alloc::vec::Vec<LastPriceSubscription>,
}
/// Статус подписки на цену последней сделки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPriceSubscription {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус подписки.
    #[prost(enumeration = "SubscriptionStatus", tag = "2")]
    pub subscription_status: i32,
    /// Uid инструмента
    #[prost(string, tag = "3")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Пакет свечей в рамках стрима.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Candle {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Интервал свечи.
    #[prost(enumeration = "SubscriptionInterval", tag = "2")]
    pub interval: i32,
    /// Цена открытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "3")]
    pub open: ::core::option::Option<Quotation>,
    /// Максимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "4")]
    pub high: ::core::option::Option<Quotation>,
    /// Минимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "5")]
    pub low: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "6")]
    pub close: ::core::option::Option<Quotation>,
    /// Объём сделок в лотах.
    #[prost(int64, tag = "7")]
    pub volume: i64,
    /// Время начала интервала свечи в часовом поясе UTC.
    #[prost(message, optional, tag = "8")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Время последней сделки, вошедшей в свечу в часовом поясе UTC.
    #[prost(message, optional, tag = "9")]
    pub last_trade_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag = "10")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Пакет стаканов в рамках стрима.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderBook {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Флаг консистентности стакана. **false** значит не все заявки попали в стакан по причинам сетевых задержек или нарушения порядка доставки.
    #[prost(bool, tag = "3")]
    pub is_consistent: bool,
    /// Массив предложений.
    #[prost(message, repeated, tag = "4")]
    pub bids: ::prost::alloc::vec::Vec<Order>,
    /// Массив спроса.
    #[prost(message, repeated, tag = "5")]
    pub asks: ::prost::alloc::vec::Vec<Order>,
    /// Время формирования стакана в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Верхний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "7")]
    pub limit_up: ::core::option::Option<Quotation>,
    /// Нижний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "8")]
    pub limit_down: ::core::option::Option<Quotation>,
    /// Uid инструмента
    #[prost(string, tag = "9")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Массив предложений/спроса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Order {
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество в лотах.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
}
/// Информация о сделке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trade {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Направление сделки.
    #[prost(enumeration = "TradeDirection", tag = "2")]
    pub direction: i32,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество лотов.
    #[prost(int64, tag = "4")]
    pub quantity: i64,
    /// Время сделки в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag = "5")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag = "6")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Пакет изменения торгового статуса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradingStatus {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус торговли инструментом.
    #[prost(enumeration = "SecurityTradingStatus", tag = "2")]
    pub trading_status: i32,
    /// Время изменения торгового статуса в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак доступности выставления лимитной заявки по инструменту.
    #[prost(bool, tag = "4")]
    pub limit_order_available_flag: bool,
    /// Признак доступности выставления рыночной заявки по инструменту.
    #[prost(bool, tag = "5")]
    pub market_order_available_flag: bool,
    /// Uid инструмента
    #[prost(string, tag = "6")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос исторических свечей.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Интервал запрошенных свечей.
    #[prost(enumeration = "CandleInterval", tag = "4")]
    pub interval: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, tag = "5")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Список свечей.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCandlesResponse {
    /// Массив свечей.
    #[prost(message, repeated, tag = "1")]
    pub candles: ::prost::alloc::vec::Vec<HistoricCandle>,
}
/// Информация о свече.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HistoricCandle {
    /// Цена открытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "1")]
    pub open: ::core::option::Option<Quotation>,
    /// Максимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "2")]
    pub high: ::core::option::Option<Quotation>,
    /// Минимальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "3")]
    pub low: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "4")]
    pub close: ::core::option::Option<Quotation>,
    /// Объём торгов в лотах.
    #[prost(int64, tag = "5")]
    pub volume: i64,
    /// Время свечи в часовом поясе UTC.
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Признак завершённости свечи. **false** значит, свеча за текущие интервал ещё сформирована не полностью.
    #[prost(bool, tag = "7")]
    pub is_complete: bool,
}
/// Запрос получения цен последних сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPricesRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, repeated, tag = "1")]
    pub figi: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Массив идентификаторов инструмента, принимает значения figi или instrument_uid.
    #[prost(string, repeated, tag = "2")]
    pub instrument_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Список цен последних сделок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastPricesResponse {
    /// Массив цен последних сделок.
    #[prost(message, repeated, tag = "1")]
    pub last_prices: ::prost::alloc::vec::Vec<LastPrice>,
}
/// Информация о цене последней сделки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LastPrice {
    /// Figi инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Цена последней сделки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Quotation>,
    /// Время получения последней цены в часовом поясе UTC по времени биржи.
    #[prost(message, optional, tag = "3")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента
    #[prost(string, tag = "11")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос стакана.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderBookRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, tag = "3")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Информация о стакане.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderBookResponse {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Глубина стакана.
    #[prost(int32, tag = "2")]
    pub depth: i32,
    /// Множество пар значений на покупку.
    #[prost(message, repeated, tag = "3")]
    pub bids: ::prost::alloc::vec::Vec<Order>,
    /// Множество пар значений на продажу.
    #[prost(message, repeated, tag = "4")]
    pub asks: ::prost::alloc::vec::Vec<Order>,
    /// Цена последней сделки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "5")]
    pub last_price: ::core::option::Option<Quotation>,
    /// Цена закрытия за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "6")]
    pub close_price: ::core::option::Option<Quotation>,
    /// Верхний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "7")]
    pub limit_up: ::core::option::Option<Quotation>,
    /// Нижний лимит цены за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Для перевод цен в валюту рекомендуем использовать [информацию со страницы](<https://tinkoff.github.io/investAPI/faq_marketdata/>)
    #[prost(message, optional, tag = "8")]
    pub limit_down: ::core::option::Option<Quotation>,
    /// Время получения цены последней сделки.
    #[prost(message, optional, tag = "21")]
    pub last_price_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Время получения цены закрытия.
    #[prost(message, optional, tag = "22")]
    pub close_price_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Время формирования стакана на бирже.
    #[prost(message, optional, tag = "23")]
    pub orderbook_ts: ::core::option::Option<::prost_types::Timestamp>,
    /// Uid инструмента.
    #[prost(string, tag = "9")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос получения торгового статуса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Запрос получения торгового статуса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusesRequest {
    /// Идентификатор инструмента, принимает значение figi или instrument_uid
    #[prost(string, repeated, tag = "1")]
    pub instrument_id: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация о торговом статусе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusesResponse {
    /// Массив информации о торговых статусах
    #[prost(message, repeated, tag = "1")]
    pub trading_statuses: ::prost::alloc::vec::Vec<GetTradingStatusResponse>,
}
/// Информация о торговом статусе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTradingStatusResponse {
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Статус торговли инструментом.
    #[prost(enumeration = "SecurityTradingStatus", tag = "2")]
    pub trading_status: i32,
    /// Признак доступности выставления лимитной заявки по инструменту.
    #[prost(bool, tag = "3")]
    pub limit_order_available_flag: bool,
    /// Признак доступности выставления рыночной заявки по инструменту.
    #[prost(bool, tag = "4")]
    pub market_order_available_flag: bool,
    /// Признак доступности торгов через API.
    #[prost(bool, tag = "5")]
    pub api_trade_available_flag: bool,
    /// Uid инструмента.
    #[prost(string, tag = "6")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос обезличенных сделок за последний час.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastTradesRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Начало запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание запрашиваемого периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, tag = "4")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Обезличенных сделок за последний час.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLastTradesResponse {
    /// Массив сделок.
    #[prost(message, repeated, tag = "1")]
    pub trades: ::prost::alloc::vec::Vec<Trade>,
}
/// Запрос активных подписок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMySubscriptions {}
/// Запрос цен закрытия торговой сессии по инструментам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClosePricesRequest {
    /// Массив по инструментам.
    #[prost(message, repeated, tag = "1")]
    pub instruments: ::prost::alloc::vec::Vec<InstrumentClosePriceRequest>,
}
/// Запрос цен закрытия торговой сессии по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentClosePriceRequest {
    /// Идентификатор инструмента, принимает значение figi или instrument_uid.
    #[prost(string, tag = "1")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Цены закрытия торговой сессии по инструментам.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetClosePricesResponse {
    /// Массив по инструментам.
    #[prost(message, repeated, tag = "1")]
    pub close_prices: ::prost::alloc::vec::Vec<InstrumentClosePriceResponse>,
}
/// Цена закрытия торговой сессии по инструменту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InstrumentClosePriceResponse {
    /// Figi инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Uid инструмента.
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Цена закрытия торговой сессии.
    #[prost(message, optional, tag = "11")]
    pub price: ::core::option::Option<Quotation>,
    /// Дата совершения торгов.
    #[prost(message, optional, tag = "21")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Тип операции со списком подписок.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionAction {
    /// Статус подписки не определён.
    Unspecified = 0,
    /// Подписаться.
    Subscribe = 1,
    /// Отписаться.
    Unsubscribe = 2,
}
impl SubscriptionAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionAction::Unspecified => "SUBSCRIPTION_ACTION_UNSPECIFIED",
            SubscriptionAction::Subscribe => "SUBSCRIPTION_ACTION_SUBSCRIBE",
            SubscriptionAction::Unsubscribe => "SUBSCRIPTION_ACTION_UNSUBSCRIBE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBSCRIPTION_ACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "SUBSCRIPTION_ACTION_SUBSCRIBE" => Some(Self::Subscribe),
            "SUBSCRIPTION_ACTION_UNSUBSCRIBE" => Some(Self::Unsubscribe),
            _ => None,
        }
    }
}
/// Интервал свечи.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionInterval {
    /// Интервал свечи не определён.
    Unspecified = 0,
    /// Минутные свечи.
    OneMinute = 1,
    /// Пятиминутные свечи.
    FiveMinutes = 2,
}
impl SubscriptionInterval {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionInterval::Unspecified => "SUBSCRIPTION_INTERVAL_UNSPECIFIED",
            SubscriptionInterval::OneMinute => "SUBSCRIPTION_INTERVAL_ONE_MINUTE",
            SubscriptionInterval::FiveMinutes => "SUBSCRIPTION_INTERVAL_FIVE_MINUTES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBSCRIPTION_INTERVAL_UNSPECIFIED" => Some(Self::Unspecified),
            "SUBSCRIPTION_INTERVAL_ONE_MINUTE" => Some(Self::OneMinute),
            "SUBSCRIPTION_INTERVAL_FIVE_MINUTES" => Some(Self::FiveMinutes),
            _ => None,
        }
    }
}
/// Результат подписки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubscriptionStatus {
    /// Статус подписки не определён.
    Unspecified = 0,
    /// Успешно.
    Success = 1,
    /// Инструмент не найден.
    InstrumentNotFound = 2,
    /// Некорректный статус подписки, список возможных значений: \[SubscriptionAction\](<https://tinkoff.github.io/investAPI/marketdata#subscriptionaction>).
    SubscriptionActionIsInvalid = 3,
    /// Некорректная глубина стакана, доступные значения: 1, 10, 20, 30, 40, 50.
    DepthIsInvalid = 4,
    /// Некорректный интервал свечей, список возможных значений: \[SubscriptionInterval\](<https://tinkoff.github.io/investAPI/marketdata#subscriptioninterval>).
    IntervalIsInvalid = 5,
    /// Превышен лимит на общее количество подписок в рамках стрима, подробнее: [Лимитная политика](<https://tinkoff.github.io/investAPI/limits/>).
    LimitIsExceeded = 6,
    /// Внутренняя ошибка сервиса.
    InternalError = 7,
    /// Превышен лимит на количество запросов на подписки в течение установленного отрезка времени
    TooManyRequests = 8,
}
impl SubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SubscriptionStatus::Unspecified => "SUBSCRIPTION_STATUS_UNSPECIFIED",
            SubscriptionStatus::Success => "SUBSCRIPTION_STATUS_SUCCESS",
            SubscriptionStatus::InstrumentNotFound => {
                "SUBSCRIPTION_STATUS_INSTRUMENT_NOT_FOUND"
            }
            SubscriptionStatus::SubscriptionActionIsInvalid => {
                "SUBSCRIPTION_STATUS_SUBSCRIPTION_ACTION_IS_INVALID"
            }
            SubscriptionStatus::DepthIsInvalid => "SUBSCRIPTION_STATUS_DEPTH_IS_INVALID",
            SubscriptionStatus::IntervalIsInvalid => {
                "SUBSCRIPTION_STATUS_INTERVAL_IS_INVALID"
            }
            SubscriptionStatus::LimitIsExceeded => {
                "SUBSCRIPTION_STATUS_LIMIT_IS_EXCEEDED"
            }
            SubscriptionStatus::InternalError => "SUBSCRIPTION_STATUS_INTERNAL_ERROR",
            SubscriptionStatus::TooManyRequests => {
                "SUBSCRIPTION_STATUS_TOO_MANY_REQUESTS"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUBSCRIPTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SUBSCRIPTION_STATUS_SUCCESS" => Some(Self::Success),
            "SUBSCRIPTION_STATUS_INSTRUMENT_NOT_FOUND" => Some(Self::InstrumentNotFound),
            "SUBSCRIPTION_STATUS_SUBSCRIPTION_ACTION_IS_INVALID" => {
                Some(Self::SubscriptionActionIsInvalid)
            }
            "SUBSCRIPTION_STATUS_DEPTH_IS_INVALID" => Some(Self::DepthIsInvalid),
            "SUBSCRIPTION_STATUS_INTERVAL_IS_INVALID" => Some(Self::IntervalIsInvalid),
            "SUBSCRIPTION_STATUS_LIMIT_IS_EXCEEDED" => Some(Self::LimitIsExceeded),
            "SUBSCRIPTION_STATUS_INTERNAL_ERROR" => Some(Self::InternalError),
            "SUBSCRIPTION_STATUS_TOO_MANY_REQUESTS" => Some(Self::TooManyRequests),
            _ => None,
        }
    }
}
/// Направление сделки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TradeDirection {
    /// Направление сделки не определено.
    Unspecified = 0,
    /// Покупка.
    Buy = 1,
    /// Продажа.
    Sell = 2,
}
impl TradeDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TradeDirection::Unspecified => "TRADE_DIRECTION_UNSPECIFIED",
            TradeDirection::Buy => "TRADE_DIRECTION_BUY",
            TradeDirection::Sell => "TRADE_DIRECTION_SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TRADE_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "TRADE_DIRECTION_BUY" => Some(Self::Buy),
            "TRADE_DIRECTION_SELL" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// Интервал свечей.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CandleInterval {
    /// Интервал не определён.
    Unspecified = 0,
    /// от 1 минуты до 1 дня.
    CandleInterval1Min = 1,
    /// от 5 минуты до 1 дня.
    CandleInterval5Min = 2,
    /// от 15 минут до 1 дня.
    CandleInterval15Min = 3,
    /// от 1 часа до 1 недели.
    Hour = 4,
    /// от 1 дня до 1 года.
    Day = 5,
    /// от 2 минут до 1 дня.
    CandleInterval2Min = 6,
    /// от 3 минут до 1 дня.
    CandleInterval3Min = 7,
    /// от 10 минут до 1 дня.
    CandleInterval10Min = 8,
    /// от 30 минут до 2 дней.
    CandleInterval30Min = 9,
    /// от 2 часов до 1 месяца.
    CandleInterval2Hour = 10,
    /// от 4 часов до 1 месяца.
    CandleInterval4Hour = 11,
    /// от 1 недели до 2 лет.
    Week = 12,
    /// от 1 месяца до 10 лет.
    Month = 13,
}
impl CandleInterval {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CandleInterval::Unspecified => "CANDLE_INTERVAL_UNSPECIFIED",
            CandleInterval::CandleInterval1Min => "CANDLE_INTERVAL_1_MIN",
            CandleInterval::CandleInterval5Min => "CANDLE_INTERVAL_5_MIN",
            CandleInterval::CandleInterval15Min => "CANDLE_INTERVAL_15_MIN",
            CandleInterval::Hour => "CANDLE_INTERVAL_HOUR",
            CandleInterval::Day => "CANDLE_INTERVAL_DAY",
            CandleInterval::CandleInterval2Min => "CANDLE_INTERVAL_2_MIN",
            CandleInterval::CandleInterval3Min => "CANDLE_INTERVAL_3_MIN",
            CandleInterval::CandleInterval10Min => "CANDLE_INTERVAL_10_MIN",
            CandleInterval::CandleInterval30Min => "CANDLE_INTERVAL_30_MIN",
            CandleInterval::CandleInterval2Hour => "CANDLE_INTERVAL_2_HOUR",
            CandleInterval::CandleInterval4Hour => "CANDLE_INTERVAL_4_HOUR",
            CandleInterval::Week => "CANDLE_INTERVAL_WEEK",
            CandleInterval::Month => "CANDLE_INTERVAL_MONTH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "CANDLE_INTERVAL_UNSPECIFIED" => Some(Self::Unspecified),
            "CANDLE_INTERVAL_1_MIN" => Some(Self::CandleInterval1Min),
            "CANDLE_INTERVAL_5_MIN" => Some(Self::CandleInterval5Min),
            "CANDLE_INTERVAL_15_MIN" => Some(Self::CandleInterval15Min),
            "CANDLE_INTERVAL_HOUR" => Some(Self::Hour),
            "CANDLE_INTERVAL_DAY" => Some(Self::Day),
            "CANDLE_INTERVAL_2_MIN" => Some(Self::CandleInterval2Min),
            "CANDLE_INTERVAL_3_MIN" => Some(Self::CandleInterval3Min),
            "CANDLE_INTERVAL_10_MIN" => Some(Self::CandleInterval10Min),
            "CANDLE_INTERVAL_30_MIN" => Some(Self::CandleInterval30Min),
            "CANDLE_INTERVAL_2_HOUR" => Some(Self::CandleInterval2Hour),
            "CANDLE_INTERVAL_4_HOUR" => Some(Self::CandleInterval4Hour),
            "CANDLE_INTERVAL_WEEK" => Some(Self::Week),
            "CANDLE_INTERVAL_MONTH" => Some(Self::Month),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod market_data_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MarketDataServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketDataServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MarketDataServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MarketDataServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MarketDataServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Метод запроса исторических свечей по инструменту.
        pub async fn get_candles(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCandlesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetCandlesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetCandles",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.MarketDataService",
                        "GetCandles",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод запроса цен последних сделок по инструментам.
        pub async fn get_last_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastPricesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLastPricesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetLastPrices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.MarketDataService",
                        "GetLastPrices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения стакана по инструменту.
        pub async fn get_order_book(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderBookRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrderBookResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetOrderBook",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.MarketDataService",
                        "GetOrderBook",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод запроса статуса торгов по инструментам.
        pub async fn get_trading_status(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTradingStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTradingStatusResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTradingStatus",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.MarketDataService",
                        "GetTradingStatus",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод запроса статуса торгов по инструментам.
        pub async fn get_trading_statuses(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTradingStatusesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetTradingStatusesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetTradingStatuses",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.MarketDataService",
                        "GetTradingStatuses",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод запроса обезличенных сделок за последний час.
        pub async fn get_last_trades(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLastTradesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetLastTradesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetLastTrades",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.MarketDataService",
                        "GetLastTrades",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод запроса цен закрытия торговой сессии по инструментам.
        pub async fn get_close_prices(
            &mut self,
            request: impl tonic::IntoRequest<super::GetClosePricesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetClosePricesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataService/GetClosePrices",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.MarketDataService",
                        "GetClosePrices",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod market_data_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct MarketDataStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MarketDataStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> MarketDataStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> MarketDataStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            MarketDataStreamServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Bi-directional стрим предоставления биржевой информации.
        pub async fn market_data_stream(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::MarketDataRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MarketDataResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataStreamService/MarketDataStream",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.MarketDataStreamService",
                        "MarketDataStream",
                    ),
                );
            self.inner.streaming(req, path, codec).await
        }
        /// Server-side стрим предоставления биржевой информации.
        pub async fn market_data_server_side_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::MarketDataServerSideStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::MarketDataResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.MarketDataStreamService/MarketDataServerSideStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.MarketDataStreamService",
                        "MarketDataServerSideStream",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Запрос получения списка операций по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC).
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Статус запрашиваемых операций.
    #[prost(enumeration = "OperationState", tag = "4")]
    pub state: i32,
    /// Figi-идентификатор инструмента для фильтрации.
    #[prost(string, tag = "5")]
    pub figi: ::prost::alloc::string::String,
}
/// Список операций.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationsResponse {
    /// Массив операций.
    #[prost(message, repeated, tag = "1")]
    pub operations: ::prost::alloc::vec::Vec<Operation>,
}
/// Данные по операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Operation {
    /// Идентификатор операции.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Идентификатор родительской операции.
    #[prost(string, tag = "2")]
    pub parent_operation_id: ::prost::alloc::string::String,
    /// Валюта операции.
    #[prost(string, tag = "3")]
    pub currency: ::prost::alloc::string::String,
    /// Сумма операции.
    #[prost(message, optional, tag = "4")]
    pub payment: ::core::option::Option<MoneyValue>,
    /// Цена операции за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "5")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Статус операции.
    #[prost(enumeration = "OperationState", tag = "6")]
    pub state: i32,
    /// Количество единиц инструмента.
    #[prost(int64, tag = "7")]
    pub quantity: i64,
    /// Неисполненный остаток по сделке.
    #[prost(int64, tag = "8")]
    pub quantity_rest: i64,
    /// Figi-идентификатор инструмента, связанного с операцией.
    #[prost(string, tag = "9")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента. Возможные значения: </br>**bond** — облигация; </br>**share** — акция; </br>**currency** — валюта; </br>**etf** — фонд; </br>**futures** — фьючерс.
    #[prost(string, tag = "10")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Дата и время операции в формате часовом поясе UTC.
    #[prost(message, optional, tag = "11")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Текстовое описание типа операции.
    #[prost(string, tag = "12")]
    pub r#type: ::prost::alloc::string::String,
    /// Тип операции.
    #[prost(enumeration = "OperationType", tag = "13")]
    pub operation_type: i32,
    /// Массив сделок.
    #[prost(message, repeated, tag = "14")]
    pub trades: ::prost::alloc::vec::Vec<OperationTrade>,
    /// Идентификатор актива
    #[prost(string, tag = "16")]
    pub asset_uid: ::prost::alloc::string::String,
    /// position_uid-идентификатора инструмента.
    #[prost(string, tag = "17")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "18")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Сделка по операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationTrade {
    /// Идентификатор сделки.
    #[prost(string, tag = "1")]
    pub trade_id: ::prost::alloc::string::String,
    /// Дата и время сделки в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Количество инструментов.
    #[prost(int64, tag = "3")]
    pub quantity: i64,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "4")]
    pub price: ::core::option::Option<MoneyValue>,
}
/// Запрос получения текущего портфеля по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Валюта, в которой требуется рассчитать портфель
    #[prost(enumeration = "portfolio_request::CurrencyRequest", tag = "2")]
    pub currency: i32,
}
/// Nested message and enum types in `PortfolioRequest`.
pub mod portfolio_request {
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum CurrencyRequest {
        /// Рубли
        Rub = 0,
        /// Доллары
        Usd = 1,
        /// Евро
        Eur = 2,
    }
    impl CurrencyRequest {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                CurrencyRequest::Rub => "RUB",
                CurrencyRequest::Usd => "USD",
                CurrencyRequest::Eur => "EUR",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "RUB" => Some(Self::Rub),
                "USD" => Some(Self::Usd),
                "EUR" => Some(Self::Eur),
                _ => None,
            }
        }
    }
}
/// Текущий портфель по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioResponse {
    /// Общая стоимость акций в портфеле.
    #[prost(message, optional, tag = "1")]
    pub total_amount_shares: ::core::option::Option<MoneyValue>,
    /// Общая стоимость облигаций в портфеле.
    #[prost(message, optional, tag = "2")]
    pub total_amount_bonds: ::core::option::Option<MoneyValue>,
    /// Общая стоимость фондов в портфеле.
    #[prost(message, optional, tag = "3")]
    pub total_amount_etf: ::core::option::Option<MoneyValue>,
    /// Общая стоимость валют в портфеле.
    #[prost(message, optional, tag = "4")]
    pub total_amount_currencies: ::core::option::Option<MoneyValue>,
    /// Общая стоимость фьючерсов в портфеле.
    #[prost(message, optional, tag = "5")]
    pub total_amount_futures: ::core::option::Option<MoneyValue>,
    /// Текущая относительная доходность портфеля, в %.
    #[prost(message, optional, tag = "6")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Список позиций портфеля.
    #[prost(message, repeated, tag = "7")]
    pub positions: ::prost::alloc::vec::Vec<PortfolioPosition>,
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "8")]
    pub account_id: ::prost::alloc::string::String,
    /// Общая стоимость опционов в портфеле.
    #[prost(message, optional, tag = "9")]
    pub total_amount_options: ::core::option::Option<MoneyValue>,
    /// Общая стоимость структурных нот в портфеле.
    #[prost(message, optional, tag = "10")]
    pub total_amount_sp: ::core::option::Option<MoneyValue>,
    /// Общая стоимость портфеля.
    #[prost(message, optional, tag = "11")]
    pub total_amount_portfolio: ::core::option::Option<MoneyValue>,
    /// Массив виртуальных позиций портфеля.
    #[prost(message, repeated, tag = "12")]
    pub virtual_positions: ::prost::alloc::vec::Vec<VirtualPortfolioPosition>,
}
/// Запрос позиций портфеля по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Список позиций по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsResponse {
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag = "1")]
    pub money: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Массив заблокированных валютных позиций портфеля.
    #[prost(message, repeated, tag = "2")]
    pub blocked: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Список ценно-бумажных позиций портфеля.
    #[prost(message, repeated, tag = "3")]
    pub securities: ::prost::alloc::vec::Vec<PositionsSecurities>,
    /// Признак идущей в данный момент выгрузки лимитов.
    #[prost(bool, tag = "4")]
    pub limits_loading_in_progress: bool,
    /// Список фьючерсов портфеля.
    #[prost(message, repeated, tag = "5")]
    pub futures: ::prost::alloc::vec::Vec<PositionsFutures>,
    /// Список опционов портфеля.
    #[prost(message, repeated, tag = "6")]
    pub options: ::prost::alloc::vec::Vec<PositionsOptions>,
}
/// Запрос доступного для вывода остатка.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawLimitsRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Доступный для вывода остаток.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawLimitsResponse {
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag = "1")]
    pub money: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Массив заблокированных валютных позиций портфеля.
    #[prost(message, repeated, tag = "2")]
    pub blocked: ::prost::alloc::vec::Vec<MoneyValue>,
    /// Заблокировано под гарантийное обеспечение фьючерсов.
    #[prost(message, repeated, tag = "3")]
    pub blocked_guarantee: ::prost::alloc::vec::Vec<MoneyValue>,
}
/// Позиции портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioPosition {
    /// Figi-идентификатора инструмента.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "2")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Количество инструмента в портфеле в штуках.
    #[prost(message, optional, tag = "3")]
    pub quantity: ::core::option::Option<Quotation>,
    /// Средневзвешенная цена позиции. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "4")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "5")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Текущий НКД.
    #[prost(message, optional, tag = "6")]
    pub current_nkd: ::core::option::Option<MoneyValue>,
    /// Deprecated Средняя цена позиции в пунктах (для фьючерсов). **Возможна задержка до секунды для пересчёта**.
    #[deprecated]
    #[prost(message, optional, tag = "7")]
    pub average_position_price_pt: ::core::option::Option<Quotation>,
    /// Текущая цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "8")]
    pub current_price: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции по методу FIFO. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "9")]
    pub average_position_price_fifo: ::core::option::Option<MoneyValue>,
    /// Deprecated Количество лотов в портфеле.
    #[deprecated]
    #[prost(message, optional, tag = "10")]
    pub quantity_lots: ::core::option::Option<Quotation>,
    /// Заблокировано на бирже.
    #[prost(bool, tag = "21")]
    pub blocked: bool,
    /// Количество бумаг, заблокированных выставленными заявками.
    #[prost(message, optional, tag = "22")]
    pub blocked_lots: ::core::option::Option<Quotation>,
    /// position_uid-идентификатора инструмента
    #[prost(string, tag = "24")]
    pub position_uid: ::prost::alloc::string::String,
    /// instrument_uid-идентификатора инструмента
    #[prost(string, tag = "25")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Вариационная маржа
    #[prost(message, optional, tag = "26")]
    pub var_margin: ::core::option::Option<MoneyValue>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "27")]
    pub expected_yield_fifo: ::core::option::Option<Quotation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VirtualPortfolioPosition {
    /// position_uid-идентификатора инструмента
    #[prost(string, tag = "1")]
    pub position_uid: ::prost::alloc::string::String,
    /// instrument_uid-идентификатора инструмента
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Figi-идентификатора инструмента.
    #[prost(string, tag = "3")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "4")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Количество инструмента в портфеле в штуках.
    #[prost(message, optional, tag = "5")]
    pub quantity: ::core::option::Option<Quotation>,
    /// Средневзвешенная цена позиции. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "6")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "7")]
    pub expected_yield: ::core::option::Option<Quotation>,
    /// Текущая рассчитанная доходность позиции.
    #[prost(message, optional, tag = "8")]
    pub expected_yield_fifo: ::core::option::Option<Quotation>,
    /// Дата до которой нужно продать виртуальные бумаги, после этой даты виртуальная позиция "сгорит"
    #[prost(message, optional, tag = "9")]
    pub expire_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Текущая цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "10")]
    pub current_price: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции по методу FIFO. **Возможна задержка до секунды для пересчёта**.
    #[prost(message, optional, tag = "11")]
    pub average_position_price_fifo: ::core::option::Option<MoneyValue>,
}
/// Баланс позиции ценной бумаги.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSecurities {
    /// Figi-идентификатор бумаги.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Количество бумаг заблокированных выставленными заявками.
    #[prost(int64, tag = "2")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag = "3")]
    pub balance: i64,
    /// Уникальный идентификатор позиции.
    #[prost(string, tag = "4")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag = "5")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Заблокировано на бирже.
    #[prost(bool, tag = "11")]
    pub exchange_blocked: bool,
    /// Тип инструмента.
    #[prost(string, tag = "16")]
    pub instrument_type: ::prost::alloc::string::String,
}
/// Баланс фьючерса.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsFutures {
    /// Figi-идентификатор фьючерса.
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Количество бумаг заблокированных выставленными заявками.
    #[prost(int64, tag = "2")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag = "3")]
    pub balance: i64,
    /// Уникальный идентификатор позиции.
    #[prost(string, tag = "4")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag = "5")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Баланс опциона.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsOptions {
    /// Уникальный идентификатор позиции опциона.
    #[prost(string, tag = "1")]
    pub position_uid: ::prost::alloc::string::String,
    /// Уникальный идентификатор  инструмента.
    #[prost(string, tag = "2")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Количество бумаг заблокированных выставленными заявками.
    #[prost(int64, tag = "11")]
    pub blocked: i64,
    /// Текущий незаблокированный баланс.
    #[prost(int64, tag = "21")]
    pub balance: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReportRequest {
    #[prost(oneof = "broker_report_request::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<broker_report_request::Payload>,
}
/// Nested message and enum types in `BrokerReportRequest`.
pub mod broker_report_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        GenerateBrokerReportRequest(super::GenerateBrokerReportRequest),
        #[prost(message, tag = "2")]
        GetBrokerReportRequest(super::GetBrokerReportRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReportResponse {
    #[prost(oneof = "broker_report_response::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<broker_report_response::Payload>,
}
/// Nested message and enum types in `BrokerReportResponse`.
pub mod broker_report_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        #[prost(message, tag = "1")]
        GenerateBrokerReportResponse(super::GenerateBrokerReportResponse),
        #[prost(message, tag = "2")]
        GetBrokerReportResponse(super::GetBrokerReportResponse),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateBrokerReportRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода в часовом поясе UTC.
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateBrokerReportResponse {
    /// Идентификатор задачи формирования брокерского отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrokerReportRequest {
    /// Идентификатор задачи формирования брокерского отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Номер страницы отчета (начинается с 1), значение по умолчанию: 0.
    #[prost(int32, tag = "2")]
    pub page: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBrokerReportResponse {
    #[prost(message, repeated, tag = "1")]
    pub broker_report: ::prost::alloc::vec::Vec<BrokerReport>,
    /// Количество записей в отчете.
    #[prost(int32, tag = "2")]
    pub items_count: i32,
    /// Количество страниц с данными отчета (начинается с 0).
    #[prost(int32, tag = "3")]
    pub pages_count: i32,
    /// Текущая страница (начинается с 0).
    #[prost(int32, tag = "4")]
    pub page: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BrokerReport {
    /// Номер сделки.
    #[prost(string, tag = "1")]
    pub trade_id: ::prost::alloc::string::String,
    /// Номер поручения.
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "3")]
    pub figi: ::prost::alloc::string::String,
    /// Признак исполнения.
    #[prost(string, tag = "4")]
    pub execute_sign: ::prost::alloc::string::String,
    /// Дата и время заключения в часовом поясе UTC.
    #[prost(message, optional, tag = "5")]
    pub trade_datetime: ::core::option::Option<::prost_types::Timestamp>,
    /// Торговая площадка.
    #[prost(string, tag = "6")]
    pub exchange: ::prost::alloc::string::String,
    /// Режим торгов.
    #[prost(string, tag = "7")]
    pub class_code: ::prost::alloc::string::String,
    /// Вид сделки.
    #[prost(string, tag = "8")]
    pub direction: ::prost::alloc::string::String,
    /// Сокращённое наименование актива.
    #[prost(string, tag = "9")]
    pub name: ::prost::alloc::string::String,
    /// Код актива.
    #[prost(string, tag = "10")]
    pub ticker: ::prost::alloc::string::String,
    /// Цена за единицу.
    #[prost(message, optional, tag = "11")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Количество.
    #[prost(int64, tag = "12")]
    pub quantity: i64,
    /// Сумма (без НКД).
    #[prost(message, optional, tag = "13")]
    pub order_amount: ::core::option::Option<MoneyValue>,
    /// НКД.
    #[prost(message, optional, tag = "14")]
    pub aci_value: ::core::option::Option<Quotation>,
    /// Сумма сделки.
    #[prost(message, optional, tag = "15")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Комиссия брокера.
    #[prost(message, optional, tag = "16")]
    pub broker_commission: ::core::option::Option<MoneyValue>,
    /// Комиссия биржи.
    #[prost(message, optional, tag = "17")]
    pub exchange_commission: ::core::option::Option<MoneyValue>,
    /// Комиссия клир. центра.
    #[prost(message, optional, tag = "18")]
    pub exchange_clearing_commission: ::core::option::Option<MoneyValue>,
    /// Ставка РЕПО (%).
    #[prost(message, optional, tag = "19")]
    pub repo_rate: ::core::option::Option<Quotation>,
    /// Контрагент/Брокер.
    #[prost(string, tag = "20")]
    pub party: ::prost::alloc::string::String,
    /// Дата расчётов в часовом поясе UTC.
    #[prost(message, optional, tag = "21")]
    pub clear_value_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата поставки в часовом поясе UTC.
    #[prost(message, optional, tag = "22")]
    pub sec_value_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Статус брокера.
    #[prost(string, tag = "23")]
    pub broker_status: ::prost::alloc::string::String,
    /// Тип дог.
    #[prost(string, tag = "24")]
    pub separate_agreement_type: ::prost::alloc::string::String,
    /// Номер дог.
    #[prost(string, tag = "25")]
    pub separate_agreement_number: ::prost::alloc::string::String,
    /// Дата дог.
    #[prost(string, tag = "26")]
    pub separate_agreement_date: ::prost::alloc::string::String,
    /// Тип расчёта по сделке.
    #[prost(string, tag = "27")]
    pub delivery_type: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerRequest {
    #[prost(oneof = "get_dividends_foreign_issuer_request::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<get_dividends_foreign_issuer_request::Payload>,
}
/// Nested message and enum types in `GetDividendsForeignIssuerRequest`.
pub mod get_dividends_foreign_issuer_request {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект запроса формирования отчёта.
        #[prost(message, tag = "1")]
        GenerateDivForeignIssuerReport(
            super::GenerateDividendsForeignIssuerReportRequest,
        ),
        /// Объект запроса сформированного отчёта.
        #[prost(message, tag = "2")]
        GetDivForeignIssuerReport(super::GetDividendsForeignIssuerReportRequest),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerResponse {
    #[prost(oneof = "get_dividends_foreign_issuer_response::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<get_dividends_foreign_issuer_response::Payload>,
}
/// Nested message and enum types in `GetDividendsForeignIssuerResponse`.
pub mod get_dividends_foreign_issuer_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата задачи запуска формирования отчёта.
        #[prost(message, tag = "1")]
        GenerateDivForeignIssuerReportResponse(
            super::GenerateDividendsForeignIssuerReportResponse,
        ),
        /// Отчёт "Справка о доходах за пределами РФ".
        #[prost(message, tag = "2")]
        DivForeignIssuerReport(super::GetDividendsForeignIssuerReportResponse),
    }
}
/// Объект запроса формирования отчёта "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDividendsForeignIssuerReportRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag = "2")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC).
    #[prost(message, optional, tag = "3")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
}
/// Объект запроса сформированного отчёта "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerReportRequest {
    /// Идентификатор задачи формирования отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
    /// Номер страницы отчета (начинается с 0), значение по умолчанию: 0.
    #[prost(int32, tag = "2")]
    pub page: i32,
}
/// Объект результата задачи запуска формирования отчёта "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenerateDividendsForeignIssuerReportResponse {
    /// Идентификатор задачи формирования отчёта.
    #[prost(string, tag = "1")]
    pub task_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetDividendsForeignIssuerReportResponse {
    #[prost(message, repeated, tag = "1")]
    pub dividends_foreign_issuer_report: ::prost::alloc::vec::Vec<
        DividendsForeignIssuerReport,
    >,
    /// Количество записей в отчете.
    #[prost(int32, tag = "2")]
    pub items_count: i32,
    /// Количество страниц с данными отчета (начинается с 0).
    #[prost(int32, tag = "3")]
    pub pages_count: i32,
    /// Текущая страница (начинается с 0).
    #[prost(int32, tag = "4")]
    pub page: i32,
}
/// Отчёт "Справка о доходах за пределами РФ".
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DividendsForeignIssuerReport {
    /// Дата фиксации реестра.
    #[prost(message, optional, tag = "1")]
    pub record_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата выплаты.
    #[prost(message, optional, tag = "2")]
    pub payment_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Наименование ценной бумаги.
    #[prost(string, tag = "3")]
    pub security_name: ::prost::alloc::string::String,
    /// ISIN-идентификатор ценной бумаги.
    #[prost(string, tag = "4")]
    pub isin: ::prost::alloc::string::String,
    /// Страна эмитента. Для депозитарных расписок указывается страна эмитента базового актива.
    #[prost(string, tag = "5")]
    pub issuer_country: ::prost::alloc::string::String,
    /// Количество ценных бумаг.
    #[prost(int64, tag = "6")]
    pub quantity: i64,
    /// Выплаты на одну бумагу
    #[prost(message, optional, tag = "7")]
    pub dividend: ::core::option::Option<Quotation>,
    /// Комиссия внешних платёжных агентов.
    #[prost(message, optional, tag = "8")]
    pub external_commission: ::core::option::Option<Quotation>,
    /// Сумма до удержания налога.
    #[prost(message, optional, tag = "9")]
    pub dividend_gross: ::core::option::Option<Quotation>,
    /// Сумма налога, удержанного агентом.
    #[prost(message, optional, tag = "10")]
    pub tax: ::core::option::Option<Quotation>,
    /// Итоговая сумма выплаты.
    #[prost(message, optional, tag = "11")]
    pub dividend_amount: ::core::option::Option<Quotation>,
    /// Валюта.
    #[prost(string, tag = "12")]
    pub currency: ::prost::alloc::string::String,
}
/// Запрос установки stream-соединения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioStreamRequest {
    /// Массив идентификаторов счётов пользователя
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация по позициям и доходностям портфелей.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioStreamResponse {
    #[prost(oneof = "portfolio_stream_response::Payload", tags = "1, 2, 3")]
    pub payload: ::core::option::Option<portfolio_stream_response::Payload>,
}
/// Nested message and enum types in `PortfolioStreamResponse`.
pub mod portfolio_stream_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата подписки.
        #[prost(message, tag = "1")]
        Subscriptions(super::PortfolioSubscriptionResult),
        /// Объект стриминга портфеля.
        #[prost(message, tag = "2")]
        Portfolio(super::PortfolioResponse),
        /// Проверка активности стрима.
        #[prost(message, tag = "3")]
        Ping(super::Ping),
    }
}
/// Объект результата подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PortfolioSubscriptionResult {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<AccountSubscriptionStatus>,
}
/// Счет клиента.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountSubscriptionStatus {
    /// Идентификатор счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Результат подписки.
    #[prost(enumeration = "PortfolioSubscriptionStatus", tag = "6")]
    pub subscription_status: i32,
}
/// Запрос списка операций по счёту с пагинацией.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsByCursorRequest {
    /// Идентификатор счёта клиента. Обязательный параметр для данного метода, остальные параметры опциональны.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор инструмента (Figi инструмента или uid инструмента)
    #[prost(string, tag = "2")]
    pub instrument_id: ::prost::alloc::string::String,
    /// Начало периода (по UTC).
    #[prost(message, optional, tag = "6")]
    pub from: ::core::option::Option<::prost_types::Timestamp>,
    /// Окончание периода (по UTC).
    #[prost(message, optional, tag = "7")]
    pub to: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор элемента, с которого начать формировать ответ.
    #[prost(string, tag = "11")]
    pub cursor: ::prost::alloc::string::String,
    /// Лимит количества операций. По умолчанию устанавливается значение **100**, максимальное значение 1000.
    #[prost(int32, tag = "12")]
    pub limit: i32,
    /// Тип операции. Принимает значение из списка OperationType.
    #[prost(enumeration = "OperationType", repeated, tag = "13")]
    pub operation_types: ::prost::alloc::vec::Vec<i32>,
    /// Статус запрашиваемых операций, возможные значения указаны в OperationState.
    #[prost(enumeration = "OperationState", tag = "14")]
    pub state: i32,
    /// Флаг возвращать ли комиссии, по умолчанию false
    #[prost(bool, tag = "15")]
    pub without_commissions: bool,
    /// Флаг получения ответа без массива сделок.
    #[prost(bool, tag = "16")]
    pub without_trades: bool,
    /// Флаг не показывать overnight операций.
    #[prost(bool, tag = "17")]
    pub without_overnights: bool,
}
/// Список операций по счёту с пагинацией.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOperationsByCursorResponse {
    /// Признак, есть ли следующий элемент.
    #[prost(bool, tag = "1")]
    pub has_next: bool,
    /// Следующий курсор.
    #[prost(string, tag = "2")]
    pub next_cursor: ::prost::alloc::string::String,
    /// Список операций.
    #[prost(message, repeated, tag = "6")]
    pub items: ::prost::alloc::vec::Vec<OperationItem>,
}
/// Данные об операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItem {
    /// Курсор.
    #[prost(string, tag = "1")]
    pub cursor: ::prost::alloc::string::String,
    /// Номер счета клиента.
    #[prost(string, tag = "6")]
    pub broker_account_id: ::prost::alloc::string::String,
    /// Идентификатор операции, может меняться с течением времени.
    #[prost(string, tag = "16")]
    pub id: ::prost::alloc::string::String,
    /// Идентификатор родительской операции, может измениться, если изменился id родительской операции.
    #[prost(string, tag = "17")]
    pub parent_operation_id: ::prost::alloc::string::String,
    /// Название операции.
    #[prost(string, tag = "18")]
    pub name: ::prost::alloc::string::String,
    /// Дата поручения.
    #[prost(message, optional, tag = "21")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Тип операции.
    #[prost(enumeration = "OperationType", tag = "22")]
    pub r#type: i32,
    /// Описание операции.
    #[prost(string, tag = "23")]
    pub description: ::prost::alloc::string::String,
    /// Статус поручения.
    #[prost(enumeration = "OperationState", tag = "24")]
    pub state: i32,
    /// Уникальный идентификатор инструмента.
    #[prost(string, tag = "31")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Figi.
    #[prost(string, tag = "32")]
    pub figi: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(string, tag = "33")]
    pub instrument_type: ::prost::alloc::string::String,
    /// Тип инструмента.
    #[prost(enumeration = "InstrumentType", tag = "34")]
    pub instrument_kind: i32,
    /// position_uid-идентификатора инструмента.
    #[prost(string, tag = "35")]
    pub position_uid: ::prost::alloc::string::String,
    /// Сумма операции.
    #[prost(message, optional, tag = "41")]
    pub payment: ::core::option::Option<MoneyValue>,
    /// Цена операции за 1 инструмент.
    #[prost(message, optional, tag = "42")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Комиссия.
    #[prost(message, optional, tag = "43")]
    pub commission: ::core::option::Option<MoneyValue>,
    /// Доходность.
    #[prost(message, optional, tag = "44")]
    pub r#yield: ::core::option::Option<MoneyValue>,
    /// Относительная доходность.
    #[prost(message, optional, tag = "45")]
    pub yield_relative: ::core::option::Option<Quotation>,
    /// Накопленный купонный доход.
    #[prost(message, optional, tag = "46")]
    pub accrued_int: ::core::option::Option<MoneyValue>,
    /// Количество единиц инструмента.
    #[prost(int64, tag = "51")]
    pub quantity: i64,
    /// Неисполненный остаток по сделке.
    #[prost(int64, tag = "52")]
    pub quantity_rest: i64,
    /// Исполненный остаток.
    #[prost(int64, tag = "53")]
    pub quantity_done: i64,
    /// Дата и время снятия заявки.
    #[prost(message, optional, tag = "56")]
    pub cancel_date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Причина отмены операции.
    #[prost(string, tag = "57")]
    pub cancel_reason: ::prost::alloc::string::String,
    /// Массив сделок.
    #[prost(message, optional, tag = "61")]
    pub trades_info: ::core::option::Option<OperationItemTrades>,
    /// Идентификатор актива
    #[prost(string, tag = "64")]
    pub asset_uid: ::prost::alloc::string::String,
}
/// Массив с информацией о сделках.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItemTrades {
    #[prost(message, repeated, tag = "6")]
    pub trades: ::prost::alloc::vec::Vec<OperationItemTrade>,
}
/// Сделка по операции.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationItemTrade {
    /// Номер сделки
    #[prost(string, tag = "1")]
    pub num: ::prost::alloc::string::String,
    /// Дата сделки
    #[prost(message, optional, tag = "6")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
    /// Количество в единицах.
    #[prost(int64, tag = "11")]
    pub quantity: i64,
    /// Цена.
    #[prost(message, optional, tag = "16")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Доходность.
    #[prost(message, optional, tag = "21")]
    pub r#yield: ::core::option::Option<MoneyValue>,
    /// Относительная доходность.
    #[prost(message, optional, tag = "22")]
    pub yield_relative: ::core::option::Option<Quotation>,
}
/// Запрос установки stream-соединения позиций.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsStreamRequest {
    /// Массив идентификаторов счётов пользователя
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация по изменению позиций портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsStreamResponse {
    #[prost(oneof = "positions_stream_response::Payload", tags = "1, 2, 3")]
    pub payload: ::core::option::Option<positions_stream_response::Payload>,
}
/// Nested message and enum types in `PositionsStreamResponse`.
pub mod positions_stream_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Объект результата подписки.
        #[prost(message, tag = "1")]
        Subscriptions(super::PositionsSubscriptionResult),
        /// Объект стриминга позиций.
        #[prost(message, tag = "2")]
        Position(super::PositionData),
        /// Проверка активности стрима.
        #[prost(message, tag = "3")]
        Ping(super::Ping),
    }
}
/// Объект результата подписки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSubscriptionResult {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<PositionsSubscriptionStatus>,
}
/// Счет клиента.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsSubscriptionStatus {
    /// Идентификатор счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Результат подписки.
    #[prost(enumeration = "PositionsAccountSubscriptionStatus", tag = "6")]
    pub subscription_status: i32,
}
/// Данные о позиции портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionData {
    /// Идентификатор счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Массив валютных позиций портфеля.
    #[prost(message, repeated, tag = "2")]
    pub money: ::prost::alloc::vec::Vec<PositionsMoney>,
    /// Список ценно-бумажных позиций портфеля.
    #[prost(message, repeated, tag = "3")]
    pub securities: ::prost::alloc::vec::Vec<PositionsSecurities>,
    /// Список фьючерсов портфеля.
    #[prost(message, repeated, tag = "4")]
    pub futures: ::prost::alloc::vec::Vec<PositionsFutures>,
    /// Список опционов портфеля.
    #[prost(message, repeated, tag = "5")]
    pub options: ::prost::alloc::vec::Vec<PositionsOptions>,
    /// Дата и время операции в формате UTC.
    #[prost(message, optional, tag = "6")]
    pub date: ::core::option::Option<::prost_types::Timestamp>,
}
/// Валютная позиция портфеля.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PositionsMoney {
    /// Доступное количество валютный позиций.
    #[prost(message, optional, tag = "1")]
    pub available_value: ::core::option::Option<MoneyValue>,
    /// Заблокированное количество валютный позиций.
    #[prost(message, optional, tag = "2")]
    pub blocked_value: ::core::option::Option<MoneyValue>,
}
/// Статус запрашиваемых операций.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationState {
    /// Статус операции не определён
    Unspecified = 0,
    /// Исполнена.
    Executed = 1,
    /// Отменена.
    Canceled = 2,
    /// Исполняется.
    Progress = 3,
}
impl OperationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationState::Unspecified => "OPERATION_STATE_UNSPECIFIED",
            OperationState::Executed => "OPERATION_STATE_EXECUTED",
            OperationState::Canceled => "OPERATION_STATE_CANCELED",
            OperationState::Progress => "OPERATION_STATE_PROGRESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_STATE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_STATE_EXECUTED" => Some(Self::Executed),
            "OPERATION_STATE_CANCELED" => Some(Self::Canceled),
            "OPERATION_STATE_PROGRESS" => Some(Self::Progress),
            _ => None,
        }
    }
}
/// Тип операции.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OperationType {
    /// Тип операции не определён.
    Unspecified = 0,
    /// Пополнение брокерского счёта.
    Input = 1,
    /// Удержание НДФЛ по купонам.
    BondTax = 2,
    /// Вывод ЦБ.
    OutputSecurities = 3,
    /// Доход по сделке РЕПО овернайт.
    Overnight = 4,
    /// Удержание налога.
    Tax = 5,
    /// Полное погашение облигаций.
    BondRepaymentFull = 6,
    /// Продажа ЦБ с карты.
    SellCard = 7,
    /// Удержание налога по дивидендам.
    DividendTax = 8,
    /// Вывод денежных средств.
    Output = 9,
    /// Частичное погашение облигаций.
    BondRepayment = 10,
    /// Корректировка налога.
    TaxCorrection = 11,
    /// Удержание комиссии за обслуживание брокерского счёта.
    ServiceFee = 12,
    /// Удержание налога за материальную выгоду.
    BenefitTax = 13,
    /// Удержание комиссии за непокрытую позицию.
    MarginFee = 14,
    /// Покупка ЦБ.
    Buy = 15,
    /// Покупка ЦБ с карты.
    BuyCard = 16,
    /// Перевод ценных бумаг из другого депозитария.
    InputSecurities = 17,
    /// Продажа в результате Margin-call.
    SellMargin = 18,
    /// Удержание комиссии за операцию.
    BrokerFee = 19,
    /// Покупка в результате Margin-call.
    BuyMargin = 20,
    /// Выплата дивидендов.
    Dividend = 21,
    /// Продажа ЦБ.
    Sell = 22,
    /// Выплата купонов.
    Coupon = 23,
    /// Удержание комиссии SuccessFee.
    SuccessFee = 24,
    /// Передача дивидендного дохода.
    DividendTransfer = 25,
    /// Зачисление вариационной маржи.
    AccruingVarmargin = 26,
    /// Списание вариационной маржи.
    WritingOffVarmargin = 27,
    /// Покупка в рамках экспирации фьючерсного контракта.
    DeliveryBuy = 28,
    /// Продажа в рамках экспирации фьючерсного контракта.
    DeliverySell = 29,
    /// Комиссия за управление по счёту автоследования.
    TrackMfee = 30,
    /// Комиссия за результат по счёту автоследования.
    TrackPfee = 31,
    /// Удержание налога по ставке 15%.
    TaxProgressive = 32,
    /// Удержание налога по купонам по ставке 15%.
    BondTaxProgressive = 33,
    /// Удержание налога по дивидендам по ставке 15%.
    DividendTaxProgressive = 34,
    /// Удержание налога за материальную выгоду по ставке 15%.
    BenefitTaxProgressive = 35,
    /// Корректировка налога по ставке 15%.
    TaxCorrectionProgressive = 36,
    /// Удержание налога за возмещение по сделкам РЕПО по ставке 15%.
    TaxRepoProgressive = 37,
    /// Удержание налога за возмещение по сделкам РЕПО.
    TaxRepo = 38,
    /// Удержание налога по сделкам РЕПО.
    TaxRepoHold = 39,
    /// Возврат налога по сделкам РЕПО.
    TaxRepoRefund = 40,
    /// Удержание налога по сделкам РЕПО по ставке 15%.
    TaxRepoHoldProgressive = 41,
    /// Возврат налога по сделкам РЕПО по ставке 15%.
    TaxRepoRefundProgressive = 42,
    /// Выплата дивидендов на карту.
    DivExt = 43,
    /// Корректировка налога по купонам.
    TaxCorrectionCoupon = 44,
    /// Комиссия за валютный остаток.
    CashFee = 45,
    /// Комиссия за вывод валюты с брокерского счета.
    OutFee = 46,
    /// Гербовый сбор.
    OutStampDuty = 47,
    /// 	SWIFT-перевод
    OutputSwift = 50,
    /// 	SWIFT-перевод
    InputSwift = 51,
    ///   Перевод на карту
    OutputAcquiring = 53,
    /// 	Перевод с карты
    InputAcquiring = 54,
    /// 	Комиссия за вывод средств
    OutputPenalty = 55,
    /// 	Списание оплаты за сервис Советов
    AdviceFee = 56,
    ///   Перевод ценных бумаг с ИИС на Брокерский счет
    TransIisBs = 57,
    ///   Перевод ценных бумаг с одного брокерского счета на другой
    TransBsBs = 58,
    ///   Вывод денежных средств со счета
    OutMulti = 59,
    ///   Пополнение денежных средств со счета
    InpMulti = 60,
    ///   Размещение биржевого овернайта
    OverPlacement = 61,
    ///   Списание комиссии
    OverCom = 62,
    ///   Доход от оверанайта
    OverIncome = 63,
    /// Экспирация
    OptionExpiration = 64,
}
impl OperationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OperationType::Unspecified => "OPERATION_TYPE_UNSPECIFIED",
            OperationType::Input => "OPERATION_TYPE_INPUT",
            OperationType::BondTax => "OPERATION_TYPE_BOND_TAX",
            OperationType::OutputSecurities => "OPERATION_TYPE_OUTPUT_SECURITIES",
            OperationType::Overnight => "OPERATION_TYPE_OVERNIGHT",
            OperationType::Tax => "OPERATION_TYPE_TAX",
            OperationType::BondRepaymentFull => "OPERATION_TYPE_BOND_REPAYMENT_FULL",
            OperationType::SellCard => "OPERATION_TYPE_SELL_CARD",
            OperationType::DividendTax => "OPERATION_TYPE_DIVIDEND_TAX",
            OperationType::Output => "OPERATION_TYPE_OUTPUT",
            OperationType::BondRepayment => "OPERATION_TYPE_BOND_REPAYMENT",
            OperationType::TaxCorrection => "OPERATION_TYPE_TAX_CORRECTION",
            OperationType::ServiceFee => "OPERATION_TYPE_SERVICE_FEE",
            OperationType::BenefitTax => "OPERATION_TYPE_BENEFIT_TAX",
            OperationType::MarginFee => "OPERATION_TYPE_MARGIN_FEE",
            OperationType::Buy => "OPERATION_TYPE_BUY",
            OperationType::BuyCard => "OPERATION_TYPE_BUY_CARD",
            OperationType::InputSecurities => "OPERATION_TYPE_INPUT_SECURITIES",
            OperationType::SellMargin => "OPERATION_TYPE_SELL_MARGIN",
            OperationType::BrokerFee => "OPERATION_TYPE_BROKER_FEE",
            OperationType::BuyMargin => "OPERATION_TYPE_BUY_MARGIN",
            OperationType::Dividend => "OPERATION_TYPE_DIVIDEND",
            OperationType::Sell => "OPERATION_TYPE_SELL",
            OperationType::Coupon => "OPERATION_TYPE_COUPON",
            OperationType::SuccessFee => "OPERATION_TYPE_SUCCESS_FEE",
            OperationType::DividendTransfer => "OPERATION_TYPE_DIVIDEND_TRANSFER",
            OperationType::AccruingVarmargin => "OPERATION_TYPE_ACCRUING_VARMARGIN",
            OperationType::WritingOffVarmargin => "OPERATION_TYPE_WRITING_OFF_VARMARGIN",
            OperationType::DeliveryBuy => "OPERATION_TYPE_DELIVERY_BUY",
            OperationType::DeliverySell => "OPERATION_TYPE_DELIVERY_SELL",
            OperationType::TrackMfee => "OPERATION_TYPE_TRACK_MFEE",
            OperationType::TrackPfee => "OPERATION_TYPE_TRACK_PFEE",
            OperationType::TaxProgressive => "OPERATION_TYPE_TAX_PROGRESSIVE",
            OperationType::BondTaxProgressive => "OPERATION_TYPE_BOND_TAX_PROGRESSIVE",
            OperationType::DividendTaxProgressive => {
                "OPERATION_TYPE_DIVIDEND_TAX_PROGRESSIVE"
            }
            OperationType::BenefitTaxProgressive => {
                "OPERATION_TYPE_BENEFIT_TAX_PROGRESSIVE"
            }
            OperationType::TaxCorrectionProgressive => {
                "OPERATION_TYPE_TAX_CORRECTION_PROGRESSIVE"
            }
            OperationType::TaxRepoProgressive => "OPERATION_TYPE_TAX_REPO_PROGRESSIVE",
            OperationType::TaxRepo => "OPERATION_TYPE_TAX_REPO",
            OperationType::TaxRepoHold => "OPERATION_TYPE_TAX_REPO_HOLD",
            OperationType::TaxRepoRefund => "OPERATION_TYPE_TAX_REPO_REFUND",
            OperationType::TaxRepoHoldProgressive => {
                "OPERATION_TYPE_TAX_REPO_HOLD_PROGRESSIVE"
            }
            OperationType::TaxRepoRefundProgressive => {
                "OPERATION_TYPE_TAX_REPO_REFUND_PROGRESSIVE"
            }
            OperationType::DivExt => "OPERATION_TYPE_DIV_EXT",
            OperationType::TaxCorrectionCoupon => "OPERATION_TYPE_TAX_CORRECTION_COUPON",
            OperationType::CashFee => "OPERATION_TYPE_CASH_FEE",
            OperationType::OutFee => "OPERATION_TYPE_OUT_FEE",
            OperationType::OutStampDuty => "OPERATION_TYPE_OUT_STAMP_DUTY",
            OperationType::OutputSwift => "OPERATION_TYPE_OUTPUT_SWIFT",
            OperationType::InputSwift => "OPERATION_TYPE_INPUT_SWIFT",
            OperationType::OutputAcquiring => "OPERATION_TYPE_OUTPUT_ACQUIRING",
            OperationType::InputAcquiring => "OPERATION_TYPE_INPUT_ACQUIRING",
            OperationType::OutputPenalty => "OPERATION_TYPE_OUTPUT_PENALTY",
            OperationType::AdviceFee => "OPERATION_TYPE_ADVICE_FEE",
            OperationType::TransIisBs => "OPERATION_TYPE_TRANS_IIS_BS",
            OperationType::TransBsBs => "OPERATION_TYPE_TRANS_BS_BS",
            OperationType::OutMulti => "OPERATION_TYPE_OUT_MULTI",
            OperationType::InpMulti => "OPERATION_TYPE_INP_MULTI",
            OperationType::OverPlacement => "OPERATION_TYPE_OVER_PLACEMENT",
            OperationType::OverCom => "OPERATION_TYPE_OVER_COM",
            OperationType::OverIncome => "OPERATION_TYPE_OVER_INCOME",
            OperationType::OptionExpiration => "OPERATION_TYPE_OPTION_EXPIRATION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OPERATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "OPERATION_TYPE_INPUT" => Some(Self::Input),
            "OPERATION_TYPE_BOND_TAX" => Some(Self::BondTax),
            "OPERATION_TYPE_OUTPUT_SECURITIES" => Some(Self::OutputSecurities),
            "OPERATION_TYPE_OVERNIGHT" => Some(Self::Overnight),
            "OPERATION_TYPE_TAX" => Some(Self::Tax),
            "OPERATION_TYPE_BOND_REPAYMENT_FULL" => Some(Self::BondRepaymentFull),
            "OPERATION_TYPE_SELL_CARD" => Some(Self::SellCard),
            "OPERATION_TYPE_DIVIDEND_TAX" => Some(Self::DividendTax),
            "OPERATION_TYPE_OUTPUT" => Some(Self::Output),
            "OPERATION_TYPE_BOND_REPAYMENT" => Some(Self::BondRepayment),
            "OPERATION_TYPE_TAX_CORRECTION" => Some(Self::TaxCorrection),
            "OPERATION_TYPE_SERVICE_FEE" => Some(Self::ServiceFee),
            "OPERATION_TYPE_BENEFIT_TAX" => Some(Self::BenefitTax),
            "OPERATION_TYPE_MARGIN_FEE" => Some(Self::MarginFee),
            "OPERATION_TYPE_BUY" => Some(Self::Buy),
            "OPERATION_TYPE_BUY_CARD" => Some(Self::BuyCard),
            "OPERATION_TYPE_INPUT_SECURITIES" => Some(Self::InputSecurities),
            "OPERATION_TYPE_SELL_MARGIN" => Some(Self::SellMargin),
            "OPERATION_TYPE_BROKER_FEE" => Some(Self::BrokerFee),
            "OPERATION_TYPE_BUY_MARGIN" => Some(Self::BuyMargin),
            "OPERATION_TYPE_DIVIDEND" => Some(Self::Dividend),
            "OPERATION_TYPE_SELL" => Some(Self::Sell),
            "OPERATION_TYPE_COUPON" => Some(Self::Coupon),
            "OPERATION_TYPE_SUCCESS_FEE" => Some(Self::SuccessFee),
            "OPERATION_TYPE_DIVIDEND_TRANSFER" => Some(Self::DividendTransfer),
            "OPERATION_TYPE_ACCRUING_VARMARGIN" => Some(Self::AccruingVarmargin),
            "OPERATION_TYPE_WRITING_OFF_VARMARGIN" => Some(Self::WritingOffVarmargin),
            "OPERATION_TYPE_DELIVERY_BUY" => Some(Self::DeliveryBuy),
            "OPERATION_TYPE_DELIVERY_SELL" => Some(Self::DeliverySell),
            "OPERATION_TYPE_TRACK_MFEE" => Some(Self::TrackMfee),
            "OPERATION_TYPE_TRACK_PFEE" => Some(Self::TrackPfee),
            "OPERATION_TYPE_TAX_PROGRESSIVE" => Some(Self::TaxProgressive),
            "OPERATION_TYPE_BOND_TAX_PROGRESSIVE" => Some(Self::BondTaxProgressive),
            "OPERATION_TYPE_DIVIDEND_TAX_PROGRESSIVE" => {
                Some(Self::DividendTaxProgressive)
            }
            "OPERATION_TYPE_BENEFIT_TAX_PROGRESSIVE" => Some(Self::BenefitTaxProgressive),
            "OPERATION_TYPE_TAX_CORRECTION_PROGRESSIVE" => {
                Some(Self::TaxCorrectionProgressive)
            }
            "OPERATION_TYPE_TAX_REPO_PROGRESSIVE" => Some(Self::TaxRepoProgressive),
            "OPERATION_TYPE_TAX_REPO" => Some(Self::TaxRepo),
            "OPERATION_TYPE_TAX_REPO_HOLD" => Some(Self::TaxRepoHold),
            "OPERATION_TYPE_TAX_REPO_REFUND" => Some(Self::TaxRepoRefund),
            "OPERATION_TYPE_TAX_REPO_HOLD_PROGRESSIVE" => {
                Some(Self::TaxRepoHoldProgressive)
            }
            "OPERATION_TYPE_TAX_REPO_REFUND_PROGRESSIVE" => {
                Some(Self::TaxRepoRefundProgressive)
            }
            "OPERATION_TYPE_DIV_EXT" => Some(Self::DivExt),
            "OPERATION_TYPE_TAX_CORRECTION_COUPON" => Some(Self::TaxCorrectionCoupon),
            "OPERATION_TYPE_CASH_FEE" => Some(Self::CashFee),
            "OPERATION_TYPE_OUT_FEE" => Some(Self::OutFee),
            "OPERATION_TYPE_OUT_STAMP_DUTY" => Some(Self::OutStampDuty),
            "OPERATION_TYPE_OUTPUT_SWIFT" => Some(Self::OutputSwift),
            "OPERATION_TYPE_INPUT_SWIFT" => Some(Self::InputSwift),
            "OPERATION_TYPE_OUTPUT_ACQUIRING" => Some(Self::OutputAcquiring),
            "OPERATION_TYPE_INPUT_ACQUIRING" => Some(Self::InputAcquiring),
            "OPERATION_TYPE_OUTPUT_PENALTY" => Some(Self::OutputPenalty),
            "OPERATION_TYPE_ADVICE_FEE" => Some(Self::AdviceFee),
            "OPERATION_TYPE_TRANS_IIS_BS" => Some(Self::TransIisBs),
            "OPERATION_TYPE_TRANS_BS_BS" => Some(Self::TransBsBs),
            "OPERATION_TYPE_OUT_MULTI" => Some(Self::OutMulti),
            "OPERATION_TYPE_INP_MULTI" => Some(Self::InpMulti),
            "OPERATION_TYPE_OVER_PLACEMENT" => Some(Self::OverPlacement),
            "OPERATION_TYPE_OVER_COM" => Some(Self::OverCom),
            "OPERATION_TYPE_OVER_INCOME" => Some(Self::OverIncome),
            "OPERATION_TYPE_OPTION_EXPIRATION" => Some(Self::OptionExpiration),
            _ => None,
        }
    }
}
/// Результат подписки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PortfolioSubscriptionStatus {
    /// Тип не определён.
    Unspecified = 0,
    /// Успешно.
    Success = 1,
    /// Счёт не найден или недостаточно прав.
    AccountNotFound = 2,
    /// Произошла ошибка.
    InternalError = 3,
}
impl PortfolioSubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PortfolioSubscriptionStatus::Unspecified => {
                "PORTFOLIO_SUBSCRIPTION_STATUS_UNSPECIFIED"
            }
            PortfolioSubscriptionStatus::Success => {
                "PORTFOLIO_SUBSCRIPTION_STATUS_SUCCESS"
            }
            PortfolioSubscriptionStatus::AccountNotFound => {
                "PORTFOLIO_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND"
            }
            PortfolioSubscriptionStatus::InternalError => {
                "PORTFOLIO_SUBSCRIPTION_STATUS_INTERNAL_ERROR"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PORTFOLIO_SUBSCRIPTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "PORTFOLIO_SUBSCRIPTION_STATUS_SUCCESS" => Some(Self::Success),
            "PORTFOLIO_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND" => {
                Some(Self::AccountNotFound)
            }
            "PORTFOLIO_SUBSCRIPTION_STATUS_INTERNAL_ERROR" => Some(Self::InternalError),
            _ => None,
        }
    }
}
/// Результат подписки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionsAccountSubscriptionStatus {
    /// Тип не определён.
    PositionsSubscriptionStatusUnspecified = 0,
    /// Успешно.
    PositionsSubscriptionStatusSuccess = 1,
    /// Счёт не найден или недостаточно прав.
    PositionsSubscriptionStatusAccountNotFound = 2,
    /// Произошла ошибка.
    PositionsSubscriptionStatusInternalError = 3,
}
impl PositionsAccountSubscriptionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusUnspecified => {
                "POSITIONS_SUBSCRIPTION_STATUS_UNSPECIFIED"
            }
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusSuccess => {
                "POSITIONS_SUBSCRIPTION_STATUS_SUCCESS"
            }
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusAccountNotFound => {
                "POSITIONS_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND"
            }
            PositionsAccountSubscriptionStatus::PositionsSubscriptionStatusInternalError => {
                "POSITIONS_SUBSCRIPTION_STATUS_INTERNAL_ERROR"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POSITIONS_SUBSCRIPTION_STATUS_UNSPECIFIED" => {
                Some(Self::PositionsSubscriptionStatusUnspecified)
            }
            "POSITIONS_SUBSCRIPTION_STATUS_SUCCESS" => {
                Some(Self::PositionsSubscriptionStatusSuccess)
            }
            "POSITIONS_SUBSCRIPTION_STATUS_ACCOUNT_NOT_FOUND" => {
                Some(Self::PositionsSubscriptionStatusAccountNotFound)
            }
            "POSITIONS_SUBSCRIPTION_STATUS_INTERNAL_ERROR" => {
                Some(Self::PositionsSubscriptionStatusInternalError)
            }
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod operations_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OperationsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OperationsServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OperationsServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OperationsServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OperationsServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Метод получения списка операций по счёту.При работе с данным методом необходимо учитывать
        /// [особенности взаимодействия](/investAPI/operations_problems) с данным методом.
        pub async fn get_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::OperationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OperationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetOperations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OperationsService",
                        "GetOperations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения портфеля по счёту.
        pub async fn get_portfolio(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PortfolioResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetPortfolio",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OperationsService",
                        "GetPortfolio",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка позиций по счёту.
        pub async fn get_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PositionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetPositions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OperationsService",
                        "GetPositions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения доступного остатка для вывода средств.
        pub async fn get_withdraw_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawLimitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WithdrawLimitsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetWithdrawLimits",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OperationsService",
                        "GetWithdrawLimits",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения брокерского отчёта.
        pub async fn get_broker_report(
            &mut self,
            request: impl tonic::IntoRequest<super::BrokerReportRequest>,
        ) -> std::result::Result<
            tonic::Response<super::BrokerReportResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetBrokerReport",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OperationsService",
                        "GetBrokerReport",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения отчёта "Справка о доходах за пределами РФ".
        pub async fn get_dividends_foreign_issuer(
            &mut self,
            request: impl tonic::IntoRequest<super::GetDividendsForeignIssuerRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetDividendsForeignIssuerResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetDividendsForeignIssuer",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OperationsService",
                        "GetDividendsForeignIssuer",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка операций по счёту с пагинацией. При работе с данным методом необходимо учитывать
        /// [особенности взаимодействия](/investAPI/operations_problems) с данным методом.
        pub async fn get_operations_by_cursor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationsByCursorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOperationsByCursorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsService/GetOperationsByCursor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OperationsService",
                        "GetOperationsByCursor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod operations_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OperationsStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OperationsStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OperationsStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OperationsStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OperationsStreamServiceClient::new(
                InterceptedService::new(inner, interceptor),
            )
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Server-side stream обновлений портфеля
        pub async fn portfolio_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PortfolioStreamResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsStreamService/PortfolioStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OperationsStreamService",
                        "PortfolioStream",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
        /// Server-side stream обновлений информации по изменению позиций портфеля
        pub async fn positions_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::PositionsStreamResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OperationsStreamService/PositionsStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OperationsStreamService",
                        "PositionsStream",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Запрос установки соединения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesStreamRequest {
    /// Идентификаторы счетов.
    #[prost(string, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Информация о торговых поручениях.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradesStreamResponse {
    #[prost(oneof = "trades_stream_response::Payload", tags = "1, 2")]
    pub payload: ::core::option::Option<trades_stream_response::Payload>,
}
/// Nested message and enum types in `TradesStreamResponse`.
pub mod trades_stream_response {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Payload {
        /// Информация об исполнении торгового поручения.
        #[prost(message, tag = "1")]
        OrderTrades(super::OrderTrades),
        /// Проверка активности стрима.
        #[prost(message, tag = "2")]
        Ping(super::Ping),
    }
}
/// Информация об исполнении торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderTrades {
    /// Идентификатор торгового поручения.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    /// Дата и время создания сообщения в часовом поясе UTC.
    #[prost(message, optional, tag = "2")]
    pub created_at: ::core::option::Option<::prost_types::Timestamp>,
    /// Направление сделки.
    #[prost(enumeration = "OrderDirection", tag = "3")]
    pub direction: i32,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "4")]
    pub figi: ::prost::alloc::string::String,
    /// Массив сделок.
    #[prost(message, repeated, tag = "5")]
    pub trades: ::prost::alloc::vec::Vec<OrderTrade>,
    /// Идентификатор счёта.
    #[prost(string, tag = "6")]
    pub account_id: ::prost::alloc::string::String,
    /// UID идентификатор инструмента.
    #[prost(string, tag = "7")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Информация о сделке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderTrade {
    /// Дата и время совершения сделки в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена за 1 инструмент, по которой совершена сделка.
    #[prost(message, optional, tag = "2")]
    pub price: ::core::option::Option<Quotation>,
    /// Количество штук в сделке.
    #[prost(int64, tag = "3")]
    pub quantity: i64,
    /// Идентификатор сделки.
    #[prost(string, tag = "4")]
    pub trade_id: ::prost::alloc::string::String,
}
/// Запрос выставления торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostOrderRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Количество лотов.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента. Игнорируется для рыночных поручений.
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    /// Направление операции.
    #[prost(enumeration = "OrderDirection", tag = "4")]
    pub direction: i32,
    /// Номер счёта.
    #[prost(string, tag = "5")]
    pub account_id: ::prost::alloc::string::String,
    /// Тип заявки.
    #[prost(enumeration = "OrderType", tag = "6")]
    pub order_type: i32,
    /// Идентификатор запроса выставления поручения для целей идемпотентности в формате UID. Максимальная длина 36 символов.
    #[prost(string, tag = "7")]
    pub order_id: ::prost::alloc::string::String,
    /// Идентификатор инструмента, принимает значения Figi или Instrument_uid.
    #[prost(string, tag = "8")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Информация о выставлении поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostOrderResponse {
    /// Биржевой идентификатор заявки.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    /// Текущий статус заявки.
    #[prost(enumeration = "OrderExecutionReportStatus", tag = "2")]
    pub execution_report_status: i32,
    /// Запрошено лотов.
    #[prost(int64, tag = "3")]
    pub lots_requested: i64,
    /// Исполнено лотов.
    #[prost(int64, tag = "4")]
    pub lots_executed: i64,
    /// Начальная цена заявки. Произведение количества запрошенных лотов на цену.
    #[prost(message, optional, tag = "5")]
    pub initial_order_price: ::core::option::Option<MoneyValue>,
    /// Исполненная средняя цена 1 одного инструмента в заявки.
    #[prost(message, optional, tag = "6")]
    pub executed_order_price: ::core::option::Option<MoneyValue>,
    /// Итоговая стоимость заявки, включающая все комиссии.
    #[prost(message, optional, tag = "7")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Начальная комиссия. Комиссия рассчитанная при выставлении заявки.
    #[prost(message, optional, tag = "8")]
    pub initial_commission: ::core::option::Option<MoneyValue>,
    /// Фактическая комиссия по итогам исполнения заявки.
    #[prost(message, optional, tag = "9")]
    pub executed_commission: ::core::option::Option<MoneyValue>,
    /// Значение НКД (накопленного купонного дохода) на дату. Подробнее: [НКД при выставлении торговых поручений](<https://tinkoff.github.io/investAPI/head-orders#coupon>)
    #[prost(message, optional, tag = "10")]
    pub aci_value: ::core::option::Option<MoneyValue>,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "11")]
    pub figi: ::prost::alloc::string::String,
    /// Направление сделки.
    #[prost(enumeration = "OrderDirection", tag = "12")]
    pub direction: i32,
    /// Начальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "13")]
    pub initial_security_price: ::core::option::Option<MoneyValue>,
    /// Тип заявки.
    #[prost(enumeration = "OrderType", tag = "14")]
    pub order_type: i32,
    /// Дополнительные данные об исполнении заявки.
    #[prost(string, tag = "15")]
    pub message: ::prost::alloc::string::String,
    /// Начальная цена заявки в пунктах (для фьючерсов).
    #[prost(message, optional, tag = "16")]
    pub initial_order_price_pt: ::core::option::Option<Quotation>,
    /// UID идентификатор инструмента.
    #[prost(string, tag = "17")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Запрос отмены торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderRequest {
    /// Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки.
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
}
/// Результат отмены торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelOrderResponse {
    /// Дата и время отмены заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Запрос получения статуса торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrderStateRequest {
    /// Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки.
    #[prost(string, tag = "2")]
    pub order_id: ::prost::alloc::string::String,
}
/// Запрос получения списка активных торговых поручений.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrdersRequest {
    /// Номер счёта.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Список активных торговых поручений.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetOrdersResponse {
    /// Массив активных заявок.
    #[prost(message, repeated, tag = "1")]
    pub orders: ::prost::alloc::vec::Vec<OrderState>,
}
/// Информация о торговом поручении.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderState {
    /// Биржевой идентификатор заявки.
    #[prost(string, tag = "1")]
    pub order_id: ::prost::alloc::string::String,
    /// Текущий статус заявки.
    #[prost(enumeration = "OrderExecutionReportStatus", tag = "2")]
    pub execution_report_status: i32,
    /// Запрошено лотов.
    #[prost(int64, tag = "3")]
    pub lots_requested: i64,
    /// Исполнено лотов.
    #[prost(int64, tag = "4")]
    pub lots_executed: i64,
    /// Начальная цена заявки. Произведение количества запрошенных лотов на цену.
    #[prost(message, optional, tag = "5")]
    pub initial_order_price: ::core::option::Option<MoneyValue>,
    /// Исполненная цена заявки. Произведение средней цены покупки на количество лотов.
    #[prost(message, optional, tag = "6")]
    pub executed_order_price: ::core::option::Option<MoneyValue>,
    /// Итоговая стоимость заявки, включающая все комиссии.
    #[prost(message, optional, tag = "7")]
    pub total_order_amount: ::core::option::Option<MoneyValue>,
    /// Средняя цена позиции по сделке.
    #[prost(message, optional, tag = "8")]
    pub average_position_price: ::core::option::Option<MoneyValue>,
    /// Начальная комиссия. Комиссия, рассчитанная на момент подачи заявки.
    #[prost(message, optional, tag = "9")]
    pub initial_commission: ::core::option::Option<MoneyValue>,
    /// Фактическая комиссия по итогам исполнения заявки.
    #[prost(message, optional, tag = "10")]
    pub executed_commission: ::core::option::Option<MoneyValue>,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "11")]
    pub figi: ::prost::alloc::string::String,
    /// Направление заявки.
    #[prost(enumeration = "OrderDirection", tag = "12")]
    pub direction: i32,
    /// Начальная цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "13")]
    pub initial_security_price: ::core::option::Option<MoneyValue>,
    /// Стадии выполнения заявки.
    #[prost(message, repeated, tag = "14")]
    pub stages: ::prost::alloc::vec::Vec<OrderStage>,
    /// Сервисная комиссия.
    #[prost(message, optional, tag = "15")]
    pub service_commission: ::core::option::Option<MoneyValue>,
    /// Валюта заявки.
    #[prost(string, tag = "16")]
    pub currency: ::prost::alloc::string::String,
    /// Тип заявки.
    #[prost(enumeration = "OrderType", tag = "17")]
    pub order_type: i32,
    /// Дата и время выставления заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "18")]
    pub order_date: ::core::option::Option<::prost_types::Timestamp>,
    /// UID идентификатор инструмента.
    #[prost(string, tag = "19")]
    pub instrument_uid: ::prost::alloc::string::String,
    /// Идентификатор ключа идемпотентности, переданный клиентом, в формате UID. Максимальная длина 36 символов.
    #[prost(string, tag = "20")]
    pub order_request_id: ::prost::alloc::string::String,
}
/// Сделки в рамках торгового поручения.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderStage {
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "1")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Количество лотов.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    /// Идентификатор сделки.
    #[prost(string, tag = "3")]
    pub trade_id: ::prost::alloc::string::String,
}
/// Запрос изменения выставленной заявки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplaceOrderRequest {
    /// Номер счета.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Идентификатор заявки на бирже.
    #[prost(string, tag = "6")]
    pub order_id: ::prost::alloc::string::String,
    /// Новый идентификатор запроса выставления поручения для целей идемпотентности. Максимальная длина 36 символов. Перезатирает старый ключ.
    #[prost(string, tag = "7")]
    pub idempotency_key: ::prost::alloc::string::String,
    /// Количество лотов.
    #[prost(int64, tag = "11")]
    pub quantity: i64,
    /// Цена за 1 инструмент.
    #[prost(message, optional, tag = "12")]
    pub price: ::core::option::Option<Quotation>,
    /// Тип цены.
    #[prost(enumeration = "PriceType", tag = "13")]
    pub price_type: i32,
}
/// Направление операции.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderDirection {
    /// Значение не указано
    Unspecified = 0,
    /// Покупка
    Buy = 1,
    /// Продажа
    Sell = 2,
}
impl OrderDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderDirection::Unspecified => "ORDER_DIRECTION_UNSPECIFIED",
            OrderDirection::Buy => "ORDER_DIRECTION_BUY",
            OrderDirection::Sell => "ORDER_DIRECTION_SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "ORDER_DIRECTION_BUY" => Some(Self::Buy),
            "ORDER_DIRECTION_SELL" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// Тип заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    /// Значение не указано
    Unspecified = 0,
    /// Лимитная
    Limit = 1,
    /// Рыночная
    Market = 2,
    /// Лучшая цена
    Bestprice = 3,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Unspecified => "ORDER_TYPE_UNSPECIFIED",
            OrderType::Limit => "ORDER_TYPE_LIMIT",
            OrderType::Market => "ORDER_TYPE_MARKET",
            OrderType::Bestprice => "ORDER_TYPE_BESTPRICE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ORDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ORDER_TYPE_LIMIT" => Some(Self::Limit),
            "ORDER_TYPE_MARKET" => Some(Self::Market),
            "ORDER_TYPE_BESTPRICE" => Some(Self::Bestprice),
            _ => None,
        }
    }
}
/// Текущий статус заявки (поручения)
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderExecutionReportStatus {
    ExecutionReportStatusUnspecified = 0,
    /// Исполнена
    ExecutionReportStatusFill = 1,
    /// Отклонена
    ExecutionReportStatusRejected = 2,
    /// Отменена пользователем
    ExecutionReportStatusCancelled = 3,
    /// Новая
    ExecutionReportStatusNew = 4,
    /// Частично исполнена
    ExecutionReportStatusPartiallyfill = 5,
}
impl OrderExecutionReportStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderExecutionReportStatus::ExecutionReportStatusUnspecified => {
                "EXECUTION_REPORT_STATUS_UNSPECIFIED"
            }
            OrderExecutionReportStatus::ExecutionReportStatusFill => {
                "EXECUTION_REPORT_STATUS_FILL"
            }
            OrderExecutionReportStatus::ExecutionReportStatusRejected => {
                "EXECUTION_REPORT_STATUS_REJECTED"
            }
            OrderExecutionReportStatus::ExecutionReportStatusCancelled => {
                "EXECUTION_REPORT_STATUS_CANCELLED"
            }
            OrderExecutionReportStatus::ExecutionReportStatusNew => {
                "EXECUTION_REPORT_STATUS_NEW"
            }
            OrderExecutionReportStatus::ExecutionReportStatusPartiallyfill => {
                "EXECUTION_REPORT_STATUS_PARTIALLYFILL"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EXECUTION_REPORT_STATUS_UNSPECIFIED" => {
                Some(Self::ExecutionReportStatusUnspecified)
            }
            "EXECUTION_REPORT_STATUS_FILL" => Some(Self::ExecutionReportStatusFill),
            "EXECUTION_REPORT_STATUS_REJECTED" => {
                Some(Self::ExecutionReportStatusRejected)
            }
            "EXECUTION_REPORT_STATUS_CANCELLED" => {
                Some(Self::ExecutionReportStatusCancelled)
            }
            "EXECUTION_REPORT_STATUS_NEW" => Some(Self::ExecutionReportStatusNew),
            "EXECUTION_REPORT_STATUS_PARTIALLYFILL" => {
                Some(Self::ExecutionReportStatusPartiallyfill)
            }
            _ => None,
        }
    }
}
/// Тип цены.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PriceType {
    /// Значение не определено.
    Unspecified = 0,
    /// Цена в пунктах (только для фьючерсов и облигаций).
    Point = 1,
    /// Цена в валюте расчётов по инструменту.
    Currency = 2,
}
impl PriceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PriceType::Unspecified => "PRICE_TYPE_UNSPECIFIED",
            PriceType::Point => "PRICE_TYPE_POINT",
            PriceType::Currency => "PRICE_TYPE_CURRENCY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PRICE_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE_TYPE_POINT" => Some(Self::Point),
            "PRICE_TYPE_CURRENCY" => Some(Self::Currency),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod orders_stream_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OrdersStreamServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrdersStreamServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrdersStreamServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrdersStreamServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OrdersStreamServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Stream сделок пользователя
        pub async fn trades_stream(
            &mut self,
            request: impl tonic::IntoRequest<super::TradesStreamRequest>,
        ) -> std::result::Result<
            tonic::Response<tonic::codec::Streaming<super::TradesStreamResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersStreamService/TradesStream",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OrdersStreamService",
                        "TradesStream",
                    ),
                );
            self.inner.server_streaming(req, path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod orders_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct OrdersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl OrdersServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> OrdersServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> OrdersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            OrdersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Метод выставления заявки.
        pub async fn post_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/PostOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OrdersService",
                        "PostOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод отмены биржевой заявки.
        pub async fn cancel_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/CancelOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OrdersService",
                        "CancelOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения статуса торгового поручения.
        pub async fn get_order_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderStateRequest>,
        ) -> std::result::Result<tonic::Response<super::OrderState>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/GetOrderState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OrdersService",
                        "GetOrderState",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка активных заявок по счёту.
        pub async fn get_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrdersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/GetOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OrdersService",
                        "GetOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод изменения выставленной заявки.
        pub async fn replace_order(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.OrdersService/ReplaceOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.OrdersService",
                        "ReplaceOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Запрос получения счетов пользователя.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountsRequest {}
/// Список счетов пользователя.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAccountsResponse {
    /// Массив счетов клиента.
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
}
/// Информация о счёте.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    /// Идентификатор счёта.
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// Тип счёта.
    #[prost(enumeration = "AccountType", tag = "2")]
    pub r#type: i32,
    /// Название счёта.
    #[prost(string, tag = "3")]
    pub name: ::prost::alloc::string::String,
    /// Статус счёта.
    #[prost(enumeration = "AccountStatus", tag = "4")]
    pub status: i32,
    /// Дата открытия счёта в часовом поясе UTC.
    #[prost(message, optional, tag = "5")]
    pub opened_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата закрытия счёта в часовом поясе UTC.
    #[prost(message, optional, tag = "6")]
    pub closed_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Уровень доступа к текущему счёту (определяется токеном).
    #[prost(enumeration = "AccessLevel", tag = "7")]
    pub access_level: i32,
}
/// Запрос маржинальных показателей по счёту
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarginAttributesRequest {
    /// Идентификатор счёта пользователя.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Маржинальные показатели по счёту.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetMarginAttributesResponse {
    /// Ликвидная стоимость портфеля. Подробнее: [что такое ликвидный портфель?](<https://help.tinkoff.ru/margin-trade/short/liquid-portfolio/>).
    #[prost(message, optional, tag = "1")]
    pub liquid_portfolio: ::core::option::Option<MoneyValue>,
    /// Начальная маржа — начальное обеспечение для совершения новой сделки. Подробнее: [начальная и минимальная маржа](<https://help.tinkoff.ru/margin-trade/short/initial-and-maintenance-margin/>).
    #[prost(message, optional, tag = "2")]
    pub starting_margin: ::core::option::Option<MoneyValue>,
    /// Минимальная маржа — это минимальное обеспечение для поддержания позиции, которую вы уже открыли. Подробнее: [начальная и минимальная маржа](<https://help.tinkoff.ru/margin-trade/short/initial-and-maintenance-margin/>).
    #[prost(message, optional, tag = "3")]
    pub minimal_margin: ::core::option::Option<MoneyValue>,
    /// Уровень достаточности средств. Соотношение стоимости ликвидного портфеля к начальной марже.
    #[prost(message, optional, tag = "4")]
    pub funds_sufficiency_level: ::core::option::Option<Quotation>,
    /// Объем недостающих средств. Разница между стартовой маржой и ликвидной стоимости портфеля.
    #[prost(message, optional, tag = "5")]
    pub amount_of_missing_funds: ::core::option::Option<MoneyValue>,
    /// Скорректированная маржа.Начальная маржа, в которой плановые позиции рассчитываются с учётом активных заявок на покупку позиций лонг или продажу позиций шорт.
    #[prost(message, optional, tag = "6")]
    pub corrected_margin: ::core::option::Option<MoneyValue>,
}
/// Запрос текущих лимитов пользователя.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserTariffRequest {}
/// Текущие лимиты пользователя.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetUserTariffResponse {
    /// Массив лимитов пользователя по unary-запросам.
    #[prost(message, repeated, tag = "1")]
    pub unary_limits: ::prost::alloc::vec::Vec<UnaryLimit>,
    /// Массив лимитов пользователей для stream-соединений.
    #[prost(message, repeated, tag = "2")]
    pub stream_limits: ::prost::alloc::vec::Vec<StreamLimit>,
}
/// Лимит unary-методов.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UnaryLimit {
    /// Количество unary-запросов в минуту.
    #[prost(int32, tag = "1")]
    pub limit_per_minute: i32,
    /// Названия методов.
    #[prost(string, repeated, tag = "2")]
    pub methods: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Лимит stream-соединений.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamLimit {
    /// Максимальное количество stream-соединений.
    #[prost(int32, tag = "1")]
    pub limit: i32,
    /// Названия stream-методов.
    #[prost(string, repeated, tag = "2")]
    pub streams: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Текущее количество открытых stream-соединений.
    #[prost(int32, tag = "3")]
    pub open: i32,
}
/// Запрос информации о пользователе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoRequest {}
/// Информация о пользователе.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInfoResponse {
    /// Признак премиум клиента.
    #[prost(bool, tag = "1")]
    pub prem_status: bool,
    /// Признак квалифицированного инвестора.
    #[prost(bool, tag = "2")]
    pub qual_status: bool,
    /// Набор требующих тестирования инструментов и возможностей, с которыми может работать пользователь. \[Подробнее\](<https://tinkoff.github.io/investAPI/faq_users/>).
    #[prost(string, repeated, tag = "3")]
    pub qualified_for_work_with: ::prost::alloc::vec::Vec<
        ::prost::alloc::string::String,
    >,
    /// Наименование тарифа пользователя.
    #[prost(string, tag = "4")]
    pub tariff: ::prost::alloc::string::String,
}
/// Тип счёта.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountType {
    /// Тип аккаунта не определён.
    Unspecified = 0,
    /// Брокерский счёт Тинькофф.
    Tinkoff = 1,
    /// ИИС счёт.
    TinkoffIis = 2,
    /// Инвесткопилка.
    InvestBox = 3,
}
impl AccountType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccountType::Unspecified => "ACCOUNT_TYPE_UNSPECIFIED",
            AccountType::Tinkoff => "ACCOUNT_TYPE_TINKOFF",
            AccountType::TinkoffIis => "ACCOUNT_TYPE_TINKOFF_IIS",
            AccountType::InvestBox => "ACCOUNT_TYPE_INVEST_BOX",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCOUNT_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCOUNT_TYPE_TINKOFF" => Some(Self::Tinkoff),
            "ACCOUNT_TYPE_TINKOFF_IIS" => Some(Self::TinkoffIis),
            "ACCOUNT_TYPE_INVEST_BOX" => Some(Self::InvestBox),
            _ => None,
        }
    }
}
/// Статус счёта.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountStatus {
    /// Статус счёта не определён.
    Unspecified = 0,
    /// Новый, в процессе открытия.
    New = 1,
    /// Открытый и активный счёт.
    Open = 2,
    /// Закрытый счёт.
    Closed = 3,
}
impl AccountStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccountStatus::Unspecified => "ACCOUNT_STATUS_UNSPECIFIED",
            AccountStatus::New => "ACCOUNT_STATUS_NEW",
            AccountStatus::Open => "ACCOUNT_STATUS_OPEN",
            AccountStatus::Closed => "ACCOUNT_STATUS_CLOSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCOUNT_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "ACCOUNT_STATUS_NEW" => Some(Self::New),
            "ACCOUNT_STATUS_OPEN" => Some(Self::Open),
            "ACCOUNT_STATUS_CLOSED" => Some(Self::Closed),
            _ => None,
        }
    }
}
/// Уровень доступа к счёту.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccessLevel {
    /// Уровень доступа не определён.
    AccountAccessLevelUnspecified = 0,
    /// Полный доступ к счёту.
    AccountAccessLevelFullAccess = 1,
    /// Доступ с уровнем прав "только чтение".
    AccountAccessLevelReadOnly = 2,
    /// Доступ отсутствует.
    AccountAccessLevelNoAccess = 3,
}
impl AccessLevel {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccessLevel::AccountAccessLevelUnspecified => {
                "ACCOUNT_ACCESS_LEVEL_UNSPECIFIED"
            }
            AccessLevel::AccountAccessLevelFullAccess => {
                "ACCOUNT_ACCESS_LEVEL_FULL_ACCESS"
            }
            AccessLevel::AccountAccessLevelReadOnly => "ACCOUNT_ACCESS_LEVEL_READ_ONLY",
            AccessLevel::AccountAccessLevelNoAccess => "ACCOUNT_ACCESS_LEVEL_NO_ACCESS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ACCOUNT_ACCESS_LEVEL_UNSPECIFIED" => {
                Some(Self::AccountAccessLevelUnspecified)
            }
            "ACCOUNT_ACCESS_LEVEL_FULL_ACCESS" => {
                Some(Self::AccountAccessLevelFullAccess)
            }
            "ACCOUNT_ACCESS_LEVEL_READ_ONLY" => Some(Self::AccountAccessLevelReadOnly),
            "ACCOUNT_ACCESS_LEVEL_NO_ACCESS" => Some(Self::AccountAccessLevelNoAccess),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod users_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct UsersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl UsersServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> UsersServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> UsersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            UsersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Метод получения счетов пользователя.
        pub async fn get_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetAccounts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.UsersService",
                        "GetAccounts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Расчёт маржинальных показателей по счёту.
        pub async fn get_margin_attributes(
            &mut self,
            request: impl tonic::IntoRequest<super::GetMarginAttributesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetMarginAttributesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetMarginAttributes",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.UsersService",
                        "GetMarginAttributes",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Запрос тарифа пользователя.
        pub async fn get_user_tariff(
            &mut self,
            request: impl tonic::IntoRequest<super::GetUserTariffRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetUserTariffResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetUserTariff",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.UsersService",
                        "GetUserTariff",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения информации о пользователе.
        pub async fn get_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInfoResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.UsersService/GetInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.UsersService",
                        "GetInfo",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Запрос открытия счёта в песочнице.
///
/// пустой запрос
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenSandboxAccountRequest {}
/// Номер открытого счёта в песочнице.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OpenSandboxAccountResponse {
    /// Номер счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Запрос закрытия счёта в песочнице.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseSandboxAccountRequest {
    /// Номер счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Результат закрытия счёта в песочнице.
///
/// пустой ответ
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloseSandboxAccountResponse {}
/// Запрос пополнения счёта в песочнице.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandboxPayInRequest {
    /// Номер счёта
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Сумма пополнения счёта в рублях
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<MoneyValue>,
}
/// Результат пополнения счёта, текущий баланс.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandboxPayInResponse {
    /// Текущий баланс счёта
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<MoneyValue>,
}
/// Generated client implementations.
pub mod sandbox_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SandboxServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SandboxServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SandboxServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SandboxServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SandboxServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Метод регистрации счёта в песочнице.
        pub async fn open_sandbox_account(
            &mut self,
            request: impl tonic::IntoRequest<super::OpenSandboxAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OpenSandboxAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/OpenSandboxAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "OpenSandboxAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения счетов в песочнице.
        pub async fn get_sandbox_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAccountsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetAccountsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxAccounts",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "GetSandboxAccounts",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод закрытия счёта в песочнице.
        pub async fn close_sandbox_account(
            &mut self,
            request: impl tonic::IntoRequest<super::CloseSandboxAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CloseSandboxAccountResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/CloseSandboxAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "CloseSandboxAccount",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод выставления торгового поручения в песочнице.
        pub async fn post_sandbox_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/PostSandboxOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "PostSandboxOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод изменения выставленной заявки.
        pub async fn replace_sandbox_order(
            &mut self,
            request: impl tonic::IntoRequest<super::ReplaceOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/ReplaceSandboxOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "ReplaceSandboxOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка активных заявок по счёту в песочнице.
        pub async fn get_sandbox_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOrdersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "GetSandboxOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод отмены торгового поручения в песочнице.
        pub async fn cancel_sandbox_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/CancelSandboxOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "CancelSandboxOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения статуса заявки в песочнице. Заявки хранятся в таблице 7 дней.
        pub async fn get_sandbox_order_state(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOrderStateRequest>,
        ) -> std::result::Result<tonic::Response<super::OrderState>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOrderState",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "GetSandboxOrderState",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения позиций по виртуальному счёту песочницы.
        pub async fn get_sandbox_positions(
            &mut self,
            request: impl tonic::IntoRequest<super::PositionsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PositionsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxPositions",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "GetSandboxPositions",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения операций в песочнице по номеру счёта.
        pub async fn get_sandbox_operations(
            &mut self,
            request: impl tonic::IntoRequest<super::OperationsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::OperationsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOperations",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "GetSandboxOperations",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения операций в песочнице по номеру счета с пагинацией.
        pub async fn get_sandbox_operations_by_cursor(
            &mut self,
            request: impl tonic::IntoRequest<super::GetOperationsByCursorRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetOperationsByCursorResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxOperationsByCursor",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "GetSandboxOperationsByCursor",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения портфолио в песочнице.
        pub async fn get_sandbox_portfolio(
            &mut self,
            request: impl tonic::IntoRequest<super::PortfolioRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PortfolioResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxPortfolio",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "GetSandboxPortfolio",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод пополнения счёта в песочнице.
        pub async fn sandbox_pay_in(
            &mut self,
            request: impl tonic::IntoRequest<super::SandboxPayInRequest>,
        ) -> std::result::Result<
            tonic::Response<super::SandboxPayInResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/SandboxPayIn",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "SandboxPayIn",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения доступного остатка для вывода средств в песочнице.
        pub async fn get_sandbox_withdraw_limits(
            &mut self,
            request: impl tonic::IntoRequest<super::WithdrawLimitsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::WithdrawLimitsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.SandboxService/GetSandboxWithdrawLimits",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.SandboxService",
                        "GetSandboxWithdrawLimits",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Запрос выставления стоп-заявки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostStopOrderRequest {
    /// Deprecated Figi-идентификатор инструмента. Необходимо использовать instrument_id.
    #[deprecated]
    #[prost(string, tag = "1")]
    pub figi: ::prost::alloc::string::String,
    /// Количество лотов.
    #[prost(int64, tag = "2")]
    pub quantity: i64,
    /// Цена за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "3")]
    pub price: ::core::option::Option<Quotation>,
    /// Стоп-цена заявки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "4")]
    pub stop_price: ::core::option::Option<Quotation>,
    /// Направление операции.
    #[prost(enumeration = "StopOrderDirection", tag = "5")]
    pub direction: i32,
    /// Номер счёта.
    #[prost(string, tag = "6")]
    pub account_id: ::prost::alloc::string::String,
    /// Тип экспирации заявки.
    #[prost(enumeration = "StopOrderExpirationType", tag = "7")]
    pub expiration_type: i32,
    /// Тип заявки.
    #[prost(enumeration = "StopOrderType", tag = "8")]
    pub stop_order_type: i32,
    /// Дата и время окончания действия стоп-заявки в часовом поясе UTC. **Для ExpirationType = GoodTillDate заполнение обязательно**.
    #[prost(message, optional, tag = "9")]
    pub expire_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Идентификатор инструмента, принимает значения Figi или instrument_uid.
    #[prost(string, tag = "10")]
    pub instrument_id: ::prost::alloc::string::String,
}
/// Результат выставления стоп-заявки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PostStopOrderResponse {
    /// Уникальный идентификатор стоп-заявки.
    #[prost(string, tag = "1")]
    pub stop_order_id: ::prost::alloc::string::String,
}
/// Запрос получения списка активных стоп-заявок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStopOrdersRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
}
/// Список активных стоп-заявок.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetStopOrdersResponse {
    /// Массив стоп-заявок по счёту.
    #[prost(message, repeated, tag = "1")]
    pub stop_orders: ::prost::alloc::vec::Vec<StopOrder>,
}
/// Запрос отмены выставленной стоп-заявки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelStopOrderRequest {
    /// Идентификатор счёта клиента.
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    /// Уникальный идентификатор стоп-заявки.
    #[prost(string, tag = "2")]
    pub stop_order_id: ::prost::alloc::string::String,
}
/// Результат отмены выставленной стоп-заявки.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelStopOrderResponse {
    /// Время отмены заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "1")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
}
/// Информация о стоп-заявке.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StopOrder {
    /// Идентификатор-идентификатор стоп-заявки.
    #[prost(string, tag = "1")]
    pub stop_order_id: ::prost::alloc::string::String,
    /// Запрошено лотов.
    #[prost(int64, tag = "2")]
    pub lots_requested: i64,
    /// Figi-идентификатор инструмента.
    #[prost(string, tag = "3")]
    pub figi: ::prost::alloc::string::String,
    /// Направление операции.
    #[prost(enumeration = "StopOrderDirection", tag = "4")]
    pub direction: i32,
    /// Валюта стоп-заявки.
    #[prost(string, tag = "5")]
    pub currency: ::prost::alloc::string::String,
    /// Тип стоп-заявки.
    #[prost(enumeration = "StopOrderType", tag = "6")]
    pub order_type: i32,
    /// Дата и время выставления заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "7")]
    pub create_date: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата и время конвертации стоп-заявки в биржевую в часовом поясе UTC.
    #[prost(message, optional, tag = "8")]
    pub activation_date_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Дата и время снятия заявки в часовом поясе UTC.
    #[prost(message, optional, tag = "9")]
    pub expiration_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Цена заявки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "10")]
    pub price: ::core::option::Option<MoneyValue>,
    /// Цена активации стоп-заявки за 1 инструмент. Для получения стоимости лота требуется умножить на лотность инструмента.
    #[prost(message, optional, tag = "11")]
    pub stop_price: ::core::option::Option<MoneyValue>,
    /// instrument_uid идентификатор инструмента.
    #[prost(string, tag = "12")]
    pub instrument_uid: ::prost::alloc::string::String,
}
/// Направление сделки стоп-заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderDirection {
    /// Значение не указано.
    Unspecified = 0,
    /// Покупка.
    Buy = 1,
    /// Продажа.
    Sell = 2,
}
impl StopOrderDirection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopOrderDirection::Unspecified => "STOP_ORDER_DIRECTION_UNSPECIFIED",
            StopOrderDirection::Buy => "STOP_ORDER_DIRECTION_BUY",
            StopOrderDirection::Sell => "STOP_ORDER_DIRECTION_SELL",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STOP_ORDER_DIRECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "STOP_ORDER_DIRECTION_BUY" => Some(Self::Buy),
            "STOP_ORDER_DIRECTION_SELL" => Some(Self::Sell),
            _ => None,
        }
    }
}
/// Тип экспирации стоп-заявке.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderExpirationType {
    /// Значение не указано.
    Unspecified = 0,
    /// Действительно до отмены.
    GoodTillCancel = 1,
    /// Действительно до даты снятия.
    GoodTillDate = 2,
}
impl StopOrderExpirationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopOrderExpirationType::Unspecified => {
                "STOP_ORDER_EXPIRATION_TYPE_UNSPECIFIED"
            }
            StopOrderExpirationType::GoodTillCancel => {
                "STOP_ORDER_EXPIRATION_TYPE_GOOD_TILL_CANCEL"
            }
            StopOrderExpirationType::GoodTillDate => {
                "STOP_ORDER_EXPIRATION_TYPE_GOOD_TILL_DATE"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STOP_ORDER_EXPIRATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "STOP_ORDER_EXPIRATION_TYPE_GOOD_TILL_CANCEL" => Some(Self::GoodTillCancel),
            "STOP_ORDER_EXPIRATION_TYPE_GOOD_TILL_DATE" => Some(Self::GoodTillDate),
            _ => None,
        }
    }
}
/// Тип стоп-заявки.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StopOrderType {
    /// Значение не указано.
    Unspecified = 0,
    /// Take-profit заявка.
    TakeProfit = 1,
    /// Stop-loss заявка.
    StopLoss = 2,
    /// Stop-limit заявка.
    StopLimit = 3,
}
impl StopOrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StopOrderType::Unspecified => "STOP_ORDER_TYPE_UNSPECIFIED",
            StopOrderType::TakeProfit => "STOP_ORDER_TYPE_TAKE_PROFIT",
            StopOrderType::StopLoss => "STOP_ORDER_TYPE_STOP_LOSS",
            StopOrderType::StopLimit => "STOP_ORDER_TYPE_STOP_LIMIT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STOP_ORDER_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "STOP_ORDER_TYPE_TAKE_PROFIT" => Some(Self::TakeProfit),
            "STOP_ORDER_TYPE_STOP_LOSS" => Some(Self::StopLoss),
            "STOP_ORDER_TYPE_STOP_LIMIT" => Some(Self::StopLimit),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod stop_orders_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct StopOrdersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl StopOrdersServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> StopOrdersServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> StopOrdersServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            StopOrdersServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Метод выставления стоп-заявки.
        pub async fn post_stop_order(
            &mut self,
            request: impl tonic::IntoRequest<super::PostStopOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::PostStopOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.StopOrdersService/PostStopOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.StopOrdersService",
                        "PostStopOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод получения списка активных стоп заявок по счёту.
        pub async fn get_stop_orders(
            &mut self,
            request: impl tonic::IntoRequest<super::GetStopOrdersRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetStopOrdersResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.StopOrdersService/GetStopOrders",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.StopOrdersService",
                        "GetStopOrders",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Метод отмены стоп-заявки.
        pub async fn cancel_stop_order(
            &mut self,
            request: impl tonic::IntoRequest<super::CancelStopOrderRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CancelStopOrderResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/tinkoff.public.invest.api.contract.v1.StopOrdersService/CancelStopOrder",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "tinkoff.public.invest.api.contract.v1.StopOrdersService",
                        "CancelStopOrder",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
