mod docker;
mod settings;
mod registry;

pub use docker::DockerClient;
use serde::{Deserialize, Serialize};
pub use settings::{get_settings, AppSettings};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum AppEvent {
    InstallDocker(String),
}
