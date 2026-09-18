pub mod auth;
pub mod billing;
pub mod carnet;
pub mod client;
pub mod cob;
pub mod cobv;
pub mod environment;
pub mod error;
pub mod pix;
pub mod split;
pub mod types;
pub mod webhooks;

#[cfg(test)]
mod lifecycle_tests;

pub use client::{Client, ClientBuilder};
pub use environment::{Endpoints, Environment, PRODUCTION_ENDPOINTS, SANDBOX_ENDPOINTS};
pub use error::Error;
pub use types::{
    BillingActionResponse, BillingAddress, BillingBankingBillet, BillingBilletConfigurations,
    BillingChargeCreateRequest, BillingChargeData, BillingChargeDateOf,
    BillingChargeDetailBankingBillet, BillingChargeDetailData, BillingChargeDetailPayment,
    BillingChargeDetailResponse, BillingChargeHistoryEntry, BillingChargeListBankingBillet,
    BillingChargeListCarnet, BillingChargeListItem, BillingChargeListParams,
    BillingChargeListPayment, BillingChargeListQuery, BillingChargeListResponse,
    BillingChargeOneStepRequest, BillingChargePayRequest, BillingChargeReadBankingBillet,
    BillingChargeReadData, BillingChargeReadPayment, BillingChargeReadResponse,
    BillingChargeResponse, BillingChargeShipping, BillingCreditCard, BillingCustomer, BillingItem,
    BillingLifecycleCustomer, BillingMarketplace, BillingNotificationResponse, BillingPagination,
    BillingPayment, BillingPdf, BillingPix, BillingRepasse, BillingShipping, CarnetCharge,
    CarnetChargePdf, CarnetCreateRequest, CarnetCustomer, CarnetData, CarnetDetailCharge,
    CarnetDetailData, CarnetDetailResponse, CarnetHistoryRequest, CarnetMetadataRequest,
    CarnetParcel, CarnetParcelNumber, CarnetParcelRequest, CarnetParcelsRequest, CarnetPdf,
    CarnetResponse, CobCalendario, CobInfoAdicional, CobPayload, CobPessoa, CobResponse, CobValor,
    CobvCalendario, CobvPayload, CobvResponse, PixTransactionDetailResponse, PixTransactionPayload,
    PixTransactionResponse, SplitConfig, SplitConfigResponse, SplitFavorecido, SplitLancamento,
    SplitLinkResponse, SplitParte, SplitPayload, SplitRepasse, WebhookPayload, WebhookResponse,
    WebhooksListResponse,
};

#[cfg(test)]
mod transport_tests;
