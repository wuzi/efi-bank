use serde::{Deserialize, Serialize};

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
