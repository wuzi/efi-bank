//! One bounded reader for OAuth and API responses. Limits are SDK policy, not provider maxima.
use crate::Error;

/// Error prefixes always fit in `limit`, including replacement UTF-8 and the marker.
/// Very small limits use `!` as the truncation marker.
fn diagnostic(bytes: &[u8], limit: usize, truncated: bool) -> String {
    let text = String::from_utf8_lossy(bytes);
    let truncated = truncated || text.len() > limit;
    let marker = if limit >= "[truncated]".len() {
        "[truncated]"
    } else {
        "!"
    };
    let available = limit.saturating_sub(if truncated { marker.len() } else { 0 });
    let mut end = text.len().min(available);
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    let mut result = text[..end].to_owned();
    if truncated {
        result.push_str(marker);
    }
    result
}

pub(crate) async fn read_response(
    mut response: reqwest::Response,
    limit: usize,
) -> Result<Vec<u8>, Error> {
    let status = response.status();
    let success = status.is_success();
    let retry_after = response
        .headers()
        .get(reqwest::header::RETRY_AFTER)
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned);
    let declared_oversize = response
        .content_length()
        .is_some_and(|length| length > limit as u64);
    if success && declared_oversize {
        return Err(Error::ResponseTooLarge { limit });
    }
    let mut bytes = Vec::new();
    let mut truncated = declared_oversize;
    loop {
        let chunk = match response.chunk().await {
            Ok(Some(chunk)) => chunk,
            Ok(None) => break,
            Err(error) if success => return Err(error.into()),
            Err(_) => {
                truncated = true;
                break;
            }
        };
        let available = limit.saturating_sub(bytes.len());
        if chunk.len() > available {
            if success {
                return Err(Error::ResponseTooLarge { limit });
            }
            bytes.extend_from_slice(&chunk[..available]);
            truncated = true;
            break;
        }
        bytes.extend_from_slice(&chunk);
    }
    if !success {
        return Err(Error::RequestFailed {
            status,
            retry_after,
            body: diagnostic(&bytes, limit, truncated),
        });
    }
    Ok(bytes)
}
