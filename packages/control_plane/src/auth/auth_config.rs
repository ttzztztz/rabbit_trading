use rabbit_trading_core::model::common::error::Error;
use std::str::FromStr;

#[derive(PartialEq, Eq)]
pub enum AuthConfig {
    NoAuth,
    BasicAuth,
    BearerAuth,
    KerberosAuth,
}

impl FromStr for AuthConfig {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const UNKNOWN_AUTH_ERROR_CODE: &'static str = "UNKNOWN_AUTH_ERROR";

        match s.to_lowercase().as_str() {
            "noauth" => Result::Ok(Self::NoAuth),
            "basic" | "basicauth" => Result::Ok(Self::BasicAuth),
            "bearer" | "bearerauth" => Result::Ok(Self::BearerAuth),
            "krb" | "kerberos" | "kerberosauth" => Result::Ok(Self::KerberosAuth),
            unknown_auth_kind => Result::Err(Error {
                code: UNKNOWN_AUTH_ERROR_CODE.to_owned(),
                message: format!("unknown {}", unknown_auth_kind),
            }),
        }
    }
}

impl ToString for AuthConfig {
    fn to_string(&self) -> String {
        match self {
            Self::NoAuth => "NoAuth".to_owned(),
            Self::BasicAuth => "BasicAuth".to_owned(),
            Self::BearerAuth => "BearerAuth".to_owned(),
            Self::KerberosAuth => "KerberosAuth".to_owned(),
        }
    }
}
