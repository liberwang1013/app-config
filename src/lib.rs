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

    fn current_environment() -> &'static str {
        let s = std::env::var(Self::default_environment_detector())
            .unwrap_or(Self::default_environment().to_lowercase());
        Box::leak(s.into_boxed_str())
    }
}

pub fn parse_configuration<'a, T: Environment>() -> Result<T, config::ConfigError>
where
    T: serde::Deserialize<'a>,
{
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join(T::default_configuration_dir());

    let environment = T::current_environment();

    log::info!("{} = {}", T::default_environment_detector(), environment);
    let settings = config::Config::builder()
        // Read the "default" configuration file
        .add_source(config::File::from(configuration_directory.join("base")).required(true))
        // Layer on the environment-specific values.
        .add_source(config::File::from(configuration_directory.join(environment)).required(false))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP__APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(
            config::Environment::with_prefix(T::default_prefix()).separator(T::default_separator()),
        )
        .build()?;
    settings.try_deserialize()
}

#[cfg(test)]
mod tests {

    use crate::{parse_configuration, Environment};

    #[derive(serde::Deserialize)]
    pub struct ApplicationSetting {
        pub address: String,
        pub port: i32,
    }

    #[derive(serde::Deserialize)]
    struct Setting {
        pub stage: String,
        pub application: ApplicationSetting,
    }

    impl Environment for Setting {}

    #[test]
    fn test_default_prefix() {
        assert_eq!("app", Setting::default_prefix());
    }

    #[test]
    fn test_current_environment() {
        std::env::set_var("APP_ENVIRONMENT", "dev");
        assert_eq!("dev", Setting::current_environment());
    }

    #[test]
    fn test_parse_configuration() {
        std::env::set_var("APP_ENVIRONMENT", "dev");
        let setting = parse_configuration::<Setting>();
        assert_eq!(true, setting.is_ok())
    }

    #[test]
    fn test_parse_configuration_stage() {
        std::env::set_var("APP_ENVIRONMENT", "dev");
        std::env::set_var("APP__STAGE", "abc");
        let setting = parse_configuration::<Setting>().unwrap();
        assert_eq!("abc", setting.stage);
    }

    #[test]
    fn test_parse_configuration_application_port() {
        std::env::set_var("APP__APPLICATION__PORT", "80");
        let setting = parse_configuration::<Setting>().unwrap();
        assert_eq!(80, setting.application.port);
    }
}
