use std::{env, sync::OnceLock};
use dotenvy::dotenv;
use crate::error::Error;

pub fn app_config() -> &'static AppConfig {
    static INSTANCE: OnceLock<AppConfig> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        AppConfig::load_from_env().unwrap_or_else(|ex| {
            panic!("FATAL ERROR - {ex:?}")
        })
    })
}

pub struct AppConfig {
    host: String,
    port: u16,
    db_url: String,
    log_filter: String,
    log_to_file: bool, 
    log_directory: String,
    log_prefix: String,
    writer_channel_buffer_size: usize,
    allow_origins: Vec<String>,
    tls_pem_folder_name: String,
    tls_cert_name: String,
    tls_key_name: String
}

impl AppConfig {
    fn load_from_env() -> Result<AppConfig, Error> {
        dotenv().ok();
        Ok(AppConfig {
            host: get_env("BIND_ADDR")?,
            port: get_env("BIND_PORT")?.parse().unwrap(),
            db_url: get_env("DATABASE_URL")?,
            log_filter: get_env("RUST_LOG")?,
            log_to_file: (|| { get_env("LOG_TO_FILE").unwrap() != "0" })(),
            log_directory: get_env("LOG_DIRECTORY")?,
            log_prefix: get_env("LOG_PREFIX")?,
            writer_channel_buffer_size: get_env("WRITER_CHANNEL_BUFFER_SIZE")?.parse().unwrap(),
            allow_origins: (|| {get_env("ALLOW_ORIGINS").unwrap().split(",").map(|o| { o.to_string() }).collect::<Vec<String>>() } )(),
            tls_pem_folder_name: get_env("TLS_PEM_FILES_FOLDER")?,
            tls_cert_name: get_env("TLS_CERT_FILE_NAME")?,
            tls_key_name: get_env("TLS_KEY_FILE_NAME")?
        })
    }

    pub fn get_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    pub fn get_db_url(&self) -> &str {
        self.db_url.as_ref()
    }

    pub fn get_log_filter(&self) -> &str {
        self.log_filter.as_ref()
    }

    pub fn log_to_file(&self) -> bool {
        self.log_to_file
    }

    pub fn get_log_directory(&self) -> &str {
        self.log_directory.as_ref()
    }

    pub fn get_log_prefix(&self) -> &str {
        self.log_prefix.as_ref()
    }

    pub fn get_writer_channel_buffer_size(&self) -> usize {
        self.writer_channel_buffer_size
    }

    pub fn get_allow_origins(&self) -> &Vec<String> {
        self.allow_origins.as_ref()
    }

    pub fn get_tls_pem_folder_name(&self) -> &str {
        self.tls_pem_folder_name.as_ref()
    }

    pub fn get_tls_cert_name(&self) -> &str {
        self.tls_cert_name.as_ref()
    }

    pub fn get_tls_key_name(&self) -> &str {
        self.tls_key_name.as_ref()
    }
}

fn get_env(name: &'static str) -> Result<String, Error> {
    env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}