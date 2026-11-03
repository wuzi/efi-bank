#![doc = include_str!("../README.md")]

mod response;

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
    BillingBilletUpdateRequest, BillingChargeCreateRequest, BillingChargeData, BillingChargeDateOf,
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
    CarnetResendRequest, CarnetResponse, CobCalendario, CobCalendarioResponse, CobInfoAdicional,
    CobListResponse, CobPayload, CobPessoa, CobResponse, CobValor, CobvCalendario,
    CobvCalendarioResponse, CobvListResponse, CobvPayload, CobvResponse, CobvValor, JsonObject,
    PixChargeListQuery, PixRecipient, PixSender, PixTransactionDetailResponse,
    PixTransactionPayload, PixTransactionResponse, SplitChargeResponse, SplitConfig,
    SplitConfigResponse, SplitFavorecido, SplitLancamento, SplitParte, SplitPayload, SplitRepasse,
    WebhookListQuery, WebhookPayload, WebhookResponse, WebhooksListResponse,
};

#[cfg(test)]
mod transport_tests;

#[cfg(test)]
mod boundary_tests;

#[cfg(test)]
mod test_support;

#[cfg(test)]
mod protocol_tests;
