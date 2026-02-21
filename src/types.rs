use serde::{Deserialize, Serialize};
use serde_json::Value;

// ========== Split de pagamento Pix ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitRepasse {
    #[serde(rename = "tipo")]
    pub tipo: String,
    #[serde(rename = "valor")]
    pub valor: String,
    #[serde(rename = "favorecido")]
    pub favorecido: SplitFavorecido,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitFavorecido {
    #[serde(rename = "cpf")]
    pub cpf: String,
    #[serde(rename = "conta")]
    pub conta: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitParte {
    #[serde(rename = "tipo")]
    pub tipo: String,
    #[serde(rename = "valor")]
    pub valor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitPayload {
    #[serde(rename = "descricao")]
    pub descricao: String,
    #[serde(rename = "lancamento")]
    pub lancamento: SplitLancamento,
    #[serde(rename = "split")]
    pub split: SplitConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitLancamento {
    #[serde(rename = "imediato")]
    pub imediato: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitConfig {
    #[serde(rename = "divisaoTarifa")]
    pub divisao_tarifa: String,
    #[serde(rename = "minhaParte")]
    pub minha_parte: SplitParte,
    #[serde(rename = "repasses")]
    pub repasses: Vec<SplitRepasse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitConfigResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "descricao")]
    pub descricao: Option<String>,
    #[serde(rename = "lancamento")]
    pub lancamento: Option<SplitLancamento>,
    #[serde(rename = "split")]
    pub split: Option<SplitConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitLinkResponse {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "message")]
    pub message: Option<String>,
}

// ========== COB - Immediate Charge (Cobrança Imediata) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobPayload {
    #[serde(rename = "calendario")]
    pub calendario: CobCalendario,
    #[serde(rename = "devedor")]
    pub devedor: Option<CobPessoa>,
    #[serde(rename = "valor")]
    pub valor: CobValor,
    #[serde(rename = "chave")]
    pub chave: String,
    #[serde(rename = "solicitacaoPagador")]
    pub solicitacao_pagador: Option<String>,
    #[serde(rename = "infoAdicionais")]
    pub info_adicionais: Option<Vec<CobInfoAdicional>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobCalendario {
    #[serde(rename = "dataDeVencimento")]
    pub data_de_vencimento: Option<String>,
    #[serde(rename = "validadeAposVencimento")]
    pub validade_apos_vencimento: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobPessoa {
    #[serde(rename = "cpf")]
    pub cpf: Option<String>,
    #[serde(rename = "cnpj")]
    pub cnpj: Option<String>,
    #[serde(rename = "nome")]
    pub nome: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobValor {
    #[serde(rename = "original")]
    pub original: String,
    #[serde(rename = "multa")]
    pub multa: Option<String>,
    #[serde(rename = "juros")]
    pub juros: Option<String>,
    #[serde(rename = "desconto")]
    pub desconto: Option<String>,
    #[serde(rename = "abatimento")]
    pub abatimento: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobInfoAdicional {
    #[serde(rename = "nome")]
    pub nome: String,
    #[serde(rename = "valor")]
    pub valor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobResponse {
    #[serde(rename = "txid")]
    pub txid: String,
    #[serde(rename = "calendario")]
    pub calendario: Option<CobCalendarioResponse>,
    #[serde(rename = "devedor")]
    pub devedor: Option<CobPessoa>,
    #[serde(rename = "valor")]
    pub valor: Option<CobValor>,
    #[serde(rename = "chave")]
    pub chave: Option<String>,
    #[serde(rename = "solicitacaoPagador")]
    pub solicitacao_pagador: Option<String>,
    #[serde(rename = "infoAdicionais")]
    pub info_adicionais: Option<Vec<CobInfoAdicional>>,
    #[serde(rename = "br")]
    pub br: Option<String>,
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "criacaoDateTime")]
    pub criacao_date_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobCalendarioResponse {
    #[serde(rename = "criacaoDateTime")]
    pub criacao_date_time: Option<String>,
    #[serde(rename = "dataDeVencimento")]
    pub data_de_vencimento: Option<String>,
    #[serde(rename = "validadeAposVencimento")]
    pub validade_apos_vencimento: Option<i32>,
}

// ========== COBV - Expiring Charge (Cobrança com Vencimento) ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobvPayload {
    #[serde(rename = "calendario")]
    pub calendario: CobvCalendario,
    #[serde(rename = "devedor")]
    pub devedor: Option<CobPessoa>,
    #[serde(rename = "valor")]
    pub valor: CobValor,
    #[serde(rename = "chave")]
    pub chave: String,
    #[serde(rename = "solicitacaoPagador")]
    pub solicitacao_pagador: Option<String>,
    #[serde(rename = "infoAdicionais")]
    pub info_adicionais: Option<Vec<CobInfoAdicional>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobvCalendario {
    #[serde(rename = "dataDeVencimento")]
    pub data_de_vencimento: String,
    #[serde(rename = "validadeAposVencimento")]
    pub validade_apos_vencimento: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobvResponse {
    #[serde(rename = "txid")]
    pub txid: String,
    #[serde(rename = "calendario")]
    pub calendario: Option<CobvCalendarioResponse>,
    #[serde(rename = "devedor")]
    pub devedor: Option<CobPessoa>,
    #[serde(rename = "valor")]
    pub valor: Option<CobValor>,
    #[serde(rename = "chave")]
    pub chave: Option<String>,
    #[serde(rename = "solicitacaoPagador")]
    pub solicitacao_pagador: Option<String>,
    #[serde(rename = "infoAdicionais")]
    pub info_adicionais: Option<Vec<CobInfoAdicional>>,
    #[serde(rename = "br")]
    pub br: Option<String>,
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "criacaoDateTime")]
    pub criacao_date_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CobvCalendarioResponse {
    #[serde(rename = "criacaoDateTime")]
    pub criacao_date_time: Option<String>,
    #[serde(rename = "dataDeVencimento")]
    pub data_de_vencimento: Option<String>,
    #[serde(rename = "validadeAposVencimento")]
    pub validade_apos_vencimento: Option<i32>,
}

// ========== Webhook Management ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "chave")]
    pub chave: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "chave")]
    pub chave: Option<String>,
    #[serde(rename = "criacaoDateTime")]
    pub criacao_date_time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhooksListResponse {
    #[serde(rename = "webhooks")]
    pub webhooks: Vec<WebhookResponse>,
}

// ========== PIX Transactions ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixTransactionPayload {
    #[serde(rename = "valor")]
    pub valor: String,
    #[serde(rename = "chaveDestino")]
    pub chave_destino: String,
    #[serde(rename = "descricao")]
    pub descricao: Option<String>,
    #[serde(rename = "idempotencyKey")]
    pub idempotency_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixTransactionResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "endToEndId")]
    pub end_to_end_id: String,
    #[serde(rename = "valor")]
    pub valor: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "chaveDestino")]
    pub chave_destino: Option<String>,
    #[serde(rename = "dataHora")]
    pub data_hora: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixTransactionDetailResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "endToEndId")]
    pub end_to_end_id: String,
    #[serde(rename = "valor")]
    pub valor: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "chaveDestino")]
    pub chave_destino: Option<String>,
    #[serde(rename = "chaveOrigem")]
    pub chave_origem: Option<String>,
    #[serde(rename = "dataHora")]
    pub data_hora: Option<String>,
    #[serde(rename = "motivoCancelamento")]
    pub motivo_cancelamento: Option<String>,
}

