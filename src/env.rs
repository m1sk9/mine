#[derive(serde::Deserialize)]
pub struct Env {
    pub discord_api_token: String,
    pub discord_guild_id: u64,
    pub server_name: String,
    pub server_ip: String,
    pub server_port: Option<String>,
}

pub fn load_envs() -> &'static Env {
    static ENV: std::sync::OnceLock<Env> = std::sync::OnceLock::new();
    ENV.get_or_init(|| envy::from_env::<Env>().expect("Failed to load envs"))
}
