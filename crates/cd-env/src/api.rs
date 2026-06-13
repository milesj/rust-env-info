use serde::{Deserialize, Serialize};
use std::env;

/// List of supported CD providers.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum CdProvider {
    AwsCodedeploy,
    Coolify,
    DenoDeploy,
    DigitalOceanAppPlatform,
    Fly,
    GoCD,
    GoogleAppEngine,
    GoogleCloudRun,
    Heroku,
    Netlify,
    Octopus,
    Railway,
    Release,
    Render,
    Seed,
    Vercel,
    #[default]
    Unknown,
}

// Other fields to maybe track: environment, url, deploy ID
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CdEnvironment {
    /// Source branch that was deployed.
    pub branch: Option<String>,

    /// Prefix that all environment variables use.
    pub env_prefix: Option<String>,

    /// Name of the provider.
    pub provider: CdProvider,

    /// Revision (commit, sha, etc) that triggered the deploy.
    pub revision: String,

    /// Unique ID of the deployed service.
    pub service_id: Option<String>,
}

pub fn var(key: &str) -> String {
    env::var(key).unwrap_or_default()
}

pub fn opt_var(key: &str) -> Option<String> {
    match env::var(key) {
        Ok(value) => {
            if value == "false" || value.is_empty() {
                None
            } else {
                Some(value)
            }
        }
        Err(_) => None,
    }
}
