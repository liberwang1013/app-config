/// The possible runtime environment for our application.
pub enum Environment {
    Local,
    Dev,
    Staging,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Dev => "dev",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "dev" => Ok(Self::Dev),
            "staging" => Ok(Self::Staging),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Use either `local`, `dev`, `staging` or `production`.",
                other
            )),
        }
    }
}

pub fn parse_configuration<'a, T>() -> Result<T, config::ConfigError>
where
    T: serde::Deserialize<'a>,
{
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT.");

    log::info!("APP_ENVIRONMENT = {}", environment.as_str());
    let settings = config::Config::builder()
        // Read the "default" configuration file
        .add_source(config::File::from(configuration_directory.join("base")).required(true))
        // Layer on the environment-specific values.
        .add_source(
            config::File::from(configuration_directory.join(environment.as_str())).required(true),
        )
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP__APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(config::Environment::with_prefix("app").separator("__"))
        .build()?;
    settings.try_deserialize()
}
