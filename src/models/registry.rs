use anyhow::Result;
use sqlx::FromRow;

use crate::web_server::AppState;

#[derive(Debug, FromRow)]
pub struct Registry {
    pub id: i64,
    pub name: String,
    pub port: i64,
    pub username: String,
    pub password: String,
    pub cert_file: String,
    pub key_file: String,
    pub status: String,
    pub created_at: i64,
    pub updated_at: i64,
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

    pub async fn create_registry(
        &self,
        name: &str,
        port: i32,
        username: &str,
        password: &str,
        cert_file: &str,
        key_file: &str,
    ) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO registry (name, port, username, password, cert_file, key_file)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
        )
        .bind(name)
        .bind(port)
        .bind(username)
        .bind(password)
        .bind(cert_file)
        .bind(key_file)
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
}
