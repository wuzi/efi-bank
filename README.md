# efi-bank

A Rust SDK for Efí Billing and Pix, with OAuth token coordination and optional PKCS#12 mTLS. Public contracts follow the [Efí documentation](https://dev.efipay.com.br/). HTTP acknowledgement and provider status strings do not establish payment settlement.

Add the library with `cargo add efi-bank@0.3.0` or `efi-bank = "0.3.0"` under `[dependencies]`. The async HTTP client uses Tokio; configure your application's runtime normally.

```rust,no_run
use efi_bank::{ClientBuilder, Environment};
# fn main() -> Result<(), efi_bank::Error> {
let client = ClientBuilder::new()
    .credentials("client-id", "client-secret")
    .environment(Environment::Sandbox)
    .pkcs12_file("/path/to/certificate.p12", "certificate-password")
    .build()?;
# let _ = client;
# Ok(()) }
```

## Transport and authentication bounds

SDK transports disable retries and redirects. Each native HTTP attempt has a 20-second total deadline and 5-second connect deadline, including TLS negotiation. Each authorization domain has its own token cache and async refresh gate; successful cold/expired acquisition is shared by concurrent callers. The freshness skew is 30 seconds. Explicit `authenticate()` and `authenticate_billing()` preserve their forced OAuth exchange semantics; automatic operation authentication reuses a fresh cached token. Waiting for a domain refresh gate is limited to 20 seconds; a queue timeout returns `AuthUnavailable` before this caller dispatches OAuth or API work. With SDK transports, cold token acquisition therefore allows up to 20 seconds of queue wait plus up to 20 seconds for its native OAuth attempt, not a whole-operation 20-second deadline. Caller-owned transports retain caller-owned attempt deadlines. Pix refresh does not block Billing refresh. A delayed 401 rejects only the matching domain/token, preserving any newer token.

Only GET may retry once after 401. A cold GET therefore makes at most four native requests: OAuth, GET, OAuth, GET. POST/PUT/PATCH/DELETE are never automatically replayed after 401, timeout, dropped response, rate limit, 5xx, oversized/invalid response, or identity mismatch. Reconcile uncertain mutations before deciding on another caller-initiated operation.

`ClientBuilder::http_client` replaces the SDK transport. Its caller must disable retries/redirects and set finite total/connect deadlines to retain native attempt bounds. mTLS builder options and a caller-owned HTTP client are mutually exclusive.

API JSON, status-only acknowledgement bodies, and provider diagnostics default to 8 MiB; OAuth success/error bodies default to 1 MiB. These are engineering limits, not provider maxima. Configure nonzero byte bounds with `api_response_limit(bytes)` and `oauth_response_limit(bytes)`, including larger limits for unusually large lists. Declared and streamed sizes are checked. Oversized successes return `Error::ResponseTooLarge { limit }` without decoding truncated JSON. Non-2xx responses retain their status, raw Retry-After, and a bounded UTF-8 diagnostic prefix marked `[truncated]` when incomplete/oversized (tiny caps use `!`). Status-only operations consume/discard bounded success bytes without requiring JSON. Successful JSON operations still reject empty bodies.

String resource identifiers are encoded as single opaque path segments. Empty IDs and literal `.`/`..` return `Error::InvalidPath` before authentication or dispatch. Encoded-looking strings such as `%2F` remain literal identities; the SDK does not decode or guess a provider ID alphabet.

PDF downloads retain their separate 10 MiB limit, exact trusted HTTPS host, no credentials/redirects, and PDF content/signature/EOF checks.

## Billing and carnets

Existing Billing APIs and DTOs are preserved. `billing_charge_update_billet(id, &BillingBilletUpdateRequest { expire_at })` uses PUT `/v1/charge/{id}/billet`, returning `BillingActionResponse`. The provider requires a YYYY-MM-DD due date after today; this SDK does not introduce a money/date validation framework. Read details separately when needed.

Billing charge discovery uses `BillingChargeListQuery`; `billing_charges_list_with_date_of(&query, Some(BillingChargeDateOf::Creation))` selects creation dates. The original list method omits the date selector; Payment/Expired are also supported. Lists return one page and retain metadata.

