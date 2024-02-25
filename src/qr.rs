use crate::error::Error;
use ring::hmac::{sign, Key, HMAC_SHA256};

pub fn generate_qr_code_data(qr_start_token: &str, qr_start_secret: &str, seconds: usize) -> Result<String, Error> {
    let hmac_key = Key::new(HMAC_SHA256, qr_start_secret.as_bytes());
    let sig = sign(&hmac_key, format!("{}", seconds).as_bytes());
    let res = format!("bankid.{}.{}.{}", qr_start_token, seconds, hex::encode(sig));
    Ok(res)
}

#[test]
fn test_qr_0() {
    let res = generate_qr_code_data(
        "67df3917-fa0d-44e5-b327-edcc928297f8",
        "d28db9a7-4cde-429e-a983-359be676944c",
        0,
    )
    .unwrap();
    assert_eq!(res, "bankid.67df3917-fa0d-44e5-b327-edcc928297f8.0.dc69358e712458a66a7525beef148ae8526b1c71610eff2c16cdffb4cdac9bf8".to_string())
}

#[test]
fn test_qr_1() {
    let res = generate_qr_code_data(
        "67df3917-fa0d-44e5-b327-edcc928297f8",
        "d28db9a7-4cde-429e-a983-359be676944c",
        1,
    )
    .unwrap();
    assert_eq!(res, "bankid.67df3917-fa0d-44e5-b327-edcc928297f8.1.949d559bf23403952a94d103e67743126381eda00f0b3cbddbf7c96b1adcbce2".to_string())
}
