use actix_web::{middleware, web, App, HttpServer};
use std::env;

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();

    // Get the root directory from the environment variable or use the default
    let web_root = env::var("WEB_ROOT").unwrap_or_else(|_| "".to_string());

    println!("Starting server with WEB_ROOT: {}", web_root);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(web_root.clone())) // Share the root directory
            .wrap(middleware::Logger::default()) // Enable logging
            .service(handlers::directory::list_directory) // Directory listing
            .service(handlers::file::serve_file) // File handler
            .service(handlers::default::unknown_file) // Default handler
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
