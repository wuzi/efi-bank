use serde::de::{self, Visitor};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_with::skip_serializing_none;

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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitConfigResponse {
    pub id: String,
    pub descricao: Option<String>,
    pub lancamento: Option<SplitLancamento>,
    pub split: Option<SplitConfig>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SplitLinkResponse {
    pub status: String,
    pub message: Option<String>,
}

// ========== COB - Immediate Charge (Cobrança Imediata) ==========

#[skip_serializing_none]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobCalendario {
    pub data_de_vencimento: Option<String>,
    pub validade_apos_vencimento: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobPessoa {
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub nome: Option<String>,
}

#[skip_serializing_none]
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

#[skip_serializing_none]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CobCalendarioResponse {
    pub criacao_date_time: Option<String>,
    pub data_de_vencimento: Option<String>,
    pub validade_apos_vencimento: Option<i32>,
}

// ========== COBV - Expiring Charge (Cobrança com Vencimento) ==========

#[skip_serializing_none]
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

#[skip_serializing_none]
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

#[skip_serializing_none]
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

#[skip_serializing_none]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PixTransactionPayload {
    pub valor: String,
    pub chave_destino: String,
    pub descricao: Option<String>,
    pub idempotency_key: Option<String>,
}

#[skip_serializing_none]
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

