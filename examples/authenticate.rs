use bankid::model::{CollectPayload, CancelPayload};
use bankid::model::AuthenticatePayloadBuilder;
use bankid::client::BankID;
use bankid::config::Pkcs12;
use bankid::config::ConfigBuilder;

#[tokio::main]
async fn main() {
    let bank_id = client();

    let payload = AuthenticatePayloadBuilder::default()
        .personal_number("123456789123".to_string())
        .end_user_ip("123.123.123.123".to_string())
        .build()
        .unwrap();

    let authenticate = &bank_id.authenticate(payload).await.unwrap();
    let collect = bank_id.collect(CollectPayload { order_ref: authenticate.clone().order_ref }).await.unwrap();
    let cancel = bank_id.cancel(CancelPayload { order_ref: authenticate.clone().order_ref }).await.unwrap();

    println!("{:#?}", authenticate);
    println!("{:#?}", collect);
    println!("{:#?}", cancel);
}

fn client() -> BankID {
    let pkcs12 = Pkcs12::DER {
        der: CA_TEST.to_vec(),
        password: "qwerty123".to_string(),
    };

    let config = ConfigBuilder::default()
        .pkcs12(pkcs12)
        .build()
        .unwrap();

    BankID::new(config)
}

const CA_TEST: &'static [u8] = include_bytes!("../resources/testcert.p12");