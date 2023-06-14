/// The possible runtime environment for our application.
pub trait Environment {
    fn default_prefix() -> &'static str {
        "app"
    }

    fn default_separator() -> &'static str {
        "__"
    }

    fn default_configuration_dir() -> &'static str {
        "configuration"
    }

    fn default_environment() -> &'static str {
        "dev"
    }

    fn default_environment_detector() -> &'static str {
        "APP_ENVIRONMENT"
    }
}

pub fn parse_configuration<'a, T: Environment>() -> Result<T, config::ConfigError>
where
    T: serde::Deserialize<'a>,
{
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join(T::default_configuration_dir());

    let environment = std::env::var(T::default_environment_detector())
        .unwrap_or(T::default_environment().to_lowercase());

    log::info!(
        "{} = {}",
        T::default_environment_detector(),
        environment.as_str()
    );
    let settings = config::Config::builder()
        // Read the "default" configuration file
        .add_source(config::File::from(configuration_directory.join("base")).required(true))
        // Layer on the environment-specific values.
        .add_source(
            config::File::from(configuration_directory.join(environment.as_str())).required(true),
        )
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP__APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix(T::default_prefix()).separator(T::default_separator()),
        )
        .build()?;
    settings.try_deserialize()
}
