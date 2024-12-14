use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod config;
mod errors;
mod handlers;
mod models;
mod services;
mod utils;
mod logger; // 引入日志模块

use config::Config;
// use handlers::pdf_handler::download_pdf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv().ok();

    // 初始化日志
    if let Err(e) = logger::init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
        std::process::exit(1);
    }

    log::info!("Starting Actix-Web server...");

    // 加载配置
    let config = Config::from_env().expect("Failed to load configuration");

    log::debug!("Configuration loaded: {:?}", config);

    // 确保下载目录存在
    // log::info!("Download directory is set to: {}", config.pdf_download_dir);

    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default()) // Actix-Web的请求日志中间件
            .data(config.clone())
            .route("/home", web::get().to(handlers::home))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
