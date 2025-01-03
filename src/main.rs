use std::env;
use axum_server::tls_rustls::RustlsConfig;
use receipt_repository_api::services::v1::commands::command_service::CommandService;
use receipt_repository_api::share_state::HandlerState;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use receipt_repository_api::configuration::app_config;
use receipt_repository_api::repository::DbRepository;
use receipt_repository_api::router::AppRouter;
use receipt_repository_api::application::Application;

#[tokio::main]
async fn main() {
    let config = app_config();
    let (non_blocking_writer, _guard);
    if config.log_to_file() {
        let file_appender = tracing_appender::rolling::hourly(config.get_log_directory(), config.get_log_prefix());
        (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);
    }
    else {
        (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    }

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::new(config.get_log_filter())
        )
        .with(
            tracing_subscriber::fmt::layer().with_writer(
                non_blocking_writer
            )
        )
        .init();
    
    let repository = DbRepository::new(config.get_db_url());
    let sender = CommandService::run(repository.clone(), config.get_writer_channel_buffer_size());
    let handler_state = HandlerState::new(repository, sender);
    let router = AppRouter::new(handler_state);

    let cur_path = env::current_dir().unwrap();
    let tls_config = RustlsConfig::from_pem_file(
        cur_path.join(config.get_tls_pem_folder_name()).join(config.get_tls_cert_name()), 
        cur_path.join(config.get_tls_pem_folder_name()).join(config.get_tls_key_name())
    ).await.unwrap();

    let app = Application::new(router, config.get_address(), tls_config);
    app.run(config.get_allow_origins()).await;
}