#[skip_serializing_none]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingRepasse {
    pub payee_code: String,
    /// transfer percentage, where 9000 equals 90%
    pub percentage: Option<i32>,
    pub fixed: Option<i32>,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
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
pub struct BillingMarketplaceResponse {
    pub repasses: Vec<BillingRepasse>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingItem {
    pub name: String,
    pub value: i64,
    pub amount: i32,
    pub marketplace: Option<BillingMarketplace>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingItemResponse {
    pub name: String,
    pub value: i64,
    pub amount: i32,
    pub marketplace: Option<BillingMarketplaceResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingShipping {
    pub name: String,
    pub value: i64,
}

#[skip_serializing_none]
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

#[skip_serializing_none]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingBilletConfigurations {
    pub fine: Option<i64>,
    pub interest: Option<i64>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingBankingBillet {
    pub expire_at: Option<String>,
    pub customer: BillingCustomer,
    pub configurations: Option<BillingBilletConfigurations>,
    pub message: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPayment {
    pub banking_billet: Option<BillingBankingBillet>,
    pub credit_card: Option<BillingCreditCard>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingCreditCard {
    pub customer: BillingCustomer,
    pub installments: i32,
    pub payment_token: String,
    pub billing_address: Option<BillingAddress>,
}

#[skip_serializing_none]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeCreateRequest {
    pub items: Vec<BillingItem>,
    pub metadata: Option<BillingChargeMetadata>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeOneStepRequest {
    pub items: Vec<BillingItem>,
    pub shippings: Option<Vec<BillingShipping>>,
    pub payment: BillingPayment,
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
pub struct BillingChargeDetailResponse {
    pub code: i32,
    pub data: BillingChargeDetailData,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeDetailData {
    pub charge_id: i64,
    pub total: i64,
    pub status: String,
    pub custom_id: Option<String>,
    pub created_at: String,
    pub notification_url: Option<String>,
    pub items: Vec<BillingItemResponse>,
    pub history: Vec<BillingChargeHistoryEntry>,
    pub shippings: Option<Vec<BillingChargeShipping>>,
    pub customer: Option<BillingCustomer>,
    pub payment: BillingChargeDetailPayment,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeHistoryEntry {
    pub message: String,
    pub created_at: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeShipping {
    pub name: String,
    pub value: i64,
    pub payee_code: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeDetailPayment {
    pub method: String,
    pub created_at: String,
    pub message: Option<String>,
    pub banking_billet: BillingChargeDetailBankingBillet,
    pub credit_card: Option<serde_json::Value>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeDetailBankingBillet {
    pub barcode: String,
    pub pix: BillingPix,
    pub link: String,
    pub billet_link: String,
    pub pdf: BillingPdf,
    pub expire_at: String,
}

#[skip_serializing_none]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPix {
    pub qrcode: String,
    pub qrcode_image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingPdf {
    pub charge: String,
}

// ========== Billing API - Lifecycle Reads ===========

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BillingActionResponse {
    pub code: i32,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BillingChargeListQuery {
    pub charge_type: String,
    pub begin_date: String,
    pub end_date: String,
    /// Correlates Efí charges with caller records; it is not an idempotency key.
    pub custom_id: Option<String>,
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub offset: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeListResponse {
    pub code: i32,
    pub data: Vec<BillingChargeListItem>,
    pub params: BillingChargeListParams,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeListParams {
    pub begin_date: String,
    pub end_date: String,
    pub pagination: BillingPagination,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BillingPagination {
    pub limit: u32,
    pub offset: u32,
    pub page: u32,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeListItem {
    /// The child charge identifier returned by the beta list endpoint.
    pub id: i64,
    /// Present only when Efí includes the parent carnet identity.
    pub carnet_id: Option<i64>,
    pub total: i64,
    pub status: String,
    pub custom_id: Option<String>,
    pub created_at: Option<String>,
    pub customer: Option<BillingLifecycleCustomer>,
    pub payment: Option<BillingChargeListPayment>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingLifecycleCustomer {
    pub name: Option<String>,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<BillingAddress>,
    pub juridical_person: Option<BillingJuridicalPerson>,
}

impl BillingLifecycleCustomer {
    #[must_use]
    pub fn tax_id(&self) -> Option<&str> {
        self.cpf.as_deref().or(self.cnpj.as_deref()).or_else(|| {
            self.juridical_person
                .as_ref()
                .map(|person| person.cnpj.as_str())
        })
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeListPayment {
    pub payment_method: String,
    pub received_by_bank_at: Option<String>,
    pub paid_at: Option<String>,
    pub paid_value: Option<i64>,
    pub banking_billet: Option<BillingChargeListBankingBillet>,
    pub carnet: Option<BillingChargeListCarnet>,
    pub pix: Option<BillingPix>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeListBankingBillet {
    pub barcode: Option<String>,
    pub link: Option<String>,
    pub expire_at: Option<String>,
    pub pdf: Option<BillingPdf>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeListCarnet {
    /// The beta endpoint may omit this parent identity from child rows.
    pub carnet_id: Option<i64>,
    pub parcel: CarnetParcelNumber,
    pub barcode: Option<String>,
    pub expire_at: Option<String>,
    pub link: Option<String>,
    pub pdf: Option<BillingPdf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeReadResponse {
    pub code: i32,
    pub data: BillingChargeReadData,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeReadData {
    pub charge_id: i64,
    pub total: i64,
    pub status: String,
    pub custom_id: Option<String>,
    pub created_at: Option<String>,
    pub notification_url: Option<String>,
    pub items: Option<Vec<BillingItemResponse>>,
    pub customer: Option<BillingLifecycleCustomer>,
    pub payment: Option<BillingChargeReadPayment>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeReadPayment {
    pub method: Option<String>,
    pub created_at: Option<String>,
    pub banking_billet: Option<BillingChargeReadBankingBillet>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingChargeReadBankingBillet {
    pub barcode: Option<String>,
    pub link: Option<String>,
    pub billet_link: Option<String>,
    pub expire_at: Option<String>,
    pub pdf: Option<BillingPdf>,
    pub pix: Option<BillingPix>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetDetailResponse {
    pub code: i32,
    pub data: CarnetDetailData,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetDetailData {
    pub carnet_id: i64,
    pub status: String,
    pub repeats: Option<u32>,
    pub value: Option<i64>,
    pub custom_id: Option<String>,
    pub notification_url: Option<String>,
    pub split_items: Option<bool>,
    pub created_at: Option<String>,
    pub cover: Option<String>,
    pub link: Option<String>,
    pub carnet_link: Option<String>,
    pub pdf: Option<CarnetPdf>,
    pub charges: Vec<CarnetDetailCharge>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetDetailCharge {
    pub charge_id: i64,
    pub parcel: CarnetParcelNumber,
    pub status: String,
    pub value: Option<i64>,
    pub expire_at: Option<String>,
    pub url: Option<String>,
    pub parcel_link: Option<String>,
    pub pdf: Option<CarnetChargePdf>,
    pub barcode: Option<String>,
    pub pix: Option<BillingPix>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CarnetParcelNumber(u32);

impl CarnetParcelNumber {
    #[must_use]
    pub const fn new(value: u32) -> Option<Self> {
        if value == 0 { None } else { Some(Self(value)) }
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }
}

impl Serialize for CarnetParcelNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.0)
    }
}

struct CarnetParcelNumberVisitor;

impl<'de> Visitor<'de> for CarnetParcelNumberVisitor {
    type Value = CarnetParcelNumber;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("a positive carnet parcel number or numeric string")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = u32::try_from(value).map_err(E::custom)?;
        CarnetParcelNumber::new(value).ok_or_else(|| E::custom("parcel number must be positive"))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = u32::try_from(value).map_err(E::custom)?;
        CarnetParcelNumber::new(value).ok_or_else(|| E::custom("parcel number must be positive"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        let value = value.parse::<u32>().map_err(E::custom)?;
        CarnetParcelNumber::new(value).ok_or_else(|| E::custom("parcel number must be positive"))
    }
}

impl<'de> Deserialize<'de> for CarnetParcelNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(CarnetParcelNumberVisitor)
    }
}

// ========== Billing API - Carnet ===========

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetCustomer {
    pub name: Option<String>,
    pub cpf: Option<String>,
    pub cnpj: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub birth: Option<String>,
    pub address: Option<BillingAddress>,
    pub juridical_person: Option<BillingJuridicalPerson>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetCreateRequest {
    pub items: Vec<BillingItem>,
    pub customer: CarnetCustomer,
    pub expire_at: String,
    pub message: Option<String>,
    pub repeats: i32,
    pub split_items: Option<bool>,
    pub metadata: Option<CarnetMetadataRequest>,
    pub configurations: Option<BillingBilletConfigurations>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetMetadataRequest {
    pub notification_url: Option<String>,
    pub custom_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetParcelRequest {
    pub expire_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetParcelsRequest {
    pub parcels: Vec<CarnetParcel>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetParcel {
    pub parcel: i32,
    pub expire_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetHistoryRequest {
    pub description: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetData {
    pub carnet_id: i64,
    pub status: String,
    pub cover: String,
    pub link: String,
    pub carnet_link: String,
    pub pdf: CarnetPdf,
    pub charges: Vec<CarnetCharge>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetPdf {
    pub carnet: String,
    pub cover: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetCharge {
    pub charge_id: i64,
    pub parcel: String,
    pub status: String,
    pub value: i64,
    pub expire_at: String,
    pub url: String,
    pub parcel_link: String,
    pub pdf: CarnetChargePdf,
    pub barcode: String,
    pub pix: BillingPix,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetChargePdf {
    pub charge: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarnetResponse {
    pub code: i32,
    pub data: CarnetData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingNotificationResponse {
    pub code: i32,
    pub data: Vec<BillingNotificationData>,
}

#[skip_serializing_none]
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

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingNotificationIdentifiers {
    pub charge_id: Option<i64>,
    pub carnet_id: Option<i64>,
    pub subscription_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingNotificationStatus {
    pub current: String,
    pub previous: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::BillingNotificationResponse;

    #[test]
    fn deserializes_mixed_carnet_notification_history() {
        let response: BillingNotificationResponse = serde_json::from_value(serde_json::json!({
            "code": 200,
            "data": [
                {
                    "created_at": "2022-03-22 09:38:36",
                    "custom_id": null,
                    "id": 1,
                    "identifiers": { "carnet_id": 2512240 },
                    "status": { "current": "up_to_date", "previous": null },
                    "type": "carnet"
                },
                {
                    "created_at": "2022-04-03 07:34:22",
                    "custom_id": null,
                    "id": 2,
                    "identifiers": {
                        "carnet_id": 2512240,
                        "charge_id": 27757742
                    },
                    "status": { "current": "paid", "previous": "waiting" },
                    "type": "carnet_charge",
                    "value": 6250
                }
            ]
        }))
        .unwrap();

        assert_eq!(response.data[0].identifiers.charge_id, None);
        assert_eq!(response.data[0].identifiers.carnet_id, Some(2512240));
        assert_eq!(response.data[1].identifiers.charge_id, Some(27757742));
    }
}
