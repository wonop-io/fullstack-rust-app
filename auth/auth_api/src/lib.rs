use core::fmt;
use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub username: String,
    pub role: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "token_kind", rename_all = "lowercase")
)]
pub enum TokenKind {
    Access,
    Refresh,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenKind::Access => write!(f, "access"),
            TokenKind::Refresh => write!(f, "refresh"),
        }
    }
}

impl FromStr for TokenKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "access" => Ok(TokenKind::Access),
            "refresh" => Ok(TokenKind::Refresh),
            _ => Err(()),
        }
    }
}

impl From<String> for TokenKind {
    fn from(s: String) -> Self {
        TokenKind::from_str(&s).unwrap_or(TokenKind::Access)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct UserToken {
    pub user_id: Uuid,
    pub kind: TokenKind,
    pub token_uuid: Uuid,
    pub created_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserLogoutResponse {
    pub status: String,
}

// #[cfg(feature = "backend")]
// use database_manager::{Manager, Model, ModelManager, QueryCondition, QuerySet};

/*
#[cfg(feature = "backend")]
#[async_trait]
impl ModelManager<User> for Manager<User> {
    fn all(&self) -> QuerySet<User> {
        self.new_query_set(QueryBuilder::new("SELECT * FROM auth_users"))
    }

    fn filter<C: QueryCondition>(&self, condition: C) -> QuerySet<User> {
        let mut query_builder = QueryBuilder::new("SELECT * FROM auth_users WHERE ");
        query_builder = condition.apply(query_builder);
        self.new_query_set(query_builder)
    }

    async fn create(&self, data: &User) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            "INSERT INTO auth_users (id, name, username, role, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING *",
        )
        .bind(data.id)
        .bind(&data.name)
        .bind(&data.username)
        .bind(&data.role)
        .bind(data.created_at)
        .bind(data.updated_at)
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    async fn get(&self, id: Uuid) -> anyhow::Result<Option<User>> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM auth_users WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn update(&self, id: Uuid, data: &User) -> anyhow::Result<User> {
        let user = sqlx::query_as::<_, User>(
            "UPDATE auth_users SET name = $2, username = $3, role = $4, updated_at = $5
             WHERE id = $1
             RETURNING *",
        )
        .bind(id)
        .bind(&data.name)
        .bind(&data.username)
        .bind(&data.role)
        .bind(Utc::now())
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM auth_users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

#[cfg(feature = "backend")]
#[async_trait]
impl Model for User {
    type Manager = Manager<Self>;

    fn get_id(&self) -> Uuid {
        self.id
    }

    async fn save(&self, manager: &Self::Manager) -> anyhow::Result<()> {
        if self.id == Uuid::nil() {
            manager.create(self).await?;
        } else {
            manager.update(self.id, self).await?;
        }
        Ok(())
    }

    async fn delete(&self, manager: &Self::Manager) -> anyhow::Result<()> {
        manager.delete(self.id).await
    }
}


/*
#[cfg(feature = "backend")]
pub struct UsernameFilter<'b>(&'b String);


#[cfg(feature = "backend")]
impl<'b> QueryCondition for UsernameFilter<'b> {
    fn apply<'a>(&'b self, mut query: QueryBuilder<'a, Postgres>) -> QueryBuilder<'a, Postgres>
    where 'b: 'a {
        query.push("username = ").push_bind(self.0);
        query
    }
}
*/
*/