// ========== Billing API - Split de Pagamento ==========

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingRepasse {
    #[serde(rename = "payee_code")]
    pub payee_code: String,
    /// transfer percentage, where 9000 equals 90%
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i32>,
    #[serde(rename = "fixed", skip_serializing_if = "Option::is_none")]
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
    #[serde(rename = "repasses")]
    pub repasses: Vec<BillingRepasse>,
    pub mode: BillingRepasseType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingItem {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: i64,
    #[serde(rename = "amount")]
    pub amount: i32,
    #[serde(rename = "marketplace")]
    pub marketplace: BillingMarketplace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingShipping {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    #[serde(rename = "street")]
    pub street: Option<String>,
    #[serde(rename = "number")]
    pub number: Option<String>,
    #[serde(rename = "neighborhood")]
    pub neighborhood: Option<String>,
    #[serde(rename = "zipcode")]
    pub zipcode: Option<String>,
    #[serde(rename = "city")]
    pub city: Option<String>,
    #[serde(rename = "complement")]
    pub complement: Option<String>,
    #[serde(rename = "state")]
    pub state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingJuridicalPerson {
    pub corporate_name: String,
    pub cnpj: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingCustomer {
    pub name: String,
    pub cpf: String,
    pub email: Option<String>,
    pub birth: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<BillingAddress>,
    pub juridical_person: Option<BillingJuridicalPerson>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingBilletConfigurations {
    #[serde(rename = "fine")]
    pub fine: Option<i64>,
    #[serde(rename = "interest")]
    pub interest: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingBankingBillet {
    #[serde(rename = "expire_at")]
    pub expire_at: Option<String>,
    #[serde(rename = "customer")]
    pub customer: BillingCustomer,
    #[serde(rename = "configurations")]
    pub configurations: Option<BillingBilletConfigurations>,
    #[serde(rename = "message")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPayment {
    #[serde(rename = "banking_billet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banking_billet: Option<BillingBankingBillet>,
    #[serde(rename = "credit_card")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card: Option<BillingCreditCard>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingCreditCard {
    #[serde(rename = "customer")]
    pub customer: BillingCustomer,
    #[serde(rename = "installments")]
    pub installments: i32,
    #[serde(rename = "payment_token")]
    pub payment_token: String,
    #[serde(rename = "billing_address")]
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
    #[serde(rename = "items")]
    pub items: Vec<BillingItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BillingChargeMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeOneStepRequest {
    #[serde(rename = "items")]
    pub items: Vec<BillingItem>,
    #[serde(rename = "shippings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippings: Option<Vec<BillingShipping>>,
    #[serde(rename = "payment")]
    pub payment: BillingPayment,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BillingChargeMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargePayRequest {
    #[serde(rename = "payment")]
    pub payment: BillingPayment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeResponse {
    #[serde(rename = "code")]
    pub code: i32,
    #[serde(rename = "data")]
    pub data: BillingChargeData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeData {
    #[serde(rename = "barcode")]
    pub barcode: Option<String>,
    #[serde(rename = "pix")]
    pub pix: Option<BillingPix>,
    #[serde(rename = "link")]
    pub link: Option<String>,
    #[serde(rename = "billet_link")]
    pub billet_link: Option<String>,
    #[serde(rename = "pdf")]
    pub pdf: Option<BillingPdf>,
    #[serde(rename = "expire_at")]
    pub expire_at: Option<String>,
    #[serde(rename = "charge_id")]
    pub charge_id: Option<i64>,
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "total")]
    pub total: Option<i64>,
    #[serde(rename = "payment")]
    pub payment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPix {
    #[serde(rename = "qrcode")]
    pub qrcode: Option<String>,
    #[serde(rename = "qrcode_image")]
    pub qrcode_image: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPdf {
    #[serde(rename = "charge")]
    pub charge: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingNotificationResponse {
    #[serde(flatten)]
    pub data: Value,
}
