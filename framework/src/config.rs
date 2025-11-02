use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub tikv: TikvConfig,
}

#[derive(Debug, Deserialize)]
pub struct TikvConfig {
    pub endpoints: Vec<String>,
}

pub fn init_tracing() {
    use tracing_subscriber::fmt::Subscriber;
    let subscriber = Subscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");
}