Carnet metadata/parcel updates, cancellations, resends, history, and settlement actions return `BillingActionResponse { code }`. They do not issue an implicit read or fabricate carnet details. Both resend methods require `CarnetResendRequest { email }`. `carnet_list(begin_date,end_date)` returns child charges in `BillingChargeListResponse`, selecting `charge_type=carnet`. Detail/create responses still require real data. Deprecated `carnet_cancel_response` and `carnet_cancel_parcel_response` forward to the corrected canonical cancellation methods during 0.3.x.

## Immediate and due-date Pix charges

```rust,no_run
use efi_bank::{CobPayload, CobCalendario, CobValor, PixChargeListQuery};
# async fn example(client: &efi_bank::Client) -> Result<(), efi_bank::Error> {
let payload = CobPayload {
    calendario: CobCalendario { expiracao: Some(3600) },
    devedor: None,
    valor: CobValor { original: "100.50".into(), multa: None, juros: None, desconto: None, abatimento: None },
    chave: "payer@example.com".into(),
    solicitacao_pagador: None,
    info_adicionais: None,
};
let charge = client.cob_create(&payload).await?;
if let Some(code) = charge.pix_copia_e_cola { println!("{code}"); }
let page = client.cob_list(&PixChargeListQuery {
    inicio: "2026-11-01T00:00:00Z".into(), fim: "2026-11-03T23:59:59Z".into(),
    cpf: None, status: Some("ATIVA".into()),
}).await?;
for charge in page.cobs { println!("{}", charge.txid); }
# Ok(()) }
```

`cob_create` uses POST `/v2/cob`; `cob_update(txid,...)` uses PUT and means create with caller txid. Revision uses `cob_patch`. Omitted expiry does not invent a TTL. Response calendar exposes optional `criacao` and opaque numeric/string `expiracao` because documented examples disagree. Copy-and-paste uses `pix_copia_e_cola`/`pixCopiaECola`.

```rust,no_run
use efi_bank::{CobvPayload, CobvCalendario, CobvValor};
use serde_json::json;
# async fn example(client: &efi_bank::Client) -> Result<(), efi_bank::Error> {
let payload = CobvPayload {
    calendario: CobvCalendario { data_de_vencimento: "2026-12-01".into(), validade_apos_vencimento: 5 },
    devedor: None,
    valor: CobvValor {
        original: "500.00".into(),
        multa: Some(serde_json::from_value(json!({"modalidade":2,"valorPerc":"2.00"}))?),
        juros: None, desconto: None, abatimento: None,
    },
    chave: "payer@example.com".into(), solicitacao_pagador: None, info_adicionais: None,
};
let charge = client.cobv_create("persisted-txid", &payload).await?;
let _ = client.cobv_patch(&charge.txid, &payload).await?;
# Ok(()) }
```

Cobv creation requires the caller txid and PUT `/v2/cobv/{txid}`; `cobv_update` has the same create-with-txid semantics. `CobvValor` uses JSON objects for multa/juros/desconto, and opaque JSON for abatimento without claiming a verified schema. Immediate `CobValor` remains unchanged; no additional immediate adjustment support is promised. Both Pix list methods require `PixChargeListQuery { inicio,fim,cpf,status }`, return `{parametros,cobs}`, and fetch only one page without an invented limit parameter.

## Pix webhooks

```rust,no_run
use efi_bank::{WebhookPayload, WebhookListQuery};
# async fn example(client: &efi_bank::Client) -> Result<(), efi_bank::Error> {
let key = "payer@example.com";
client.webhook_configure(key, &WebhookPayload { webhook_url: "https://example.com/pix".into() }).await?;
let webhook = client.webhook_get(key).await?;
println!("{}", webhook.webhook_url);
let page = client.webhook_list(&WebhookListQuery {
    inicio: "2026-11-01T00:00:00Z".into(), fim: "2026-11-03T23:59:59Z".into(),
}).await?;
let _ = page.parametros;
client.webhook_delete(key).await?;
# Ok(()) }
```

Configuration is PUT `/v2/webhook/{chave}` with only `{webhookUrl}`; the key is a path identity. Configure/delete return `()` on bounded 2xx acknowledgement, including empty 204. GET returns `webhook_url`, optional key/creation, without a fabricated id. List retains parametros and requires both dates. Configure's success body has no asserted schema.

## Send and track Pix

