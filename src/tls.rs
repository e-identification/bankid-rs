use std::{fs::File, io::Read};

use crate::error::{Error, Result};

pub struct Identity {}

impl Identity {
    pub fn from_pkcs12_file(path: &str, password: &str) -> Result<reqwest::Identity> {
        let mut buf = Vec::new();

        let mut file = File::open(path).map_err(|source| Error::ReadError { source })?;

        file.read_to_end(&mut buf)
            .map_err(|source| Error::ReadError { source })?;

        match reqwest::Identity::from_pkcs12_der(&buf, password) {
            Ok(identify) => Ok(identify),
            Err(error) => Err(Error::Pkcs12Error { source: error }),
        }
    }

    pub fn from_vec(der: Vec<u8>, password: &str) -> Result<reqwest::Identity> {
        match reqwest::Identity::from_pkcs12_der(&der, password) {
            Ok(identify) => Ok(identify),
            Err(error) => Err(Error::Pkcs12Error { source: error }),
        }
    }
}

pub struct Certificate {}

impl Certificate {
    #[allow(dead_code)]
    pub fn from_file(path: &str) -> Result<Vec<reqwest::Certificate>> {
        let mut buf = Vec::new();

        let mut file = File::open(path).map_err(|source| Error::ReadError { source })?;

        file.read_to_end(&mut buf)
            .map_err(|source| Error::ReadError { source })?;

        let pems = pem::parse_many(&buf).map_err(|source| Error::PemError { source })?;

        pems.into_iter()
            .map(|pem| {
                reqwest::Certificate::from_pem(&pem::encode(&pem).into_bytes())
                    .map_err(|source| Error::CertificateError { source })
            })
            .collect::<Result<Vec<_>>>()
    }

    #[allow(dead_code)]
    pub fn from_string(subject: &str) -> Result<Vec<reqwest::Certificate>> {
        let buf: Vec<u8> = subject.into();
        let pems = pem::parse_many(buf).map_err(|source| Error::PemError { source })?;

        pems.into_iter()
            .map(|pem| {
                reqwest::Certificate::from_pem(&pem::encode(&pem).into_bytes())
                    .map_err(|source| Error::CertificateError { source })
            })
            .collect::<Result<Vec<_>>>()
    }
}
