use actix_web::{web, App, HttpServer, Responder, HttpResponse, get};
use actix_cors::Cors;
use actix_web::http;
use actix_files::Files;
use crate::config::AppConfig;
use crate::db::{connect_to_tikv, insert_kv, Database};
use std::sync::Arc;
use std::path::Path;
use crate::frontend::get_dummy_data;

/// start server and connect to TiKV
pub async fn start_server() -> anyhow::Result<()> {
    
    // load config
    let settings = config::Config::builder()
        .add_source(config::File::with_name("Config"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;

    let app_config: AppConfig = settings.try_deserialize()?;

    // connect to TiKV using the endpoint from the config
    println!("🔌 Trying to connect to TiKV...");
    let db = Arc::new(connect_to_tikv(app_config.tikv.endpoints.clone()).await?);
    println!("✅ Connected to TiKV!");

    // check the path of the book
    let book_path = "./book/book";
    if !Path::new(book_path).exists() {
        eprintln!("⚠️ The guide was not found in `{}`. Try to execute `mdbook build book`!", book_path);
    }

    // Start HTTP server
    println!("🚀 Actix Server listening on http://{}:{}", app_config.host, app_config.port);
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![http::header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600),
            )
            .app_data(web::Data::new(db.clone()))
            .service(insert_test_data)
            .service(get_dummy_data) // endpoint frontend test
            .route("/{guide}", web::get().to(redirect_to_slash)) // redirect without slash
            .service(Files::new("/guide", book_path).index_file("index.html")) // serves the guide
    })
    .bind((app_config.host.as_str(), app_config.port))?
    .run()
    .await?;

    Ok(())
}

/// remove the '/' from url
pub async fn redirect_to_slash(path: web::Path<String>) -> HttpResponse {
    let mut location = path.into_inner();
    if !location.ends_with('/') {
        location.push('/');
    }

    HttpResponse::MovedPermanently()
        .append_header(("Location", format!("/{}", location)))
        .finish()
}

/// Endpoint GET /api/insert on TiKV
#[get("/api/insert")]
async fn insert_test_data(db: web::Data<Arc<Database>>) -> impl Responder {
    if let Err(e) = insert_kv(&db, "test_key", "test_value").await {
        eprintln!("❌ Error during insert: {:?}", e);
        return HttpResponse::InternalServerError().body("Insert failed");
    }

    HttpResponse::Ok().body("✅ Inserted test_key -> test_value")
}
