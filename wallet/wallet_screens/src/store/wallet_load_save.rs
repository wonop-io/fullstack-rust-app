#[cfg(target_arch = "wasm32")]
use reqwest::Client;
use wallet_api::wallet::Wallet;

#[async_trait::async_trait(?Send)]
pub trait SaveableWallet {
    async fn save(&self) -> Result<(), anyhow::Error>;
    async fn load() -> Result<Self, anyhow::Error>
    where
        Self: Sized;
}

#[async_trait::async_trait(?Send)]
impl SaveableWallet for Wallet {
    #[cfg(not(target_arch = "wasm32"))]
    async fn save(&self) -> Result<(), anyhow::Error> {
        Err(anyhow::anyhow!("Not implemented"))
    }

    #[cfg(target_arch = "wasm32")]
    async fn save(&self) -> Result<(), anyhow::Error> {
        let client = Client::new();
        let res = client
            .put("http://localhost:8080/api/v1/wallet")
            .json(self)
            .fetch_credentials_include()
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to save wallet"))
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    async fn load() -> Result<Self, anyhow::Error> {
        Err(anyhow::anyhow!("Not implemented"))
    }

    #[cfg(target_arch = "wasm32")]
    async fn load() -> Result<Self, anyhow::Error> {
        let client = Client::new();
        let res = client
            .get("http://localhost:8080/api/v1/wallet")
            .fetch_credentials_include()
            .send()
            .await?;

        if res.status().is_success() {
            Ok(res.json().await?)
        } else {
            Err(anyhow::anyhow!("Failed to load wallet"))
        }
    }
}
