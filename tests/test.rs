#[cfg(feature = "__sync")]
pub use test as maybe_async_test;

use maybe_async::maybe_async;
#[cfg(feature = "__async")]
pub use tokio::test as maybe_async_test;

use bankid::config::{API_URL_TEST, CA_TEST};
use bankid::{
    client::BankID,
    config::{ConfigBuilder, Pkcs12},
    model::{AuthenticatePayloadBuilder, CancelPayload, CollectPayload},
};

#[maybe_async]
#[maybe_async_test]
async fn test_authenticate() {
    let bank_id = client();

    let payload = AuthenticatePayloadBuilder::default()
        .end_user_ip("123.123.123.123".to_string())
        .build()
        .unwrap();

    bank_id.authenticate(payload).await.unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_sign() {}

#[maybe_async]
#[maybe_async_test]
async fn test_collect() {
    let bank_id = client();

    let payload = AuthenticatePayloadBuilder::default()
        .end_user_ip("123.123.123.123".to_string())
        .build()
        .unwrap();

    let authenticate = bank_id.authenticate(payload).await.unwrap();

    bank_id
        .collect(CollectPayload {
            order_ref: authenticate.order_ref,
        })
        .await
        .unwrap();
}

#[maybe_async]
#[maybe_async_test]
async fn test_cancel() {
    let bank_id = client();

    let payload = AuthenticatePayloadBuilder::default()
        .end_user_ip("123.123.123.123".to_string())
        .build()
        .unwrap();

    let authenticate = bank_id.authenticate(payload).await.unwrap();

    bank_id
        .cancel(CancelPayload {
            order_ref: authenticate.order_ref,
        })
        .await
        .unwrap();
}

fn client() -> BankID {
    let pkcs12 = Pkcs12::Der {
        der: P12_TEST.to_vec(),
        password: "qwerty123".to_string(),
    };

    let config = ConfigBuilder::default()
        .pkcs12(pkcs12)
        .url(API_URL_TEST.to_string())
        .ca(CA_TEST.to_string())
        .build()
        .unwrap();

    BankID::new(config)
}

const P12_TEST: &[u8] = include_bytes!("../resources/testcert.p12");
