use reqwest::StatusCode;

use crate::{BillingChargeListQuery, CarnetParcelNumber, Error};

mod support {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};
    use std::thread::{self, JoinHandle};

    use crate::Client;

    pub struct Exchange {
        pub request_line: &'static str,
        pub status: u16,
        pub body: &'static str,
    }

    pub struct TestServer {
        base_url: String,
        handle: Option<JoinHandle<()>>,
    }

    impl TestServer {
        pub fn start(exchanges: Vec<Exchange>) -> Self {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let base_url = format!("http://{}", listener.local_addr().unwrap());
            let handle = thread::spawn(move || {
                for exchange in exchanges {
                    let (mut stream, _) = listener.accept().unwrap();
                    let request = read_request(&mut stream);
                    assert_eq!(request.lines().next(), Some(exchange.request_line));
                    let reason = if exchange.status >= 400 {
                        "Error"
                    } else {
                        "OK"
                    };
                    write!(
                        stream,
                        "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                        exchange.status,
                        reason,
                        exchange.body.len(),
                        exchange.body,
                    )
                    .unwrap();
                }
            });
            Self {
                base_url,
                handle: Some(handle),
            }
        }

        pub fn client(&self) -> Client {
            Client::test_billing_client(&self.base_url)
        }

        pub fn finish(mut self) {
            self.handle.take().unwrap().join().unwrap();
        }
    }

    pub(super) fn read_request(stream: &mut TcpStream) -> String {
        let mut bytes = Vec::new();
        let mut buffer = [0; 4096];
        loop {
            let count = stream.read(&mut buffer).unwrap();
            if count == 0 {
                break;
            }
            bytes.extend_from_slice(&buffer[..count]);
            if bytes.windows(4).any(|window| window == b"\r\n\r\n") {
                break;
            }
        }
        String::from_utf8(bytes).unwrap()
    }

    pub fn auth() -> Exchange {
        Exchange {
            request_line: "POST /v1/authorize HTTP/1.1",
            status: 200,
            body: r#"{"access_token":"fixture-token","expires_in":3600}"#,
        }
    }
}

use support::{Exchange, TestServer};

#[tokio::test]
async fn charge_list_sends_encoded_filters_and_deserializes_candidate_fields() {
    let response = r#"{
        "code": 200,
        "data": [
            {
                "id": 711008222,
                "total": 500,
                "status": "paid",
                "custom_id": "invoice/42",
                "created_at": "2024-04-30 20:23:31",
                "customer": {
                    "name": "Gorbadoc Oldbuck",
                    "phone_number": "5144916523",
                    "cpf": "94271564656"
                },
                "payment": {
                    "payment_method": "banking_billet",
                    "paid_value": 500,
                    "banking_billet": { "expire_at": "2024-06-30T15:00:00.000Z" }
                }
            },
            {
                "id": 700030468,
                "total": 1000,
                "status": "settled",
                "custom_id": "plan-42",
                "customer": { "name": "Empresa Exemplo", "cnpj": "99794567000144" },
                "payment": {
                    "payment_method": "carnet",
                    "carnet": { "parcel": "12", "expire_at": "2025-03-05T15:00:00.000Z" }
                }
            }
        ],
        "params": {
            "begin_date": "2024-05-01T00:00:00.000Z",
            "end_date": "2024-05-30T00:00:00.000Z",
            "pagination": { "limit": 25, "offset": 25, "page": 2 }
        }
    }"#;
    let server = TestServer::start(vec![
        support::auth(),
        Exchange {
            request_line: "GET /v1/charges?charge_type=billet&begin_date=2024-05-01&end_date=2024-05-30&date_of=creation&custom_id=invoice%2F42&limit=25&page=2&offset=25 HTTP/1.1",
            status: 200,
            body: response,
        },
    ]);

    let result = server
        .client()
        .billing_charges_list(&BillingChargeListQuery {
            charge_type: "billet".into(),
            begin_date: "2024-05-01".into(),
            end_date: "2024-05-30".into(),
            date_of: Some(crate::BillingChargeDateOf::Creation),
            custom_id: Some("invoice/42".into()),
            limit: Some(25),
            page: Some(2),
            offset: Some(25),
        })
        .await
        .unwrap();

    assert_eq!(result.data[0].id, 711008222);
    assert_eq!(result.data[0].total, 500);
    assert_eq!(
        result.data[0].customer.as_ref().unwrap().cpf.as_deref(),
        Some("94271564656")
    );
    assert_eq!(
        result.data[0]
            .payment
            .as_ref()
            .unwrap()
            .banking_billet
            .as_ref()
            .unwrap()
            .expire_at
            .as_deref(),
        Some("2024-06-30T15:00:00.000Z")
    );
    assert_eq!(result.params.pagination.page, 2);
    assert_eq!(result.data[1].carnet_id, None);
    let listed_carnet = result.data[1]
        .payment
        .as_ref()
        .unwrap()
        .carnet
        .as_ref()
        .unwrap();
    assert_eq!(listed_carnet.carnet_id, None);
    assert_eq!(listed_carnet.parcel.get(), 12);
    assert_eq!(
        result.data[1].customer.as_ref().unwrap().tax_id(),
        Some("99794567000144")
    );
    server.finish();
}

