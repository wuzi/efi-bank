#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Sandbox,
    Production,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Endpoints {
    pub pix_api_base_url: &'static str,
    pub pix_api_oauth_token_url: &'static str,
    pub billing_api_base_url: &'static str,
    pub billing_api_oauth_token_url: &'static str,
}

pub const SANDBOX_ENDPOINTS: Endpoints = Endpoints {
    pix_api_base_url: "https://pix-h.api.efipay.com.br",
    pix_api_oauth_token_url: "https://pix-h.api.efipay.com.br/oauth/token",
    billing_api_base_url: "https://cobrancas-h.api.efipay.com.br",
    billing_api_oauth_token_url: "https://cobrancas-h.api.efipay.com.br/v1/authorize",
};

pub const PRODUCTION_ENDPOINTS: Endpoints = Endpoints {
    pix_api_base_url: "https://pix.api.efipay.com.br",
    pix_api_oauth_token_url: "https://pix.api.efipay.com.br/oauth/token",
    billing_api_base_url: "https://cobrancas.api.efipay.com.br",
    billing_api_oauth_token_url: "https://cobrancas.api.efipay.com.br/v1/authorize",
};

impl Environment {
    #[must_use]
    pub const fn endpoints(self) -> Endpoints {
        match self {
            Self::Sandbox => SANDBOX_ENDPOINTS,
            Self::Production => PRODUCTION_ENDPOINTS,
        }
    }
}
