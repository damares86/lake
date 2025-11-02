use tikv_client::{RawClient};
use config::Config;
use std::sync::Arc;
use anyhow::{Result, Context};

#[derive(Clone)]
pub struct Database {
    pub client: Arc<RawClient>,
}

impl Database {
    /// get endpoints from config and connect to TiKV
    pub async fn connect_from_config() -> Result<Self> {
        let settings = Config::builder()
            .add_source(config::File::with_name("config"))
            .add_source(config::Environment::with_prefix("APP"))
            .build()
            .context("Error on config loading")?;

        let endpoints: Vec<String> = settings
            .get::<Vec<String>>("tikv.endpoints")
            .context("Error trying to read tikv.endpoints")?;

        let client = RawClient::new(endpoints)
            .await
            .context("Error on TiKV connection")?;

        Ok(Self {
            client: Arc::new(client),
        })
    }

    /// insert a key/value data in TiKV cluster
    pub async fn insert(&self, key: &str, value: &str) -> Result<()> {
        self.client
            .put(key.to_owned(), value.to_owned())
            .await
            .context("Error during insert in TiKV")
    }
}


pub async fn connect_to_tikv(endpoints: Vec<String>) -> Result<Database> {
    let client = RawClient::new(endpoints)
        .await
        .context("Error on TiKV connection")?;

    Ok(Database {
        client: Arc::new(client),
    })
}


pub async fn insert_kv(db: &Database, key: &str, value: &str) -> Result<()> {
    db.insert(key, value).await
}

