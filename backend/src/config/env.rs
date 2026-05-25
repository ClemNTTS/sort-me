use dotenvy::dotenv;

pub fn load_env() {
    dotenv().ok();
}

pub fn get_var(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("{} must be set", key))
}

pub fn get_var_or(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

pub fn get_app_port() -> u16 {
    get_var_or("APP_PORT", "8080").parse().unwrap_or(8080)
}

pub fn get_app_env() -> String {
    get_var_or("APP_ENV", "development")
}

pub fn get_database_url() -> String {
    get_var("DATABASE_URL")
}

pub fn get_rust_log() -> String {
    get_var_or("RUST_LOG", "backend_sortme=debug,tower_http=debug")
}

pub fn get_imap_host() -> String {
    get_var_or("IMAP_HOST", "")
}

pub fn get_imap_port() -> u16 {
    get_var_or("IMAP_PORT", "993").parse().unwrap_or(993)
}

pub fn get_imap_username() -> String {
    get_var_or("IMAP_USERNAME", "")
}

pub fn get_imap_password() -> String {
    get_var_or("IMAP_PASSWORD", "")
}

pub fn get_imap_use_tls() -> bool {
    get_var_or("IMAP_USE_TLS", "true").parse().unwrap_or(true)
}

pub fn get_llm_provider() -> String {
    get_var_or("LLM_PROVIDER", "ollama")
}

pub fn get_ollama_base_url() -> String {
    get_var_or("OLLAMA_BASE_URL", "http://ollama:11434")
}

pub fn get_ollama_model() -> String {
    get_var_or("OLLAMA_MODEL", "")
}

pub fn get_mistral_api_key() -> String {
    get_var_or("MISTRAL_API_KEY", "")
}

pub fn get_mistral_model() -> String {
    get_var_or("MISTRAL_MODEL", "")
}

pub fn get_gemini_api_key() -> String {
    get_var_or("GEMINI_API_KEY", "")
}

pub fn get_gemini_model() -> String {
    get_var_or("GEMINI_MODEL", "")
}

pub fn get_scan_interval() -> String {
    get_var_or("SCAN_INTERVAL", "manual")
}
