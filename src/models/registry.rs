use anyhow::Result;
use chrono::Utc;
use serde::Deserialize;
use sqlx::FromRow;

use crate::web_server::AppState;

#[derive(Debug, FromRow)]
pub struct Registry {
    pub id: i64,
    pub name: String,
    pub host: String,
    pub port: i64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub cert_file: Option<String>,
    pub key_file: Option<String>,
    pub status: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Registry {
    pub fn new(name: String, host: String, port: i64) -> Self {
        Self {
            id: 0,
            name,
            host,
            port,
            ..Default::default()
        }
    }
}

impl Default for Registry {
    fn default() -> Self {
        Self {
            id: 0,
            name: "".to_string(),
            host: "".to_string(),
            port: 0,
            username: None,
            password: None,
            cert_file: None,
            key_file: None,
            status: None,
            created_at: Utc::now().timestamp(),
            updated_at: Utc::now().timestamp(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateRegistryInput {
    pub name: String,
    pub host: String,
    pub port: i64,
    pub ssl: Option<SslType>,
    pub auth: Option<Auth>,
}
#[derive(Debug, Deserialize)]
pub enum SslType {
    SelfSigned(SelfSignedSsl),
    Provided(ProvidedSsl),
}
#[derive(Debug, Deserialize)]
pub struct SelfSignedSsl {
    pub cert_dir: String,
}
#[derive(Debug, Deserialize)]
pub struct ProvidedSsl {
    pub cert_file: String,
    pub key_file: String,
}
#[derive(Debug, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

impl AppState {
    pub async fn get_registries(&self) -> Result<Vec<Registry>> {
        let registries = sqlx::query_as(r#"SELECT * FROM registry"#)
            .fetch_all(&self.pool)
            .await?;
        Ok(registries)
    }

    pub async fn get_registry(&self, id: i64) -> Result<Option<Registry>> {
        let registry = sqlx::query_as(r#"SELECT * FROM registry WHERE id = $1"#)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(registry)
    }

    pub async fn create_registry(&self, registry: &Registry) -> Result<Registry> {
        let registry = sqlx::query_as(
            r#"INSERT INTO registry (name, host, port, username, password, cert_file, key_file, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *"#,
        )
        .bind(&registry.name)
        .bind(&registry.host)
        .bind(registry.port)
        .bind(&registry.username)
        .bind(&registry.password)
        .bind(&registry.cert_file)
        .bind(&registry.key_file)
        .bind(&registry.status)
        .bind(registry.created_at)
        .bind(registry.updated_at)
        .fetch_one(&self.pool)
        .await?;
        Ok(registry)
    }

    pub async fn update_status(&self, id: i64, status: &str) -> Result<()> {
        sqlx::query(r#"UPDATE registry SET status = $1 WHERE id = $2"#)
            .bind(status)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn delete_registry(&self, id: i64) -> Result<()> {
        sqlx::query(r#"DELETE FROM registry WHERE id = $1"#)
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    // 根据名词获取仓库
    pub async fn get_registry_by_name(&self, name: &str) -> Result<Option<Registry>> {
        let registry = sqlx::query_as(r#"SELECT * FROM registry WHERE name = $1"#)
            .bind(name)
            .fetch_optional(&self.pool)
            .await?;
        Ok(registry)
    }
}
