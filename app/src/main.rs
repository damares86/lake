use framework::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    println!("🔌 Starting db connection and server");

    if let Err(e) = start_server().await {
        eprintln!("❌ Error on server start: {:?}", e);
    }

    Ok(())
}