#[tokio::test]
async fn accurate_reads_accept_documented_charge_and_carnet_shapes() {
    let charge_response = r#"{
        "code": 200,
        "data": {
            "charge_id": 1234567,
            "total": 8900,
            "status": "waiting",
            "custom_id": "invoice-42",
            "customer": {
                "name": "Empresa Exemplo",
                "email": "billing@example.com",
                "juridical_person": { "corporate_name": "Empresa Exemplo Ltda", "cnpj": "99794567000144" }
            },
            "payment": {
                "method": "banking_billet",
                "banking_billet": { "expire_at": "2026-10-30" }
            }
        }
    }"#;
    let carnet_response = r#"{
        "code": 200,
        "data": {
            "carnet_id": 14196,
            "status": "active",
            "repeats": 2,
            "value": 22500,
            "custom_id": "plan-42",
            "notification_url": null,
            "split_items": false,
            "charges": [
                { "charge_id": 184208, "status": "waiting", "parcel": 1, "expire_at": "2026-10-20", "value": 22500 },
                { "charge_id": 184209, "status": "waiting", "parcel": "2", "expire_at": "2026-11-20", "value": 22500 }
            ]
        }
    }"#;
    let server = TestServer::start(vec![
        support::auth(),
        Exchange {
            request_line: "GET /v1/charge/1234567 HTTP/1.1",
            status: 200,
            body: charge_response,
        },
        Exchange {
            request_line: "GET /v1/carnet/14196 HTTP/1.1",
            status: 200,
            body: carnet_response,
        },
    ]);
    let client = server.client();

    let charge = client.billing_charge_read(1234567).await.unwrap();
    assert_eq!(charge.data.custom_id.as_deref(), Some("invoice-42"));
    assert_eq!(
        charge.data.customer.unwrap().tax_id(),
        Some("99794567000144")
    );
    assert_eq!(
        charge
            .data
            .payment
            .unwrap()
            .banking_billet
            .unwrap()
            .expire_at
            .as_deref(),
        Some("2026-10-30")
    );

    let carnet = client.carnet_detail_get(14196).await.unwrap();
    assert_eq!(carnet.data.carnet_id, 14196);
    assert_eq!(
        carnet.data.charges[0].parcel,
        CarnetParcelNumber::new(1).unwrap()
    );
    assert_eq!(carnet.data.charges[1].parcel.get(), 2);
    assert_eq!(carnet.data.charges[1].charge_id, 184209);
    server.finish();
}

