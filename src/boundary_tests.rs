//! Synthesized local fixtures; these are not captured provider responses.
use std::io::Write;
use std::net::TcpListener;
use std::thread;

use crate::test_support::{
    fixture, fixture_auth as auth, fixture_with_builder, read_request, read_request_async, respond,
};
use crate::{Client, Error};
use reqwest::Method;

#[tokio::test]
async fn domain_tokens_are_isolated_and_reused() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let mut client =
        Client::test_billing_client(&format!("http://{}", listener.local_addr().unwrap()));
    client.test_distinct_oauth();
    let server = thread::spawn(move || {
        let mut requests = Vec::new();
        for body in [
            r#"{"access_token":"pix-fixture","expires_in":3600}"#,
            r#"{"access_token":"billing-fixture","expires_in":3600}"#,
            "{}",
            "{}",
        ] {
            let (mut stream, _) = listener.accept().unwrap();
            requests.push(read_request(&mut stream));
            respond(&mut stream, 200, body);
        }
        requests
    });
    client.authenticate().await.unwrap();
    client.authenticate_billing().await.unwrap();
    client
        .send_authenticated::<(), serde_json::Value>(Method::GET, "/pix", None)
        .await
        .unwrap();
    client
        .send_authenticated_billing::<(), serde_json::Value>(Method::GET, "/billing", None)
        .await
        .unwrap();
    let requests = server.join().unwrap();
    assert!(requests[0].starts_with("POST /oauth/token "));
    assert!(requests[1].starts_with("POST /v1/authorize "));
    let pix_bearer = "authorization: bearer pix-fixture\r\n";
    assert!(requests[2].to_ascii_lowercase().contains(pix_bearer));
    let billing_bearer = "authorization: bearer billing-fixture\r\n";
    assert!(requests[3].to_ascii_lowercase().contains(billing_bearer));
}

#[tokio::test]
async fn oauth_default_rejects_oversized_valid_response() {
    let body =
        serde_json::json!({"access_token":"x".repeat(1024 * 1024),"expires_in":3600}).to_string();
    let (client, server) = fixture(vec![(200, body)]);
    let result = client.authenticate().await;
    tokio::task::spawn_blocking(move || server.join().unwrap())
        .await
        .unwrap();
    assert!(matches!(result, Err(Error::ResponseTooLarge { limit }) if limit == 1024 * 1024));
    assert!(client.pix_token.cached.lock().unwrap().is_none());
}

#[tokio::test]
async fn billing_notification_identity_remains_one_literal_segment() {
    let response = r#"{"code":200,"data":[]}"#.to_owned();
    let (client, server) = fixture(vec![
        auth(),
        (200, response.clone()),
        (200, response.clone()),
        (200, response),
    ]);
    for id in ["%2F/a?b#c+@é", "%2e%2e", "..."] {
        client.billing_notification_get(id).await.unwrap();
    }
    let requests = server.join().unwrap();
    assert_eq!(requests.len(), 4);
    assert_eq!(
        requests[1..]
            .iter()
            .map(|r| r.lines().next().unwrap())
            .collect::<Vec<_>>(),
        [
            "GET /v1/notification/%252F%2Fa%3Fb%23c%2B%40%C3%A9 HTTP/1.1",
            "GET /v1/notification/%252e%252e HTTP/1.1",
            "GET /v1/notification/... HTTP/1.1",
        ]
    );
}

#[tokio::test]
async fn webhook_delete_acknowledges_empty_success() {
    let (client, server) = fixture(vec![auth(), (204, String::new())]);
    let result = client.webhook_delete("fixture-key").await;
    server.join().unwrap();
    assert!(
        result.is_ok(),
        "empty documented acknowledgement: {result:?}"
    );
}

