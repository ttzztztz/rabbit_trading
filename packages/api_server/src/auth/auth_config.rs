use anyhow::{anyhow, Error};
use std::str::FromStr;

#[derive(PartialEq, Eq)]
pub enum AuthConfig {
    NoAuth,
    BasicAuth,    // todo: support this kind of http auth
    BearerAuth,   // todo: support this kind of http auth
    KerberosAuth, // todo: support this kind of http auth
}

impl FromStr for AuthConfig {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "noauth" => Result::Ok(Self::NoAuth),
            "basic" | "basicauth" => Result::Ok(Self::BasicAuth),
            "bearer" | "bearerauth" => Result::Ok(Self::BearerAuth),
            "krb" | "kerberos" | "kerberosauth" => Result::Ok(Self::KerberosAuth),
            unknown_auth_kind => Result::Err(anyhow!("UNKNOWN_AUTH kind: {}", unknown_auth_kind)),
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