```rust,no_run
use efi_bank::{PixTransactionPayload, PixSender, PixRecipient};
# async fn example(client: &efi_bank::Client) -> Result<(), efi_bank::Error> {
// Persist this identity with the intent; reuse it only for that same intent.
let id_envio = "persisted-operation-id";
let result = client.pix_send(id_envio, &PixTransactionPayload {
    valor: "100.00".into(),
    pagador: PixSender { chave: "payer@example.com".into(), info_pagador: Some("services".into()) },
    favorecido: PixRecipient { chave: "recipient@example.com".into() },
}).await?;
println!("{}: {}", result.id_envio, result.status);
if let Some(e2e_id) = result.e2e_id {
    let sent = client.pix_get_transaction(&e2e_id).await?;
    println!("{:?}", sent.status);
}
# Ok(()) }
```

Send is PUT `/v3/gn/pix/{idEnvio}` with nested payer/recipient key data. There is no generated identity, body idempotencyKey, bank-account recipient, received-Pix lookup, or mutation retry. Missing/mismatched response idEnvio returns an uncertain JSON/protocol error. Unknown status strings remain intact; acknowledgement does not prove settlement. Send exposes `e2eId`; sent lookup at GET `/v2/gn/pix/enviados/{e2eId}` exposes the distinct `endToEndId` wire field and optional ancillary JSON objects.

## Split

Configuration DTOs retain `divisaoTarifa` and `minhaParte`. Config GET remains exposed, but the provider reference carries a maintenance notice; the SDK does not establish current availability. Link/unlink Cob/Cobv methods return `()` on bounded 2xx acknowledgement. Cob linkage documents 204; Cobv examples conflict among 200/201/204, so no body/status value is manufactured. Charge GET methods return `SplitChargeResponse` (a JSON object) retaining nested config/settlement fields without lifting config.id.

```rust,no_run
# async fn example(client: &efi_bank::Client) -> Result<(), efi_bank::Error> {
client.split_link_cob("txid", "split-config-id").await?;
let charge = client.split_get_cob("txid").await?;
println!("{:?}", charge.get("config"));
client.split_unlink_cob("txid").await?;
# Ok(()) }
```

## Error handling and 0.2.x migration

`Error` is non-exhaustive: external matches need a wildcard. `InvalidPath` is local pre-dispatch rejection; `ResponseTooLarge` after a mutation is uncertain. Existing provider status/body/raw Retry-After accessors remain. Treat raw provider diagnostics according to your application's disclosure policy.

```rust,no_run
use efi_bank::Error;
# fn handle(error: Error) {
match error {
    Error::InvalidPath => eprintln!("invalid operation identifier"),
    Error::ResponseTooLarge { limit } => eprintln!("response exceeded {limit} bytes"),
    other => eprintln!("request error (HTTP status {:?})", other.status_code()),
}
# }
```

A 0.2.3 requirement does not select 0.3.0. Upgrade deliberately and review these source breaks:

|0.2.x caller|0.3.0 migration|
|---|---|
|Carnet action returns/detail assumptions|Consume BillingActionResponse; read details explicitly|
|Carnet resend without payload|Supply CarnetResendRequest{email}|
|Carnet list whole-carnet expectation|Consume BillingChargeListResponse child charges/params|
|Immediate due-date calendar fields|Use optional numeric expiracao; no fabricated TTL|
|br or top-level criacao_date_time|Use pix_copia_e_cola and optional nested calendario.criacao|
|Cobv CobValor adjustment strings|Use CobvValor object adjustments/opaque abatimento|
|Cobv creation without txid|Pass caller txid; documented PUT route|
|Pix list filters/limit returning Vec|Pass required dates via PixChargeListQuery; consume parametros/cobs|
|webhook_create/webhook_update, url/chave payload, response id|Use webhook_configure(key,&WebhookPayload{webhook_url}); get/delete by key|
|webhook_list()|Pass WebhookListQuery dates and retain parametros|
|Flat Pix send/body idempotency_key|Supply stable id_envio and nested PixSender/PixRecipient|
|Generic transaction id/origin/destination/date/cancellation fields|Use corrected optional sent lookup fields and exact endToEndId|
|Split link/unlink status reads or SplitLinkResponse|Consume () acknowledgement; read charge separately|
|Split charge interpreted as SplitConfigResponse|Consume nested SplitChargeResponse JSON|
|Exhaustive Error match|Add wildcard and handle local path/body-limit failures deliberately|

Working Billing charge/create/update/read/list/cancel/notification, carnet create/detail, and PDF APIs remain supported. Billing's embedded `pix.qrcode` is separate from these Pix DTO corrections. No automatic pagination, broader PATCH schema, or settlement/reconciliation engine is added.
