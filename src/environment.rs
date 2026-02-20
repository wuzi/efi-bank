#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Environment {
    Sandbox,
    Production,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Endpoints {
    pub pix_api_base_url: &'static str,
    pub oauth_token_url: &'static str,
}

pub const SANDBOX_ENDPOINTS: Endpoints = Endpoints {
    pix_api_base_url: "https://pix-h.api.efipay.com.br",
    oauth_token_url: "https://pix-h.api.efipay.com.br/oauth/token",
};

pub const PRODUCTION_ENDPOINTS: Endpoints = Endpoints {
    pix_api_base_url: "https://pix.api.efipay.com.br",
    oauth_token_url: "https://pix.api.efipay.com.br/oauth/token",
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
