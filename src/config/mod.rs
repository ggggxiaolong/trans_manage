pub mod app_config;

use app_config::ApplicationConfig;

lazy_static! {
    pub static ref CONFIG: ApplicationConfig = ApplicationConfig::default();
}
