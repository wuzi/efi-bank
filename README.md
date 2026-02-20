# efi-bank

A Rust SDK for integrating with the payment services provided by [Efí Bank](https://dev.efipay.com.br/en/docs/api-pix/credenciais/). This library provides strongly-typed bindings for the Efí Pix API with automatic OAuth token management and mTLS support.

## Features

- **mTLS support** - PKCS#12 certificate authentication
- **Sandbox & Production environments** - Environment switching
- **Automatic OAuth token management** - Token caching and refresh handled internally

## Installation

```sh
cargo install efi-bank
```

## Quick Start

### Initialize the Client

```rust
use efi_bank::{ClientBuilder, Environment};

// Create a client with credentials and mTLS certificate
let client = ClientBuilder::new()
    .credentials("your-client-id", "your-client-secret")
    .environment(Environment::Sandbox)
    .pkcs12_file("/path/to/certificate.p12", "certificate-password")
    .build()?;
```

Authentication is automatic, the client handles token retrieval and refresh transparently when making a request.

### Split Payment Configuration

Configure how payments should be split among multiple recipients:

```rust
use efi_bank::{
    ClientBuilder, Environment,
    SplitPayload, SplitConfig, SplitParte, SplitRepasse, SplitFavorecido, SplitLancamento
};

let client = ClientBuilder::new()
    .credentials("client-id", "client-secret")
    .environment(Environment::Sandbox)
    .pkcs12_file("/path/to/cert.p12", "password")
    .build()?;

// Create a split configuration
let split_payload = SplitPayload {
    descricao: "Split configuration for multi-vendor payments".to_string(),
    lancamento: SplitLancamento {
        imediato: true,
    },
    split: SplitConfig {
        divisao_tarifa: "assumir_total".to_string(),
        minha_parte: SplitParte {
            tipo: "porcentagem".to_string(),
            valor: "50".to_string(),
        },
        repasses: vec![
            SplitRepasse {
                tipo: "porcentagem".to_string(),
                valor: "50".to_string(),
                favorecido: SplitFavorecido {
                    cpf: "12345678900".to_string(),
                    conta: "0000000".to_string(),
                },
            },
        ],
    },
};

let config_response = client.split_create_config(&split_payload)?;
println!("Split config created: {}", config_response.id);

// Link the split config to a charge
let link_response = client.split_link_cob("txid123", &config_response.id)?;
println!("Link status: {}", link_response.status);
```

### Create Immediate Charges (COB)

Create charges that expire after a short period:

```rust
use efi_bank::{CobPayload, CobCalendario, CobValor, CobPessoa};

let cob_payload = CobPayload {
    calendario: CobCalendario {
        data_de_vencimento: None,
        validade_apos_vencimento: Some(1),
    },
    devedor: Some(CobPessoa {
        cpf: Some("12345678900".to_string()),
        cnpj: None,
        nome: Some("John Doe".to_string()),
    }),
    valor: CobValor {
        original: "100.50".to_string(),
        multa: None,
        juros: None,
        desconto: None,
        abatimento: None,
    },
    chave: "seu-pix-key@example.com".to_string(),
    solicitacao_pagador: Some("Invoice #001".to_string()),
    info_adicionais: None,
};

let cob_response = client.cob_create(&cob_payload)?;
println!("Charge created with txid: {}", cob_response.txid);
println!("QR Code: {}", cob_response.br.unwrap_or_default());

// Retrieve a specific charge
let charge = client.cob_get(&cob_response.txid)?;
println!("Charge status: {}", charge.status.unwrap_or_default());

// List all charges (with optional filters)
let charges = client.cob_list(
    Some("12345678900"),  // CPF filter
    Some("ATIVA"),        // Status filter
    Some(10),             // Limit
)?;
for charge in charges {
    println!("Charge: {} - {}", charge.txid, charge.status.unwrap_or_default());
}
```

### Create Expiring Charges (COBV)

Create charges with specific expiration dates:

```rust
use efi_bank::{CobvPayload, CobvCalendario, CobValor};

let cobv_payload = CobvPayload {
    calendario: CobvCalendario {
        data_de_vencimento: "2024-12-31".to_string(),
        validade_apos_vencimento: 5,
    },
    devedor: Some(CobPessoa {
        cpf: Some("12345678900".to_string()),
        cnpj: None,
        nome: Some("Jane Doe".to_string()),
    }),
    valor: CobValor {
        original: "500.00".to_string(),
        multa: Some("50.00".to_string()),          // Fine after due date
        juros: Some("10.00".to_string()),          // Interest
        desconto: Some("25.00".to_string()),       // Discount if paid early
        abatimento: None,
    },
    chave: "seu-pix-key@example.com".to_string(),
    solicitacao_pagador: Some("Bill #2024-001".to_string()),
    info_adicionais: None,
};

let cobv_response = client.cobv_create(&cobv_payload)?;
println!("Expiring charge created: {}", cobv_response.txid);

// Update the charge
let updated = client.cobv_update(&cobv_response.txid, &cobv_payload)?;
println!("Updated charge status: {}", updated.status.unwrap_or_default());
```

### Webhook Management

Register and manage webhooks to receive payment notifications:

```rust
use efi_bank::WebhookPayload;

let webhook_payload = WebhookPayload {
    url: "https://your-api.com/webhooks/pix".to_string(),
    chave: "seu-pix-key@example.com".to_string(),
};

// Register a webhook
let webhook = client.webhook_create(&webhook_payload)?;
println!("Webhook registered: {}", webhook.id);

// List all webhooks
let webhooks = client.webhook_list()?;
for wh in webhooks.webhooks {
    println!("Webhook: {} -> {}", wh.id, wh.url);
}

// Delete a webhook
client.webhook_delete(&webhook.id)?;
println!("Webhook deleted");
```

### PIX Transactions

Send and track PIX transfers:

```rust
use efi_bank::PixTransactionPayload;

let transaction_payload = PixTransactionPayload {
    valor: "100.00".to_string(),
    chave_destino: "recipient-pix-key@example.com".to_string(),
    descricao: Some("Payment for services".to_string()),
    idempotency_key: Some("unique-key-2024-001".to_string()),
};

let transaction = client.pix_send(&transaction_payload)?;
println!("Transaction sent: {}", transaction.end_to_end_id);
println!("Status: {}", transaction.status);

// Get transaction details
let details = client.pix_get_transaction(&transaction.end_to_end_id)?;
println!("Transaction status: {}", details.status);
if let Some(cancel_reason) = details.motivo_cancelamento {
    println!("Cancellation reason: {}", cancel_reason);
}
```

## Error Handling

The SDK returns `Result<T, EfiError>` for all operations:

```rust
use efi_bank::EfiError;

match client.cob_create(&payload) {
    Ok(response) => println!("Success: {}", response.txid),
    Err(EfiError::Http { status, body }) => {
        eprintln!("HTTP Error {}: {}", status, body);
    }
    Err(EfiError::Json(e)) => eprintln!("JSON parsing error: {}", e),
    Err(EfiError::AuthUnavailable) => eprintln!("Authentication failed"),
    Err(e) => eprintln!("Error: {:?}", e),
}
```
