//! BankID configuration objects.
//!
//! Used to populate [`Config`] that is ultimately used to construct a [`Client`][crate::Client].

use std::convert::{TryFrom, TryInto};

use derive_builder::Builder;

use crate::{
    error::Error,
    tls::{Certificate, Identity},
};

#[derive(Debug, Clone)]
pub enum Pkcs12 {
    File { path: String, password: String },
    Der { der: Vec<u8>, password: String },
}

impl TryFrom<Pkcs12> for reqwest::Identity {
    type Error = Error;

    fn try_from(pkcs12: Pkcs12) -> Result<Self, Self::Error> {
        match pkcs12 {
            Pkcs12::File { path, password } => Ok(Identity::from_pkcs12_file(path.as_str(), password.as_str())?),
            Pkcs12::Der { der, password } => Ok(Identity::from_vec(der, password.as_str())?),
        }
    }
}

#[derive(Builder, Debug, Clone)]
pub struct Config {
    pub pkcs12: Pkcs12,
    #[builder(default = "String::from(API_URL_PROD)")]
    pub url: String,
    #[builder(default = "String::from(CA_PROD)")]
    pub ca: String,
}

impl TryFrom<Config> for reqwest::ClientBuilder {
    type Error = Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let certificates = Certificate::from_string(config.ca.as_str())?;

        let mut builder = Self::new();

        builder = builder.identity(config.pkcs12.try_into()?);

        for cert in certificates {
            builder = builder.add_root_certificate(cert);
        }

        Ok(builder)
    }
}

impl TryFrom<Config> for reqwest::blocking::ClientBuilder {
    type Error = Error;

    fn try_from(config: Config) -> Result<Self, Self::Error> {
        let certificates = Certificate::from_string(config.ca.as_str())?;

        let mut builder = Self::new();

        builder = builder.identity(config.pkcs12.try_into()?);

        for cert in certificates {
            builder = builder.add_root_certificate(cert);
        }

        Ok(builder)
    }
}

#[allow(dead_code)]
pub const API_URL_TEST: &str = "https://appapi2.test.bankid.com/rp/v5.1";
#[allow(dead_code)]
pub const API_URL_PROD: &str = "https://appapi2.bankid.com/rp/v5.1";

#[allow(dead_code)]
pub const CA_TEST: &str = include_str!("../../resources/test.ca");
#[allow(dead_code)]
pub const CA_PROD: &str = include_str!("../../resources/production.ca");
