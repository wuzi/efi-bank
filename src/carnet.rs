use reqwest::Method;

use crate::client::Client;
use crate::error::Error;
use crate::types::{
    BillingActionResponse, CarnetCreateRequest, CarnetDetailResponse, CarnetHistoryRequest,
    CarnetMetadataRequest, CarnetParcelRequest, CarnetParcelsRequest, CarnetResponse,
};

impl Client {
    /// Download a provider PDF without forwarding API credentials or following redirects.
    pub async fn billing_pdf_download(&self, url: &str) -> Result<Vec<u8>, Error> {
        let url = artifact_url(url)?;
        download_pdf(url.as_str()).await
    }

    pub async fn carnet_create(
        &self,
        payload: &CarnetCreateRequest,
    ) -> Result<CarnetResponse, Error> {
        self.send_authenticated_billing(Method::POST, "/v1/carnet", Some(payload))
            .await
    }

    pub async fn carnet_get(&self, carnet_id: i64) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::GET,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_detail_get(&self, carnet_id: i64) -> Result<CarnetDetailResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}");
        self.send_authenticated_billing::<serde_json::Value, CarnetDetailResponse>(
            Method::GET,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_list(
        &self,
        begin_date: &str,
        end_date: &str,
    ) -> Result<CarnetResponse, Error> {
        let path =
            format!("/v1/charges?begin_date={begin_date}&end_date={end_date}&charge_type=carnet");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::GET,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_update_metadata(
        &self,
        carnet_id: i64,
        payload: &CarnetMetadataRequest,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/metadata");
        self.send_authenticated_billing(Method::PUT, &path, Some(payload))
            .await
    }

    pub async fn carnet_update_parcel(
        &self,
        carnet_id: i64,
        parcel: i32,
        payload: &CarnetParcelRequest,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/parcel/{parcel}");
        self.send_authenticated_billing(Method::PUT, &path, Some(payload))
            .await
    }

    pub async fn carnet_update_parcels(
        &self,
        carnet_id: i64,
        payload: &CarnetParcelsRequest,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/parcels");
        self.send_authenticated_billing(Method::PUT, &path, Some(payload))
            .await
    }

    pub async fn carnet_cancel(&self, carnet_id: i64) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/cancel");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::PUT,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_cancel_response(
        &self,
        carnet_id: i64,
    ) -> Result<BillingActionResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/cancel");
        self.send_authenticated_billing::<serde_json::Value, BillingActionResponse>(
            Method::PUT,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_cancel_parcel(
        &self,
        carnet_id: i64,
        parcel: i32,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/parcel/{parcel}/cancel");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::PUT,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_cancel_parcel_response(
        &self,
        carnet_id: i64,
        parcel: i32,
    ) -> Result<BillingActionResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/parcel/{parcel}/cancel");
        self.send_authenticated_billing::<serde_json::Value, BillingActionResponse>(
            Method::PUT,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_resend(&self, carnet_id: i64) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/resend");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::POST,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_resend_parcel(
        &self,
        carnet_id: i64,
        parcel: i32,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/parcel/{parcel}/resend");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::POST,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_add_history(
        &self,
        carnet_id: i64,
        payload: &CarnetHistoryRequest,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/history");
        self.send_authenticated_billing(Method::POST, &path, Some(payload))
            .await
    }

    pub async fn carnet_settle(&self, carnet_id: i64) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/settle");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::PUT,
            &path,
            None,
        )
        .await
    }

    pub async fn carnet_settle_parcel(
        &self,
        carnet_id: i64,
        parcel: i32,
    ) -> Result<CarnetResponse, Error> {
        let path = format!("/v1/carnet/{carnet_id}/parcel/{parcel}/settle");
        self.send_authenticated_billing::<serde_json::Value, CarnetResponse>(
            Method::PUT,
            &path,
            None,
        )
        .await
    }
}

const MAX_PDF_BYTES: usize = 10 * 1024 * 1024;

fn artifact_url(value: &str) -> Result<reqwest::Url, Error> {
    let url = reqwest::Url::parse(value).map_err(|_| Error::InvalidArtifact("invalid URL"))?;
    // Exact provider download hosts only: no arbitrary hosts, ports, credentials or redirects.
    if url.scheme() != "https"
        || !matches!(url.host_str(), Some("download.gerencianet.com.br"))
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port_or_known_default() != Some(443)
        || url.fragment().is_some()
    {
        return Err(Error::InvalidArtifact("untrusted URL"));
    }
    Ok(url)
}

