use anyhow::{Context, Result};
use log::info;
use serde_json::Value;
use sqlx::pool::PoolOptions;
use sqlx::types::Json;
use sqlx::PgPool;

use crate::cloud_providers::aws::SecretsClient;
use crate::config_manager::Config;
use crate::types::aws::secrets::DatabaseAuth;
use crate::types::event::Event;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

pub struct AuroraClient {
    pool: PgPool,
}

impl AuroraClient {
    pub async fn new(config: &Config, pool_size: Option<u32>) -> Self {
        let secrets_client = SecretsClient::new(config.aws_init_type.clone()).await;

        // NOTE: conditional added to fix integrations tests with docker mostly
        let db_secrets = if std::env::var("USE_LOCAL_CREDENTIALS").is_ok() {
            let username =
                std::env::var("DATABASE_USER").unwrap_or_else(|_| "postgres".to_string());
            let password =
                std::env::var("DATABASE_PASSWORD").unwrap_or_else(|_| "password".to_string());

            DatabaseAuth { username, password }
        } else {
            secrets_client
                .get_secrets(&config.database_secrets_arn)
                .await
                .expect("Failed to get secrets")
        };

        // encode password to escape special chars that would break url
        let encoded_password =
            utf8_percent_encode(&db_secrets.password, NON_ALPHANUMERIC).to_string();

        let url = format!(
            "postgres://{}:{}@{}/{}",
            db_secrets.username, encoded_password, config.database_host, config.database_name
        );

        // Use PgPoolOptions to set max_size
        let pool = PoolOptions::new()
            .max_connections(pool_size.unwrap_or(100))
            .connect(&url)
            .await
            .expect("Failed establish connection");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to migrate the database");

        info!("Successfully created connection pool");

        AuroraClient { pool }
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn insert_row(&self, job_id: &str, data: Json<Value>) -> Result<()> {
        let query = "INSERT INTO batch_jobs_logs (data, job_id) VALUES ($1, $2)";

        info!("Inserting row with job_id: {}", job_id);

        sqlx::query(query)
            .bind(data)
            .bind(job_id)
            .execute(&self.pool)
            .await
            .context("Failed to insert row")?;

        info!("Successfully inserted row with job_id: {}", job_id);

        Ok(())
    }

    pub async fn batch_insert_events(
        &self,
        job_id: &str,
        data: impl IntoIterator<Item = &Event>,
    ) -> Result<()> {
        let query = "INSERT INTO batch_jobs_logs (data, job_id) VALUES ($1, $2)";

        info!("Inserting row with job_id: {}", job_id);


        let mut transaction = self
            .get_pool()
            .begin()
            .await
            .context("Failed to begin transaction")?;

        let mut rows_affected = 0;

        for event in data {
            let json_data = Json(serde_json::to_value(event)?); // Convert the event to JSON
            println!("Inserting row with job_id: {}, data: {:#?}", job_id, json_data.to_string());
            rows_affected += sqlx::query(query)
                .bind(json_data)
                .bind(job_id)
                .execute(&mut *transaction) // Use the transaction directly
                .await
                .context("Failed to insert event into database")?
                .rows_affected();
        }
        // Commit the transaction
        transaction
            .commit()
            .await
            .context("Failed to commit transaction")?;

        info!("Successfully inserted {rows_affected} rows with job_id: {job_id}");

        Ok(())
    }

    /// closes the connection pool
    pub async fn close(&self) -> Result<()> {
        self.pool.close().await;
        info!("Successfully closed connection pool");
        Ok(())
    }
}
