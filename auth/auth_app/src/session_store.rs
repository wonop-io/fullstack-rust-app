use std::time::SystemTime;

// Modified version of https://github.com/maxcountryman/tower-sessions-stores/blob/main/sqlx-store/src/postgres_store.rs
use async_trait::async_trait;
use chrono::{DateTime, Utc};
pub use sqlx;
use sqlx::{PgConnection, PgPool};
use tokio::time::{interval, Duration};
use tower_sessions_core::{
    session::{Id, Record},
    session_store, ExpiredDeletion, SessionStore,
};

/// An error type for SQLx stores.
#[derive(thiserror::Error, Debug)]
pub enum SqlxStoreError {
    /// A variant to map `sqlx` errors.
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    /// A variant to map `rmp_serde` encode errors.
    #[error(transparent)]
    Encode(#[from] rmp_serde::encode::Error),

    /// A variant to map `rmp_serde` decode errors.
    #[error(transparent)]
    Decode(#[from] rmp_serde::decode::Error),
}

impl From<SqlxStoreError> for session_store::Error {
    fn from(err: SqlxStoreError) -> Self {
        match err {
            SqlxStoreError::Sqlx(inner) => session_store::Error::Backend(inner.to_string()),
            SqlxStoreError::Decode(inner) => session_store::Error::Decode(inner.to_string()),
            SqlxStoreError::Encode(inner) => session_store::Error::Encode(inner.to_string()),
        }
    }
}
#[derive(Clone, Debug)]
pub struct PostgresStore {
    pool: PgPool,
    schema_name: String,
    table_name: String,
}

impl PostgresStore {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            schema_name: "tower_sessions".to_string(),
            table_name: "session".to_string(),
        }
    }

    /// Set the session table schema name with the provided name.
    pub fn with_schema_name(mut self, schema_name: impl AsRef<str>) -> Result<Self, String> {
        let schema_name = schema_name.as_ref();
        if !is_valid_identifier(schema_name) {
            return Err(format!(
                "Invalid schema name '{}'. Schema names must start with a letter or underscore \
                 (including letters with diacritical marks and non-Latin letters).Subsequent \
                 characters can be letters, underscores, digits (0-9), or dollar signs ($).",
                schema_name
            ));
        }

        schema_name.clone_into(&mut self.schema_name);
        Ok(self)
    }

    /// Set the session table name with the provided name.
    pub fn with_table_name(mut self, table_name: impl AsRef<str>) -> Result<Self, String> {
        let table_name = table_name.as_ref();
        if !is_valid_identifier(table_name) {
            return Err(format!(
                "Invalid table name '{}'. Table names must start with a letter or underscore \
                 (including letters with diacritical marks and non-Latin letters).Subsequent \
                 characters can be letters, underscores, digits (0-9), or dollar signs ($).",
                table_name
            ));
        }

        table_name.clone_into(&mut self.table_name);
        Ok(self)
    }

    pub async fn migrate(&self) -> sqlx::Result<()> {
        let mut tx = self.pool.begin().await?;

        let create_schema_query = format!(
            r#"create schema if not exists "{schema_name}""#,
            schema_name = self.schema_name,
        );
        // Concurrent create schema may fail due to duplicate key violations.
        //
        // This works around that by assuming the schema must exist on such an error.
        if let Err(err) = sqlx::query(&create_schema_query).execute(&mut *tx).await {
            if !err
                .to_string()
                .contains("duplicate key value violates unique constraint")
            {
                return Err(err);
            }

            return Ok(());
        }

        let create_table_query = format!(
            r#"
            create table if not exists "{schema_name}"."{table_name}"
            (
                id text primary key not null,
                data bytea not null,
                expiry_date timestamptz not null
            )
            "#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );
        sqlx::query(&create_table_query).execute(&mut *tx).await?;

        tx.commit().await?;

        Ok(())
    }

    async fn id_exists(&self, conn: &mut PgConnection, id: &Id) -> session_store::Result<bool> {
        let query = format!(
            r#"
            select exists(select 1 from "{schema_name}"."{table_name}" where id = $1)
            "#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );

        Ok(sqlx::query_scalar(&query)
            .bind(id.to_string())
            .fetch_one(conn)
            .await
            .map_err(SqlxStoreError::Sqlx)?)
    }

    async fn save_with_conn(
        &self,
        conn: &mut PgConnection,
        record: &Record,
    ) -> session_store::Result<()> {
        let query = format!(
            r#"
            insert into "{schema_name}"."{table_name}" (id, data, expiry_date)
            values ($1, $2, $3)
            on conflict (id) do update
            set
              data = excluded.data,
              expiry_date = excluded.expiry_date
            "#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );

        let expiry_date: SystemTime = record.expiry_date.into();
        let expiry_date = DateTime::<Utc>::from(expiry_date);

        sqlx::query(&query)
            .bind(record.id.to_string())
            .bind(rmp_serde::to_vec(&record).map_err(SqlxStoreError::Encode)?)
            .bind(expiry_date)
            .execute(conn)
            .await
            .map_err(SqlxStoreError::Sqlx)?;

        Ok(())
    }

    pub async fn continuously_delete_expired(self, delta: Duration) {
        let mut interval = interval(delta);
        loop {
            interval.tick().await;
            if let Err(e) = self.delete_expired().await {
                eprintln!("Error deleting expired sessions: {:?}", e);
            }
        }
    }
}

#[async_trait]
impl ExpiredDeletion for PostgresStore {
    async fn delete_expired(&self) -> session_store::Result<()> {
        let query = format!(
            r#"
            delete from "{schema_name}"."{table_name}"
            where expiry_date < (now() at time zone 'utc')
            "#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );
        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .map_err(SqlxStoreError::Sqlx)?;
        Ok(())
    }
}

#[async_trait]
impl SessionStore for PostgresStore {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let mut tx = self.pool.begin().await.map_err(SqlxStoreError::Sqlx)?;

        while self.id_exists(&mut tx, &record.id).await? {
            record.id = Id::default();
        }
        self.save_with_conn(&mut tx, record).await?;

        tx.commit().await.map_err(SqlxStoreError::Sqlx)?;

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let mut conn = self.pool.acquire().await.map_err(SqlxStoreError::Sqlx)?;
        self.save_with_conn(&mut conn, record).await
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let query = format!(
            r#"
            select data from "{schema_name}"."{table_name}"
            where id = $1 and expiry_date > $2
            "#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );
        let record_value: Option<(Vec<u8>,)> = sqlx::query_as(&query)
            .bind(session_id.to_string())
            .bind(Utc::now())
            .fetch_optional(&self.pool)
            .await
            .map_err(SqlxStoreError::Sqlx)?;

        if let Some((data,)) = record_value {
            Ok(Some(
                rmp_serde::from_slice(&data).map_err(SqlxStoreError::Decode)?,
            ))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let query = format!(
            r#"delete from "{schema_name}"."{table_name}" where id = $1"#,
            schema_name = self.schema_name,
            table_name = self.table_name
        );
        sqlx::query(&query)
            .bind(session_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(SqlxStoreError::Sqlx)?;

        Ok(())
    }
}

fn is_valid_identifier(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .next()
            .map(|c| c.is_alphabetic() || c == '_')
            .unwrap_or_default()
        && name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '$')
}