async fn download_pdf(url: &str) -> Result<Vec<u8>, Error> {
    let http = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(20))
        .connect_timeout(std::time::Duration::from_secs(5))
        .build()?;
    let mut response = http
        .get(url)
        .header(reqwest::header::ACCEPT, "application/pdf")
        .send()
        .await?;
    if response.status() != reqwest::StatusCode::OK {
        return Err(Error::InvalidArtifact("unsuccessful download"));
    }
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next());
    if !content_type.is_some_and(|value| value.trim().eq_ignore_ascii_case("application/pdf")) {
        return Err(Error::InvalidArtifact("unexpected content type"));
    }
    if response
        .content_length()
        .is_some_and(|length| length > MAX_PDF_BYTES as u64)
    {
        return Err(Error::InvalidArtifact("PDF exceeds size limit"));
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        if bytes.len().saturating_add(chunk.len()) > MAX_PDF_BYTES {
            return Err(Error::InvalidArtifact("PDF exceeds size limit"));
        }
        bytes.extend_from_slice(&chunk);
    }
    if !bytes.starts_with(b"%PDF-")
        || !bytes[bytes.len().saturating_sub(1024)..]
            .windows(5)
            .any(|window| window == b"%%EOF")
    {
        return Err(Error::InvalidArtifact("invalid PDF content"));
    }
    Ok(bytes)
}

#[cfg(test)]
mod artifact_tests {
    use super::*;
    use std::{
        io::{Read, Write},
        net::TcpListener,
        thread,
    };

    #[test]
    fn pdf_artifact_url_rejects_untrusted_destinations() {
        assert!(artifact_url("https://download.gerencianet.com.br/example.pdf").is_ok());
        for url in [
            "http://download.gerencianet.com.br/example.pdf",
            "https://127.0.0.1/example.pdf",
            "https://localhost/example.pdf",
            "https://download.gerencianet.com.br.evil.example/example.pdf",
            "https://user:secret@download.gerencianet.com.br/example.pdf",
            "https://download.gerencianet.com.br:8443/example.pdf",
        ] {
            assert!(artifact_url(url).is_err(), "accepted {url}");
        }
    }

    async fn fixture(
        status: &str,
        content_type: &str,
        body: &[u8],
        declared_length: usize,
    ) -> Result<Vec<u8>, Error> {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let response = format!("HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {declared_length}\r\nLocation: http://127.0.0.1:1/secret\r\nConnection: close\r\n\r\n").into_bytes();
        let body = body.to_vec();
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 8192];
            let size = stream.read(&mut request).unwrap();
            let request = String::from_utf8_lossy(&request[..size]).to_lowercase();
            assert!(!request.contains("authorization:"));
            stream.write_all(&response).unwrap();
            let _ = stream.write_all(&body);
        });
        let result = download_pdf(&url).await;
        server.join().unwrap();
        result
    }

    #[tokio::test]
    async fn artifact_download_bounds_stream_without_content_length() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let server = thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 8192];
            assert!(stream.read(&mut request).unwrap() > 0);
            stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Type: application/pdf\r\nConnection: close\r\n\r\n%PDF-1.7\n").unwrap();
            let chunk = [b'x'; 8192];
            for _ in 0..=(MAX_PDF_BYTES / chunk.len()) {
                if stream.write_all(&chunk).is_err() {
                    break;
                }
            }
        });
        assert!(matches!(
            download_pdf(&url).await,
            Err(Error::InvalidArtifact("PDF exceeds size limit"))
        ));
        server.join().unwrap();
    }

    #[tokio::test]
    async fn artifact_download_returns_pdf_bytes_and_rejects_redirects_html_and_oversize() {
        let pdf = b"%PDF-1.7\nfixture\n%%EOF\n";
        assert_eq!(
            fixture("200 OK", "application/pdf", pdf, pdf.len())
                .await
                .unwrap(),
            pdf
        );
        assert!(
            fixture("302 Found", "application/pdf", pdf, pdf.len())
                .await
                .is_err()
        );
        assert!(
            fixture("200 OK", "text/html", pdf, pdf.len())
                .await
                .is_err()
        );
        assert!(
            fixture("200 OK", "application/pdf", b"https://example.com/file", 24)
                .await
                .is_err()
        );
        assert!(
            fixture("200 OK", "application/pdf", pdf, MAX_PDF_BYTES + 1)
                .await
                .is_err()
        );
        assert!(
            fixture("500 Error", "application/pdf", pdf, pdf.len())
                .await
                .is_err()
        );
    }
}
