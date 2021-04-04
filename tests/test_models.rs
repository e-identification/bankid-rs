use bankid::model::*;

#[test]
fn test_authenticate() {
    let json_str = r#"{"orderRef":"1b9839fe-9b79-4b71-8534-618f4c7ea12b","autoStartToken":"f70c5c18-9a0c-4299-93b8-ed7b0fc4bb0d","qrStartToken":"82a40faa-9cb1-4acc-a7fd-bd76b0aaf6c6","qrStartSecret":"843050ed-0a4c-4135-b33c-acc08783a61b"}"#;

    let authenticate: Authenticate = serde_json::from_str(&json_str).unwrap();

    assert_eq!(
        "1b9839fe-9b79-4b71-8534-618f4c7ea12b".to_string(),
        authenticate.order_ref
    );
    assert_eq!(
        "f70c5c18-9a0c-4299-93b8-ed7b0fc4bb0d".to_string(),
        authenticate.auto_start_token
    );
    assert_eq!(
        "82a40faa-9cb1-4acc-a7fd-bd76b0aaf6c6".to_string(),
        authenticate.qr_start_token
    );
    assert_eq!(
        "843050ed-0a4c-4135-b33c-acc08783a61b".to_string(),
        authenticate.qr_start_secret
    );
}

#[test]
fn test_sign() {}

#[test]
fn test_collect() {}

#[test]
fn test_cancel() {}
