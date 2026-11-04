//! Shared loopback support for synthesized fixtures; never live provider traffic.

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread::{self, JoinHandle};

use crate::{Client, ClientBuilder};

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
        Self::start_with_headers(exchanges, &[])
    }

    pub fn start_with_headers(
        exchanges: Vec<Exchange>,
        response_headers: &'static [(&'static str, &'static str)],
    ) -> Self {
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
                stream
                    .set_write_timeout(Some(std::time::Duration::from_secs(5)))
                    .unwrap();
                write!(
                    stream,
                    "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n",
                    exchange.status,
                    reason,
                    exchange.body.len(),
                )
                .unwrap();
                for (name, value) in response_headers {
                    write!(stream, "{name}: {value}\r\n").unwrap();
                }
                write!(stream, "Connection: close\r\n\r\n{}", exchange.body).unwrap();
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

pub fn auth() -> Exchange {
    Exchange {
        request_line: "POST /v1/authorize HTTP/1.1",
        status: 200,
        body: r#"{"access_token":"fixture-token","expires_in":3600}"#,
    }
}
pub(crate) fn read_request(stream: &mut TcpStream) -> String {
    stream
        .set_read_timeout(Some(std::time::Duration::from_secs(5)))
        .unwrap();
    let mut bytes = Vec::new();
    loop {
        let mut buffer = [0; 4096];
        let count = stream.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..count]);
        if request_complete(&bytes) {
            break;
        }
    }
    String::from_utf8(bytes).unwrap()
}

fn request_complete(bytes: &[u8]) -> bool {
    let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") else {
        return false;
    };
    let headers = String::from_utf8_lossy(&bytes[..end]);
    let length = headers
        .lines()
        .find_map(|line| {
            line.to_ascii_lowercase()
                .strip_prefix("content-length: ")
                .and_then(|s| s.parse::<usize>().ok())
        })
        .unwrap_or(0);
    bytes.len() >= end + 4 + length
}

pub(crate) async fn read_request_async(stream: &mut tokio::net::TcpStream) -> String {
    use tokio::io::AsyncReadExt;
    let mut bytes = Vec::new();
    loop {
        let mut buffer = [0; 4096];
        let size = stream.read(&mut buffer).await.unwrap();
        if size == 0 {
            break;
        }
        bytes.extend_from_slice(&buffer[..size]);
        if request_complete(&bytes) {
            break;
        }
    }
    String::from_utf8(bytes).unwrap()
}

pub(crate) fn respond(stream: &mut TcpStream, status: u16, body: &str) {
    stream
        .set_write_timeout(Some(std::time::Duration::from_secs(5)))
        .unwrap();
    let _ = write!(
        stream,
        "HTTP/1.1 {status} fixture\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );
}

pub(crate) fn fixture(exchanges: Vec<(u16, String)>) -> (Client, thread::JoinHandle<Vec<String>>) {
    fixture_with_builder(
        exchanges,
        ClientBuilder::new().credentials("fixture-client", "fixture-secret"),
    )
}

pub(crate) fn fixture_with_builder(
    exchanges: Vec<(u16, String)>,
    builder: ClientBuilder,
) -> (Client, thread::JoinHandle<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let client = Client::test_client_from_builder(
        &format!("http://{}", listener.local_addr().unwrap()),
        builder,
    );
    let handle = thread::spawn(move || {
        exchanges
            .into_iter()
            .map(|(status, body)| {
                let (mut stream, _) = listener.accept().unwrap();
                let request = read_request(&mut stream);
                respond(&mut stream, status, &body);
                request
            })
            .collect()
    });
    (client, handle)
}

pub(crate) fn fixture_auth() -> (u16, String) {
    (
        200,
        r#"{"access_token":"fixture-token","expires_in":3600}"#.into(),
    )
}