#[tokio::test]
async fn carnet_metadata_acknowledges_action_without_synthetic_data() {
    let (client, server) = fixture(vec![auth(), (200, r#"{"code":200}"#.into())]);
    let result = client
        .carnet_update_metadata(
            1,
            &crate::CarnetMetadataRequest {
                notification_url: None,
                custom_id: Some("invoice".into()),
            },
        )
        .await;
    server.join().unwrap();
    assert!(
        result.is_ok(),
        "action envelope has no carnet data: {result:?}"
    );
}

#[test]
fn cob_read_retains_documented_calendar_and_copy_paste() {
    let response: crate::CobResponse = serde_json::from_value(serde_json::json!({
        "txid":"fixture", "calendario":{"criacao":"2026-11-03T00:00:00Z","expiracao":3600}, "pixCopiaECola":"fixture-code"
    })).unwrap();
    let value = serde_json::to_value(response).unwrap();
    assert_eq!(value["calendario"]["criacao"], "2026-11-03T00:00:00Z");
    assert_eq!(value["calendario"]["expiracao"], 3600);
    assert_eq!(value["pixCopiaECola"], "fixture-code");
}

#[tokio::test]
async fn shared_bounds_cover_stream_status_errors_and_builder_validation() {
    assert!(
        crate::ClientBuilder::new()
            .credentials("id", "secret")
            .api_response_limit(0)
            .build()
            .is_err()
    );
    assert!(
        crate::ClientBuilder::new()
            .credentials("id", "secret")
            .oauth_response_limit(0)
            .build()
            .is_err()
    );
    // Exact-cap JSON succeeds; JSON and status-only modes both reject larger bodies.
    let (client, server) = fixture_with_builder(
        vec![
            auth(),
            (200, "{}".into()),
            (201, "unspecified acknowledgement".into()),
        ],
        crate::ClientBuilder::new()
            .credentials("fixture", "fixture")
            .api_response_limit(2),
    );
    client
        .send_authenticated::<(), serde_json::Value>(Method::GET, "/exact", None)
        .await
        .unwrap();
    assert!(matches!(
        client
            .send_authenticated_status::<()>(Method::PUT, "/status", None)
            .await,
        Err(Error::ResponseTooLarge { limit: 2 })
    ));
    assert_eq!(server.join().unwrap().len(), 3);

    // No Content-Length: streaming size, not a header, enforces the same boundary.
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let client = Client::test_client_from_builder(
        &format!("http://{}", listener.local_addr().unwrap()),
        crate::ClientBuilder::new()
            .credentials("fixture", "fixture")
            .api_response_limit(4),
    );
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        read_request(&mut stream);
        respond(&mut stream, 200, &auth().1);
        let (mut stream, _) = listener.accept().unwrap();
        read_request(&mut stream);
        stream.write_all(b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n3\r\n{\"x\r\n4\r\n\":0}\r\n0\r\n\r\n").unwrap();
    });
    assert!(matches!(
        client
            .send_authenticated::<(), serde_json::Value>(Method::PATCH, "/mutation", None)
            .await,
        Err(Error::ResponseTooLarge { limit: 4 })
    ));
    server.join().unwrap();

    // Both OAuth and API errors preserve metadata with marked, bounded diagnostics.
    for oauth in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let client = Client::test_client_from_builder(
            &format!("http://{}", listener.local_addr().unwrap()),
            crate::ClientBuilder::new()
                .credentials("fixture", "fixture")
                .api_response_limit(16)
                .oauth_response_limit(if oauth { 16 } else { 1024 }),
        );
        let server = thread::spawn(move || {
            if !oauth {
                let (mut stream, _) = listener.accept().unwrap();
                read_request(&mut stream);
                respond(&mut stream, 200, &auth().1);
            }
            let (mut stream, _) = listener.accept().unwrap();
            read_request(&mut stream);
            let body = [0xff; 100];
            stream.write_all(b"HTTP/1.1 429 fixture\r\nRetry-After: 42\r\nContent-Length: 100\r\nConnection: close\r\n\r\n").unwrap();
            let _ = stream.write_all(&body);
        });
        let error = if oauth {
            client.authenticate().await.unwrap_err()
        } else {
            client
                .send_authenticated_status::<()>(Method::DELETE, "/mutation", None)
                .await
                .unwrap_err()
        };
        assert_eq!(
            error.status_code(),
            Some(reqwest::StatusCode::TOO_MANY_REQUESTS)
        );
        assert_eq!(error.retry_after(), Some("42"));
        let body = error.response_body().unwrap();
        assert!(body.len() <= 16);
        assert!(body.ends_with("[truncated]"));
        server.join().unwrap();
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn shared_cold_expiry_singleflight_and_domain_independence() {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use tokio::io::AsyncWriteExt;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let mut client =
        Client::test_billing_client(&format!("http://{}", listener.local_addr().unwrap()));
    client.test_distinct_oauth();
    let client = Arc::new(client);
    let pix_auths = Arc::new(AtomicUsize::new(0));
    let billing_auths = Arc::new(AtomicUsize::new(0));
    let (started, mut auth_started) = tokio::sync::mpsc::unbounded_channel();
    let release = Arc::new(tokio::sync::Semaphore::new(0));
    let server_release = release.clone();
    let p = pix_auths.clone();
    let b = billing_auths.clone();
    let server = tokio::spawn(async move {
        let mut tasks = tokio::task::JoinSet::new();
        for _ in 0..29 {
            let (mut stream, _) = listener.accept().await.unwrap();
            let p = p.clone();
            let b = b.clone();
            let release = server_release.clone();
            let started = started.clone();
            tasks.spawn(async move {
                let request = read_request_async(&mut stream).await;
                let body = if request.starts_with("POST /oauth/token ") {
                    p.fetch_add(1, Ordering::SeqCst);
                    started.send(()).unwrap();
                    let _permit = release.acquire().await.unwrap();
                    r#"{"access_token":"pix-local","expires_in":3600}"#
                } else if request.starts_with("POST /v1/authorize ") {
                    b.fetch_add(1, Ordering::SeqCst);
                    r#"{"access_token":"billing-local","expires_in":3600}"#
                } else {
                    let expected = if request.starts_with("GET /billing ") {
                        "authorization: bearer billing-local\r\n"
                    } else {
                        "authorization: bearer pix-local\r\n"
                    };
                    assert!(request.to_ascii_lowercase().contains(expected));
                    "{}"
                };
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                stream.write_all(response.as_bytes()).await.unwrap();
            });
        }
        while let Some(result) = tasks.join_next().await {
            result.unwrap();
        }
    });
    for wave in 0..2 {
        let barrier = Arc::new(tokio::sync::Barrier::new(13));
        let mut tasks = tokio::task::JoinSet::new();
        for _ in 0..12 {
            let client = client.clone();
            let barrier = barrier.clone();
            tasks.spawn(async move {
                barrier.wait().await;
                client
                    .send_authenticated::<(), serde_json::Value>(Method::GET, "/pix", None)
                    .await
                    .unwrap()
            });
        }
        barrier.wait().await;
        if wave == 0 {
            auth_started.recv().await.unwrap();
            // Billing must complete while the Pix exchange is deliberately held.
            tokio::time::timeout(
                std::time::Duration::from_secs(5),
                client.send_authenticated_billing::<(), serde_json::Value>(
                    Method::GET,
                    "/billing",
                    None,
                ),
            )
            .await
            .unwrap()
            .unwrap();
            release.add_permits(100);
        }
        while let Some(result) = tasks.join_next().await {
            result.unwrap();
        }
        assert_eq!(pix_auths.load(Ordering::SeqCst), wave + 1);
        client
            .pix_token
            .cached
            .lock()
            .unwrap()
            .as_mut()
            .unwrap()
            .expires_at = std::time::Instant::now();
    }
    client
        .send_authenticated_billing::<(), serde_json::Value>(Method::GET, "/billing", None)
        .await
        .unwrap();
    assert_eq!(billing_auths.load(Ordering::SeqCst), 1);
    server.await.unwrap();
}

#[test]
fn shared_path_builder_preserves_opaque_identities_and_rejects_local_dots() {
    use crate::client::resource_path;
    for invalid in ["", ".", ".."] {
        assert!(matches!(
            resource_path(&["v2", invalid]),
            Err(Error::InvalidPath)
        ));
    }
    assert_eq!(
        resource_path(&["v2", "%2F/%2e%2e?#+@é"]).unwrap(),
        "/v2/%252F%2F%252e%252e%3F%23%2B%40%C3%A9"
    );
    assert_eq!(resource_path(&["v2", "..."]).unwrap(), "/v2/...");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn shared_delayed_old_401_reuses_new_token_without_third_exchange() {
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    use tokio::io::AsyncWriteExt;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = Arc::new(Client::test_billing_client(&format!(
        "http://{}",
        listener.local_addr().unwrap()
    )));
    let auths = Arc::new(AtomicUsize::new(0));
    let server_auths = auths.clone();
    let server = tokio::spawn(async move {
        let old_reads = Arc::new(tokio::sync::Barrier::new(2));
        let newer_used = Arc::new(tokio::sync::Semaphore::new(0));
        let mut tasks = tokio::task::JoinSet::new();
        for _ in 0..6 {
            let (mut stream, _) = listener.accept().await.unwrap();
            let auths = server_auths.clone();
            let old_reads = old_reads.clone();
            let newer_used = newer_used.clone();
            tasks.spawn(async move {
                let request = read_request_async(&mut stream).await.to_ascii_lowercase();
                let (status, body) = if request.starts_with("post /v1/authorize ") {
                    let index = auths.fetch_add(1, Ordering::SeqCst) + 1;
                    (200, format!(r#"{{"access_token":"local-{index}","expires_in":3600}}"#))
                } else if request.contains("authorization: bearer local-1\r\n") {
                    old_reads.wait().await;
                    if request.starts_with("get /two ") { let _permit = newer_used.acquire().await.unwrap(); }
                    (401, "{}".into())
                } else {
                    assert!(request.contains("authorization: bearer local-2\r\n"));
                    if request.starts_with("get /one ") { newer_used.add_permits(1); }
                    (200, "{}".into())
                };
                stream.write_all(format!("HTTP/1.1 {status} fixture\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).as_bytes()).await.unwrap();
            });
        }
        while let Some(result) = tasks.join_next().await {
            result.unwrap();
        }
    });
    client.authenticate_billing().await.unwrap();
    let (one, two) = tokio::join!(
        client.send_authenticated_billing::<(), serde_json::Value>(Method::GET, "/one", None),
        client.send_authenticated_billing::<(), serde_json::Value>(Method::GET, "/two", None)
    );
    one.unwrap();
    two.unwrap();
    server.await.unwrap();
    assert_eq!(auths.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn shared_declared_default_oversize_rejects_before_waiting_for_body_bytes() {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let client = Client::test_billing_client(&format!("http://{}", listener.local_addr().unwrap()));
    let (release, held) = std::sync::mpsc::channel();
    let server = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        read_request(&mut stream);
        respond(&mut stream, 200, &auth().1);
        let (mut stream, _) = listener.accept().unwrap();
        read_request(&mut stream);
        stream
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 8388609\r\nConnection: close\r\n\r\n")
            .unwrap();
        // No body is sent until the caller has already reported the header-bound failure.
        held.recv_timeout(std::time::Duration::from_secs(5))
            .unwrap();
    });
    let result = client
        .send_authenticated::<(), serde_json::Value>(Method::PUT, "/mutation", None)
        .await;
    release.send(()).unwrap();
    server.join().unwrap();
    assert!(matches!(
        result,
        Err(Error::ResponseTooLarge { limit: 8_388_608 })
    ));
}

#[tokio::test]
async fn shared_second_401_is_returned_without_another_replay() {
    for billing in [false, true] {
        let (client, server) =
            fixture(vec![auth(), (401, "{}".into()), auth(), (401, "{}".into())]);
        let result = if billing {
            client
                .send_authenticated_billing::<(), serde_json::Value>(Method::GET, "/read", None)
                .await
        } else {
            client
                .send_authenticated::<(), serde_json::Value>(Method::GET, "/read", None)
                .await
        };
        assert_eq!(
            result.unwrap_err().status_code(),
            Some(reqwest::StatusCode::UNAUTHORIZED)
        );
        assert_eq!(server.join().unwrap().len(), 4);
    }
}

#[tokio::test]
async fn explicit_authentication_exchanges_even_with_a_fresh_cached_token() {
    use tokio::io::AsyncWriteExt;
    for billing in [false, true] {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let client =
            Client::test_billing_client(&format!("http://{}", listener.local_addr().unwrap()));
        let (done, mut stopped) = tokio::sync::oneshot::channel::<()>();
        let server = tokio::spawn(async move {
            let mut requests = Vec::new();
            loop {
                let (mut stream, _) = tokio::select! {
                    _ = &mut stopped => break,
                    incoming = listener.accept() => incoming.unwrap(),
                };
                let request = read_request_async(&mut stream).await;
                requests.push(request.lines().next().unwrap().to_owned());
                let body = auth().1;
                stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",body.len()).as_bytes()).await.unwrap();
            }
            requests
        });
        for _ in 0..2 {
            if billing {
                client.authenticate_billing().await.unwrap();
            } else {
                client.authenticate().await.unwrap();
            }
        }
        done.send(()).unwrap();
        let requests = server.await.unwrap();
        assert_eq!(
            requests,
            ["POST /v1/authorize HTTP/1.1", "POST /v1/authorize HTTP/1.1"]
        );
    }
}
