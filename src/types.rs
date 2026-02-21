use serde::{Deserialize, Serialize};

// ========== Split de pagamento Pix ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitRepasse {
    pub tipo: String,
    pub valor: String,
    pub favorecido: SplitFavorecido,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitFavorecido {
    pub cpf: String,
    pub conta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitParte {
    pub tipo: String,
    pub valor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitPayload {
    pub descricao: String,
    pub lancamento: SplitLancamento,
    pub split: SplitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitLancamento {
    pub imediato: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitConfig {
    pub divisao_tarifa: String,
    pub minha_parte: SplitParte,
    pub repasses: Vec<SplitRepasse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitConfigResponse {
    pub id: String,
    pub descricao: Option<String>,
    pub lancamento: Option<SplitLancamento>,
    pub split: Option<SplitConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitLinkResponse {
    pub status: String,
    pub message: Option<String>,
}

// ========== COB - Immediate Charge (Cobrança Imediata) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobPayload {
    pub calendario: CobCalendario,
    pub devedor: Option<CobPessoa>,
    pub valor: CobValor,
    pub chave: String,
    pub solicitacao_pagador: Option<String>,
    pub info_adicionais: Option<Vec<CobInfoAdicional>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobCalendario {
    pub data_de_vencimento: Option<String>,
    pub validade_apos_vencimento: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobPessoa {
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub nome: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobValor {
    pub original: String,
    pub multa: Option<String>,
    pub juros: Option<String>,
    pub desconto: Option<String>,
    pub abatimento: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobInfoAdicional {
    pub nome: String,
    pub valor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobResponse {
    pub txid: String,
    pub calendario: Option<CobCalendarioResponse>,
    pub devedor: Option<CobPessoa>,
    pub valor: Option<CobValor>,
    pub chave: Option<String>,
    pub solicitacao_pagador: Option<String>,
    pub info_adicionais: Option<Vec<CobInfoAdicional>>,
    pub br: Option<String>,
    pub status: Option<String>,
    pub criacao_date_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobCalendarioResponse {
    pub criacao_date_time: Option<String>,
    pub data_de_vencimento: Option<String>,
    pub validade_apos_vencimento: Option<i32>,
}

// ========== COBV - Expiring Charge (Cobrança com Vencimento) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobvPayload {
    pub calendario: CobvCalendario,
    pub devedor: Option<CobPessoa>,
    pub valor: CobValor,
    pub chave: String,
    pub solicitacao_pagador: Option<String>,
    pub info_adicionais: Option<Vec<CobInfoAdicional>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobvCalendario {
    pub data_de_vencimento: String,
    pub validade_apos_vencimento: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobvResponse {
    pub txid: String,
    pub calendario: Option<CobvCalendarioResponse>,
    pub devedor: Option<CobPessoa>,
    pub valor: Option<CobValor>,
    pub chave: Option<String>,
    pub solicitacao_pagador: Option<String>,
    pub info_adicionais: Option<Vec<CobInfoAdicional>>,
    pub br: Option<String>,
    pub status: Option<String>,
    pub criacao_date_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobvCalendarioResponse {
    pub criacao_date_time: Option<String>,
    pub data_de_vencimento: Option<String>,
    pub validade_apos_vencimento: Option<i32>,
}

// ========== Webhook Management ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub url: String,
    pub chave: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookResponse {
    pub id: String,
    pub url: String,
    pub chave: Option<String>,
    pub criacao_date_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhooksListResponse {
    pub webhooks: Vec<WebhookResponse>,
}

// ========== PIX Transactions ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixTransactionPayload {
    pub valor: String,
    pub chave_destino: String,
    pub descricao: Option<String>,
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixTransactionResponse {
    pub id: String,
    pub end_to_end_id: String,
    pub valor: String,
    pub status: String,
    pub chave_destino: Option<String>,
    pub data_hora: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixTransactionDetailResponse {
    pub id: String,
    pub end_to_end_id: String,
    pub valor: String,
    pub status: String,
    pub chave_destino: Option<String>,
    pub chave_origem: Option<String>,
    pub data_hora: Option<String>,
    pub motivo_cancelamento: Option<String>,
}

// ========== Billing API - Split de Pagamento ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingRepasse {
    pub payee_code: String,
    /// transfer percentage, where 9000 equals 90%
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BillingRepasseType {
    /// fee is deducted only from the account that issued the charge
    IssuerOnly = 1,
    /// fee is deducted proportionally according to the percentage defined for each account receiving the transfer
    Proportional = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingMarketplace {
    pub repasses: Vec<BillingRepasse>,
    pub mode: BillingRepasseType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingItem {
    pub name: String,
    pub value: i64,
    pub amount: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace: Option<BillingMarketplace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingShipping {
    pub name: String,
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    pub street: Option<String>,
    pub number: Option<String>,
    pub neighborhood: Option<String>,
    pub zipcode: Option<String>,
    pub city: Option<String>,
    pub complement: Option<String>,
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingJuridicalPerson {
    pub corporate_name: String,
    pub cnpj: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingCustomer {
    pub name: Option<String>,
    pub cpf: Option<String>,
    pub email: Option<String>,
    pub birth: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<BillingAddress>,
    pub juridical_person: Option<BillingJuridicalPerson>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingBilletConfigurations {
    pub fine: Option<i64>,
    pub interest: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingBankingBillet {
    pub expire_at: Option<String>,
    pub customer: BillingCustomer,
    pub configurations: Option<BillingBilletConfigurations>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPayment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banking_billet: Option<BillingBankingBillet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<BillingCreditCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingCreditCard {
    pub customer: BillingCustomer,
    pub installments: i32,
    pub payment_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddress>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeMetadata {
    /// Your valid URL address that will receive notifications of transaction status changes.
    /// Maximum of 255 characters.
    pub notification_url: Option<String>,
    /// Allows associating an Efí transaction with a specific ID from your system or application,
    /// allowing you to identify it if you have a specific identification and want to maintain it.
    /// Maximum of 255 characters.
    pub custom_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeCreateRequest {
    pub items: Vec<BillingItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BillingChargeMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeOneStepRequest {
    pub items: Vec<BillingItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippings: Option<Vec<BillingShipping>>,
    pub payment: BillingPayment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BillingChargeMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargePayRequest {
    pub payment: BillingPayment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeResponse {
    pub code: i32,
    pub data: BillingChargeData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeData {
    pub barcode: String,
    #[serde(rename = "pix")]
    pub pix: Option<BillingPix>,
    pub link: String,
    pub billet_link: String,
    pub pdf: BillingPdf,
    pub expire_at: String,
    pub charge_id: i64,
    pub status: String,
    pub total: i64,
    pub payment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPix {
    pub qrcode: Option<String>,
    pub qrcode_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPdf {
    pub charge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingNotificationResponse {
    pub code: i32,
    pub data: Vec<BillingNotificationData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingNotificationData {
    pub created_at: String,
    pub custom_id: Option<String>,
    pub id: i64,
    pub identifiers: BillingNotificationIdentifiers,
    pub status: BillingNotificationStatus,
    #[serde(rename = "type")]
    pub r#type: String,
    pub received_by_bank_at: Option<String>,
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingNotificationIdentifiers {
    pub charge_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingNotificationStatus {
    pub current: String,
    pub previous: Option<String>,
}