#[tokio::test]
async fn cancellations_parse_code_only_responses() {
    let server = TestServer::start(vec![
        support::auth(),
        Exchange {
            request_line: "PUT /v1/charge/123/cancel HTTP/1.1",
            status: 200,
            body: r#"{"code":200}"#,
        },
        Exchange {
            request_line: "PUT /v1/carnet/456/cancel HTTP/1.1",
            status: 200,
            body: r#"{"code":200}"#,
        },
        Exchange {
            request_line: "PUT /v1/carnet/456/parcel/2/cancel HTTP/1.1",
            status: 200,
            body: r#"{"code":200}"#,
        },
    ]);
    let client = server.client();

    assert_eq!(
        client
            .billing_charge_cancel_response(123)
            .await
            .unwrap()
            .code,
        200
    );
    assert_eq!(client.carnet_cancel_response(456).await.unwrap().code, 200);
    assert_eq!(
        client
            .carnet_cancel_parcel_response(456, 2)
            .await
            .unwrap()
            .code,
        200
    );
    server.finish();
}

#[tokio::test]
async fn http_failures_expose_typed_status_and_body() {
    let server = TestServer::start(vec![
        support::auth(),
        Exchange {
            request_line: "GET /v1/charge/123 HTTP/1.1",
            status: 409,
            body: r#"{"error":"charge_already_paid"}"#,
        },
    ]);
    let error = server.client().billing_charge_read(123).await.unwrap_err();

    assert_eq!(error.status_code(), Some(StatusCode::CONFLICT));
    assert_eq!(
        error.response_body(),
        Some(r#"{"error":"charge_already_paid"}"#)
    );
    assert!(matches!(error, Error::RequestFailed { .. }));
    server.finish();
}

#[test]
fn carnet_parcel_number_rejects_zero_and_non_numeric_strings() {
    assert!(serde_json::from_str::<CarnetParcelNumber>("0").is_err());
    assert!(serde_json::from_str::<CarnetParcelNumber>(r#""second""#).is_err());
}

#[test]
fn carnet_customer_round_trips_optional_address_without_changing_legacy_wire_shape() {
    let address = serde_json::json!({
        "street":"Rua Um", "number":"10", "neighborhood":"Centro",
        "zipcode":"30110000", "city":"Belo Horizonte", "state":"MG", "complement":"Sala 2"
    });
    let customer: crate::CarnetCustomer = serde_json::from_value(serde_json::json!({
        "name":"Example", "cpf":"94271564656", "address":address
    }))
    .unwrap();
    let encoded = serde_json::to_value(customer).unwrap();
    assert_eq!(encoded["address"], address);
    let legacy = serde_json::json!({"name":"Example", "cpf":"94271564656"});
    let customer: crate::CarnetCustomer = serde_json::from_value(legacy.clone()).unwrap();
    assert_eq!(serde_json::to_value(customer).unwrap(), legacy);
}

#[test]
fn charge_read_preserves_items_for_bounded_carnet_verification() {
    let fixture = serde_json::json!({"code":200,"data":{
        "charge_id":11,"total":1000,"status":"waiting",
        "items":[{"name":"Service", "value":1000, "amount":1}]
    }});
    let read: crate::BillingChargeReadResponse = serde_json::from_value(fixture.clone()).unwrap();
    assert_eq!(
        serde_json::to_value(read).unwrap()["data"]["items"],
        fixture["data"]["items"]
    );
}

async fn oauth_request_count(
    initial_ttl: u64,
    refresh_ttl: u64,
) -> (Result<crate::BillingChargeReadResponse, Error>, Vec<String>) {
    use std::{
        io::Write,
        net::TcpListener,
        sync::{
            Arc, Mutex,
            atomic::{AtomicBool, Ordering},
        },
        thread,
        time::Duration,
    };
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let finished = Arc::new(AtomicBool::new(false));
    let requests = Arc::new(Mutex::new(Vec::new()));
    let server_finished = finished.clone();
    let server_requests = requests.clone();
    let server = thread::spawn(move || {
        let mut auths = 0;
        let mut reads = 0;
        while !server_finished.load(Ordering::SeqCst) {
            let (mut stream, _) = match listener.accept() {
                Ok(stream) => stream,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(1));
                    continue;
                }
                Err(error) => panic!("fixture listener: {error}"),
            };
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let request = support::read_request(&mut stream);
            let first = request.lines().next().unwrap().to_owned();
            server_requests.lock().unwrap().push(first.clone());
            let (status, body) = if first == "POST /v1/authorize HTTP/1.1" {
                auths += 1;
                let ttl = if auths == 1 { initial_ttl } else { refresh_ttl };
                (
                    200,
                    format!(r#"{{"access_token":"token-{auths}","expires_in":{ttl}}}"#),
                )
            } else {
                assert_eq!(first, "GET /v1/charge/11 HTTP/1.1");
                reads += 1;
                if reads == 1 {
                    (401, "{}".into())
                } else {
                    (
                        200,
                        r#"{"code":200,"data":{"charge_id":11,"total":1000,"status":"waiting"}}"#
                            .into(),
                    )
                }
            };
            write!(stream, "HTTP/1.1 {status} Fixture\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
        }
    });
    let result = crate::Client::test_billing_client(&base)
        .billing_charge_read(11)
        .await;
    finished.store(true, Ordering::SeqCst);
    server.join().unwrap();
    let requests = requests.lock().unwrap().clone();
    (result, requests)
}

#[tokio::test]
async fn unusable_oauth_ttl_is_rejected_before_api_request_or_extra_refresh() {
    let (result, requests) = oauth_request_count(30, 30).await;
    assert!(matches!(result, Err(Error::AuthUnavailable)));
    assert_eq!(requests, ["POST /v1/authorize HTTP/1.1"]);
    let (result, requests) = oauth_request_count(3600, 30).await;
    assert!(matches!(result, Err(Error::AuthUnavailable)));
    assert_eq!(
        requests,
        [
            "POST /v1/authorize HTTP/1.1",
            "GET /v1/charge/11 HTTP/1.1",
            "POST /v1/authorize HTTP/1.1"
        ]
    );
}

#[tokio::test]
async fn cold_401_retry_reserves_at_most_four_requests() {
    let (result, requests) = oauth_request_count(3600, 3600).await;
    assert_eq!(result.unwrap().data.charge_id, 11);
    assert_eq!(
        requests,
        [
            "POST /v1/authorize HTTP/1.1",
            "GET /v1/charge/11 HTTP/1.1",
            "POST /v1/authorize HTTP/1.1",
            "GET /v1/charge/11 HTTP/1.1"
        ]
    );
}

#[tokio::test]
async fn charge_list_preserves_optional_date_selection() {
    use crate::BillingChargeDateOf;
    for (date_of, request_line) in [
        (
            None,
            "GET /v1/charges?charge_type=billet&begin_date=2024-05-01&end_date=2024-05-30 HTTP/1.1",
        ),
        (
            Some(BillingChargeDateOf::Payment),
            "GET /v1/charges?charge_type=billet&begin_date=2024-05-01&end_date=2024-05-30&date_of=payment HTTP/1.1",
        ),
        (
            Some(BillingChargeDateOf::Expired),
            "GET /v1/charges?charge_type=billet&begin_date=2024-05-01&end_date=2024-05-30&date_of=expired HTTP/1.1",
        ),
    ] {
        let server = TestServer::start(vec![
            support::auth(),
            Exchange {
                request_line,
                status: 200,
                body: r#"{"code":200,"data":[],"params":{"begin_date":"2024-05-01","end_date":"2024-05-30","pagination":{"limit":100,"offset":0,"page":1}}}"#,
            },
        ]);
        let response = server
            .client()
            .billing_charges_list(&BillingChargeListQuery {
                charge_type: "billet".into(),
                begin_date: "2024-05-01".into(),
                end_date: "2024-05-30".into(),
                date_of,
                custom_id: None,
                limit: None,
                page: None,
                offset: None,
            })
            .await
            .unwrap();
        assert!(response.data.is_empty());
        server.finish();
    }
}
