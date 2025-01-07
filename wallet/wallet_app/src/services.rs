use app_config::ErrorResponse;
use sqlx::PgPool;
use uuid::Uuid;
use wallet_api::Wallet;

#[derive(Debug)]
pub struct WalletService {
    pool: PgPool,
}

impl WalletService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_wallet(&self, user_id: Uuid) -> Result<Wallet, ErrorResponse> {
        sqlx::query_as!(Wallet, "SELECT * FROM wallets WHERE user_id = $1", user_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ErrorResponse::not_found("Wallet not found"))
    }

    pub async fn update_wallet(
        &self,
        user_id: Uuid,
        wallet: Wallet,
    ) -> Result<Wallet, ErrorResponse> {
        sqlx::query!(
            r#"
            INSERT INTO wallets (user_id, encrypted_private_key, address, balance, salt, token_decimals)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (user_id)
            DO UPDATE SET
                encrypted_private_key = EXCLUDED.encrypted_private_key,
                address = EXCLUDED.address,
                balance = EXCLUDED.balance,
                salt = EXCLUDED.salt,
                token_decimals = EXCLUDED.token_decimals
            RETURNING *
            "#,
            user_id,
            wallet.encrypted_private_key,
            wallet.address,
            wallet.balance,
            wallet.salt,
            wallet.token_decimals
        )
        .fetch_one(&self.pool)
        .await
        .map(|row| Wallet {
            id: row.id,
            user_id: row.user_id,
            encrypted_private_key: row.encrypted_private_key,
            address: row.address,
            balance: row.balance,
            salt: row.salt,
            token_decimals: row.token_decimals,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .map_err(|e| {
            log::error!("Error updating wallet: {:?}", e);
            ErrorResponse::internal_error()
        })
    }
}
