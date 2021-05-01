use bankid::{
    client::BankID,
    config::{ConfigBuilder, Pkcs12},
    model::{AuthenticatePayloadBuilder, CancelPayload, CollectPayload},
};
use bankid::config::CA_TEST;

#[tokio::main]
async fn main() {
    let bank_id = client();

    let payload = AuthenticatePayloadBuilder::default()
        .personal_number("123456789123".to_string())
        .end_user_ip("123.123.123.123".to_string())
        .build()
        .unwrap();

    let authenticate = &bank_id.authenticate(payload).await.unwrap();
    let collect = bank_id.collect(CollectPayload { order_ref: authenticate.clone().order_ref })
        .await
        .unwrap();

    let cancel = bank_id.cancel(CancelPayload { order_ref: authenticate.clone().order_ref })
        .await
        .unwrap();

    println!("{:#?}", authenticate);
    println!("{:#?}", collect);
    println!("{:#?}", cancel);
}

fn client() -> BankID {
    let pkcs12 = Pkcs12::Der {
        der: P12_TEST.to_vec(),
        password: "qwerty123".to_string(),
    };

    let config = ConfigBuilder::default()
        .pkcs12(pkcs12)
        .url("https://appapi2.test.bankid.com/rp/v5.1".to_string())
        .ca(CA_TEST.to_string())
        .build()
        .unwrap();

    BankID::new(config)
}

const P12_TEST: &'static [u8] = include_bytes!("../resources/testcert.p12");
