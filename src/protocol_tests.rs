//! Synthesized local fixtures derived from the approved documented field relationships.
//! These are not captured provider responses and do not prove settlement or live acceptance.
use crate::test_support::{fixture, fixture_auth as auth};
use crate::*;
use serde_json::{Value, json};

fn body(request: &str) -> Value {
    serde_json::from_str(request.split_once("\r\n\r\n").unwrap().1).unwrap()
}
fn target(request: &str) -> &str {
    request.lines().next().unwrap()
}
fn query(request: &str) -> Vec<(String, String)> {
    let path = target(request).split_whitespace().nth(1).unwrap();
    reqwest::Url::parse(&format!("http://fixture{path}"))
        .unwrap()
        .query_pairs()
        .into_owned()
        .collect()
}

#[tokio::test]
async fn carnet_actions_keep_routes_and_parse_code_without_detail_reads() {
    let (client, server) = fixture(
        std::iter::once(auth())
            .chain((0..8).map(|_| (200, r#"{"code":200}"#.into())))
            .collect(),
    );
    let email = CarnetResendRequest {
        email: "billing@example.com".into(),
    };
    assert_eq!(
        client
            .carnet_update_metadata(
                12,
                &CarnetMetadataRequest {
                    custom_id: Some("invoice".into()),
                    notification_url: None
                }
            )
            .await
            .unwrap()
            .code,
        200
    );
    assert_eq!(
        client
            .carnet_update_parcel(
                12,
                2,
                &CarnetParcelRequest {
                    expire_at: "2026-12-01".into()
                }
            )
            .await
            .unwrap()
            .code,
        200
    );
    assert_eq!(
        client
            .carnet_update_parcels(
                12,
                &CarnetParcelsRequest {
                    parcels: vec![CarnetParcel {
                        parcel: 2,
                        expire_at: "2026-12-01".into()
                    }]
                }
            )
            .await
            .unwrap()
            .code,
        200
    );
    assert_eq!(client.carnet_resend(12, &email).await.unwrap().code, 200);
    assert_eq!(
        client
            .carnet_resend_parcel(12, 2, &email)
            .await
            .unwrap()
            .code,
        200
    );
    assert_eq!(
        client
            .carnet_add_history(
                12,
                &CarnetHistoryRequest {
                    description: "local fixture".into()
                }
            )
            .await
            .unwrap()
            .code,
        200
    );
    assert_eq!(client.carnet_settle(12).await.unwrap().code, 200);
    assert_eq!(client.carnet_settle_parcel(12, 2).await.unwrap().code, 200);
    let requests = server.join().unwrap();
    assert_eq!(
        requests[1..].iter().map(|r| target(r)).collect::<Vec<_>>(),
        [
            "PUT /v1/carnet/12/metadata HTTP/1.1",
            "PUT /v1/carnet/12/parcel/2 HTTP/1.1",
            "PUT /v1/carnet/12/parcels HTTP/1.1",
            "POST /v1/carnet/12/resend HTTP/1.1",
            "POST /v1/carnet/12/parcel/2/resend HTTP/1.1",
            "POST /v1/carnet/12/history HTTP/1.1",
            "PUT /v1/carnet/12/settle HTTP/1.1",
            "PUT /v1/carnet/12/parcel/2/settle HTTP/1.1"
        ]
    );
    assert_eq!(body(&requests[4]), json!({"email":"billing@example.com"}));
    assert_eq!(body(&requests[5]), json!({"email":"billing@example.com"}));
    assert_eq!(body(&requests[2]), json!({"expire_at":"2026-12-01"}));
}

#[tokio::test]
async fn carnet_list_returns_child_charges_and_metadata_in_one_page() {
    let params = json!({"begin_date":"2026-11-01","end_date":"2026-11-03","pagination":{"limit":100,"offset":0,"page":1}});
    let child = json!({"id":17,"total":1000,"status":"waiting","payment":{"payment_method":"carnet","carnet":{"parcel":"2","expire_at":"2026-12-01"}}});
    let (client, server) = fixture(vec![
        auth(),
        (
            200,
            json!({"code":200,"data":[],"params":params}).to_string(),
        ),
        (
            200,
            json!({"code":200,"data":[child],"params":params}).to_string(),
        ),
    ]);
    assert!(
        client
            .carnet_list("2026-11-01+00:00", "2026-11-03")
            .await
            .unwrap()
            .data
            .is_empty()
    );
    let list = client
        .carnet_list("2026-11-01+00:00", "2026-11-03")
        .await
        .unwrap();
    assert_eq!(
        list.data[0]
            .payment
            .as_ref()
            .unwrap()
            .carnet
            .as_ref()
            .unwrap()
            .parcel
            .get(),
        2
    );
    assert_eq!(list.params.pagination.limit, 100);
    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 3);
    for request in &requests[1..] {
        assert_eq!(
            query(request),
            vec![
                ("charge_type".into(), "carnet".into()),
                ("begin_date".into(), "2026-11-01+00:00".into()),
                ("end_date".into(), "2026-11-03".into())
            ]
        );
    }
}

#[tokio::test]
async fn cob_and_cobv_mutations_send_correct_calendar_and_value_objects() {
    let immediate =
        json!({"calendario":{"expiracao":3600},"valor":{"original":"10.00"},"chave":"payer"});
    let due = json!({"calendario":{"dataDeVencimento":"2026-12-01","validadeAposVencimento":5},"valor":{"original":"10.00","multa":{"modalidade":2,"valorPerc":"2.00"},"juros":{"modalidade":3,"valorPerc":"0.10"},"desconto":{"modalidade":1,"descontoDataFixa":[{"data":"2026-11-30","valorPerc":"1.00"}]},"abatimento":["opaque",1]},"chave":"payer"});
    let response = json!({"txid":"local","calendario":{"criacao":"2026-11-03T00:00:00Z","expiracao":"3600"},"pixCopiaECola":"fixture-code"});
    let (client,server)=fixture(vec![auth(),(201,response.to_string()),(201,json!({"txid":"due","valor":due["valor"],"calendario":{"criacao":"2026-11-03","dataDeVencimento":"2026-12-01"},"pixCopiaECola":"due-code"}).to_string())]);
    let cob = client
        .cob_create(&serde_json::from_value(immediate.clone()).unwrap())
        .await
        .unwrap();
    assert_eq!(cob.calendario.unwrap().expiracao, Some(json!("3600")));
    assert_eq!(cob.pix_copia_e_cola.as_deref(), Some("fixture-code"));
    let cobv = client
        .cobv_create("due/id", &serde_json::from_value(due.clone()).unwrap())
        .await
        .unwrap();
    assert_eq!(cobv.valor.unwrap().abatimento, Some(json!(["opaque", 1])));
    assert_eq!(
        cobv.calendario.unwrap().criacao.as_deref(),
        Some("2026-11-03")
    );
    let requests = server.join().unwrap();
    assert_eq!(target(&requests[1]), "POST /v2/cob HTTP/1.1");
    assert_eq!(body(&requests[1]), immediate);
    assert_eq!(target(&requests[2]), "PUT /v2/cobv/due%2Fid HTTP/1.1");
    assert_eq!(body(&requests[2]), due);
    let minimal: CobResponse = serde_json::from_value(json!({"txid":"minimal"})).unwrap();
    assert!(minimal.calendario.is_none());
    let numeric: CobResponse =
        serde_json::from_value(json!({"txid":"numeric","calendario":{"expiracao":3600}})).unwrap();
    assert_eq!(numeric.calendario.unwrap().expiracao, Some(json!(3600)));
}

#[tokio::test]
async fn pix_charge_lists_encode_required_dates_and_preserve_envelopes() {
    let params = json!({"inicio":"2026-11-01","fim":"2026-11-03","paginacao":{"paginaAtual":0,"quantidadeTotalDeItens":1}});
    let (client,server)=fixture(vec![auth(),(200,json!({"parametros":params,"cobs":[]}).to_string()),(200,json!({"parametros":params,"cobs":[{"txid":"immediate"}]}).to_string()),(200,json!({"parametros":params,"cobs":[]}).to_string()),(200,json!({"parametros":params,"cobs":[{"txid":"due","valor":{"original":"10.00","multa":{"modalidade":2,"valorPerc":"1.00"}}}]}).to_string())]);
    let q = PixChargeListQuery {
        inicio: "2026-11-01T00:00:00+03:00".into(),
        fim: "2026-11-03".into(),
        cpf: Some("123/?".into()),
        status: Some("ATIVA &".into()),
    };
    assert!(client.cob_list(&q).await.unwrap().cobs.is_empty());
    let list = client.cob_list(&q).await.unwrap();
    assert_eq!(list.cobs[0].txid, "immediate");
    assert_eq!(list.parametros["paginacao"]["paginaAtual"], 0);
    assert!(client.cobv_list(&q).await.unwrap().cobs.is_empty());
    let due = client.cobv_list(&q).await.unwrap();
    assert!(due.cobs[0].valor.as_ref().unwrap().multa.is_some());
    assert_eq!(due.parametros["fim"], "2026-11-03");
    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 5);
    for (i, request) in requests[1..].iter().enumerate() {
        assert!(target(request).starts_with(if i < 2 {
            "GET /v2/cob?"
        } else {
            "GET /v2/cobv?"
        }));
        assert_eq!(
            query(request),
            vec![
                ("inicio".into(), q.inicio.clone()),
                ("fim".into(), q.fim.clone()),
                ("cpf".into(), "123/?".into()),
                ("status".into(), "ATIVA &".into())
            ]
        );
    }
}

#[tokio::test]
async fn webhooks_configure_by_key_and_keep_get_list_metadata() {
    let (client,server)=fixture(vec![auth(),(201,"arbitrary bounded bytes".into()),(200,r#"{"webhookUrl":"https://example.com/callback","criacao":"2026-11-03"}"#.into()),(200,r#"{"parametros":{"inicio":"2026-11-01"},"webhooks":[{"webhookUrl":"https://example.com/callback","chave":"key"}]}"#.into()),(204,String::new())]);
    client
        .webhook_configure(
            "a+tag@example.com",
            &WebhookPayload {
                webhook_url: "https://example.com/callback".into(),
            },
        )
        .await
        .unwrap();
    assert_eq!(
        client
            .webhook_get("a+tag@example.com")
            .await
            .unwrap()
            .webhook_url,
        "https://example.com/callback"
    );
    let list = client
        .webhook_list(&WebhookListQuery {
            inicio: "2026-11-01+00:00".into(),
            fim: "2026-11-03".into(),
        })
        .await
        .unwrap();
    assert_eq!(list.parametros["inicio"], "2026-11-01");
    assert_eq!(list.webhooks[0].chave.as_deref(), Some("key"));
    client.webhook_delete("+5511").await.unwrap();
    let requests = server.join().unwrap();
    assert_eq!(
        target(&requests[1]),
        "PUT /v2/webhook/a%2Btag%40example.com HTTP/1.1"
    );
    assert_eq!(
        body(&requests[1]),
        json!({"webhookUrl":"https://example.com/callback"})
    );
    assert_eq!(
        target(&requests[2]),
        "GET /v2/webhook/a%2Btag%40example.com HTTP/1.1"
    );
    assert_eq!(
        query(&requests[3]),
        vec![
            ("inicio".into(), "2026-11-01+00:00".into()),
            ("fim".into(), "2026-11-03".into())
        ]
    );
    assert_eq!(target(&requests[4]), "DELETE /v2/webhook/%2B5511 HTTP/1.1");
}

#[tokio::test]
async fn send_reuses_caller_identity_and_rejects_missing_or_mismatched_response_identity() {
    let send = json!({"valor":"10.00","pagador":{"chave":"payer","infoPagador":"service"},"favorecido":{"chave":"recipient"}});
    let payload: PixTransactionPayload = serde_json::from_value(send.clone()).unwrap();
    let (client,server)=fixture(vec![auth(),(201,json!({"idEnvio":"stable/id","e2eId":"e2e","status":"UNKNOWN_PROVIDER_STATE","horario":{"solicitacao":"now"}}).to_string()),(201,json!({"idEnvio":"other","status":"UNKNOWN"}).to_string()),(201,json!({"status":"UNKNOWN"}).to_string()),(200,json!({"idEnvio":"stable/id","endToEndId":"read-e2e","horario":{"solicitacao":"now"},"favorecido":{"chave":"recipient"}}).to_string())]);
    let result = client.pix_send("stable/id", &payload).await.unwrap();
    assert_eq!(result.id_envio, "stable/id");
    assert_eq!(result.e2e_id.as_deref(), Some("e2e"));
    assert_eq!(result.status, "UNKNOWN_PROVIDER_STATE");
    assert!(matches!(
        client.pix_send("stable/id", &payload).await,
        Err(Error::Json(_))
    ));
    assert!(matches!(
        client.pix_send("stable/id", &payload).await,
        Err(Error::Json(_))
    ));
    let detail = client.pix_get_transaction("read/e2e").await.unwrap();
    assert_eq!(detail.end_to_end_id.as_deref(), Some("read-e2e"));
    assert_eq!(detail.favorecido.unwrap()["chave"], "recipient");
    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 5);
    for request in &requests[1..4] {
        assert_eq!(target(request), "PUT /v3/gn/pix/stable%2Fid HTTP/1.1");
        assert_eq!(body(request), send);
    }
    assert_eq!(
        target(&requests[4]),
        "GET /v2/gn/pix/enviados/read%2Fe2e HTTP/1.1"
    );
}

#[tokio::test]
async fn split_links_unlinks_acknowledge_status_and_reads_preserve_nested_charge_data() {
    let charge = json!({"txid":"charge","config":{"id":"nested"},"split":{"settlement":[{"state":"provider"}]}});
    let (client, server) = fixture(vec![
        auth(),
        (204, String::new()),
        (201, "unspecified bytes".into()),
        (200, String::new()),
        (204, String::new()),
        (200, charge.to_string()),
        (200, charge.to_string()),
    ]);
    client.split_link_cob("tx/id", "%2F/..?x").await.unwrap();
    client.split_link_cobv("tx", "config").await.unwrap();
    client.split_unlink_cob("tx").await.unwrap();
    client.split_unlink_cobv("tx").await.unwrap();
    assert_eq!(
        serde_json::to_value(client.split_get_cob("tx").await.unwrap()).unwrap(),
        charge
    );
    assert_eq!(
        serde_json::to_value(client.split_get_cobv("tx").await.unwrap()).unwrap(),
        charge
    );
    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 7);
    assert_eq!(
        requests[1..].iter().map(|r| target(r)).collect::<Vec<_>>(),
        [
            "PUT /v2/gn/split/cob/tx%2Fid/vinculo/%252F%2F..%3Fx HTTP/1.1",
            "PUT /v2/gn/split/cobv/tx/vinculo/config HTTP/1.1",
            "DELETE /v2/gn/split/cob/tx/vinculo HTTP/1.1",
            "DELETE /v2/gn/split/cobv/tx/vinculo HTTP/1.1",
            "GET /v2/gn/split/cob/tx HTTP/1.1",
            "GET /v2/gn/split/cobv/tx HTTP/1.1"
        ]
    );
    let config = SplitConfig {
        divisao_tarifa: "assumir_total".into(),
        minha_parte: SplitParte {
            tipo: "porcentagem".into(),
            valor: "100".into(),
        },
        repasses: vec![],
    };
    assert_eq!(
        serde_json::to_value(config).unwrap(),
        json!({"divisaoTarifa":"assumir_total","minhaParte":{"tipo":"porcentagem","valor":"100"},"repasses":[]})
    );
}

#[tokio::test]
async fn invalid_identifiers_fail_before_authentication_or_api_dispatch() {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let client = Client::test_billing_client(&format!("http://{}", listener.local_addr().unwrap()));
    for id in ["", ".", ".."] {
        assert!(matches!(
            client.billing_notification_get(id).await,
            Err(Error::InvalidPath)
        ));
        assert!(matches!(
            client
                .webhook_configure(
                    id,
                    &WebhookPayload {
                        webhook_url: "https://example.com".into()
                    }
                )
                .await,
            Err(Error::InvalidPath)
        ));
        assert!(matches!(
            client.split_link_cob(id, "config").await,
            Err(Error::InvalidPath)
        ));
        assert!(matches!(
            client.split_link_cob("tx", id).await,
            Err(Error::InvalidPath)
        ));
    }
    assert!(matches!(listener.accept(),Err(error) if error.kind()==std::io::ErrorKind::WouldBlock));
}
