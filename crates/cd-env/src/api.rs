use serde::{Deserialize, Serialize};
use std::{env, fmt};

/// List of supported CD providers.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CdProvider {
    AwsCodedeploy,
    Coolify,
    DenoDeploy,
    #[serde(rename = "digitalocean-app-platform")]
    DigitalOceanAppPlatform,
    Fly,
    #[serde(rename = "gocd")]
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

impl fmt::Display for CdProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = match self {
            CdProvider::AwsCodedeploy => "aws-codedeploy",
            CdProvider::Coolify => "coolify",
            CdProvider::DenoDeploy => "deno-deploy",
            CdProvider::DigitalOceanAppPlatform => "digitalocean-app-platform",
            CdProvider::Fly => "fly",
            CdProvider::GoCD => "gocd",
            CdProvider::GoogleAppEngine => "google-app-engine",
            CdProvider::GoogleCloudRun => "google-cloud-run",
            CdProvider::Heroku => "heroku",
            CdProvider::Netlify => "netlify",
            CdProvider::Octopus => "octopus",
            CdProvider::Railway => "railway",
            CdProvider::Release => "release",
            CdProvider::Render => "render",
            CdProvider::Seed => "seed",
            CdProvider::Vercel => "vercel",
            CdProvider::Unknown => "unknown",
        };

        f.write_str(id)
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::de::value::{Error as DeError, StrDeserializer};
    use serde::de::IntoDeserializer;

    fn from_id(id: &str) -> Result<CdProvider, DeError> {
        let de: StrDeserializer<DeError> = id.into_deserializer();
        CdProvider::deserialize(de)
    }

    #[test]
    fn display_uses_kebab_case_ids() {
        assert_eq!(CdProvider::AwsCodedeploy.to_string(), "aws-codedeploy");
        assert_eq!(CdProvider::GoCD.to_string(), "gocd");
        assert_eq!(
            CdProvider::DigitalOceanAppPlatform.to_string(),
            "digitalocean-app-platform"
        );
        assert_eq!(CdProvider::Unknown.to_string(), "unknown");
    }

    #[test]
    fn serde_round_trips_display_ids() {
        // The serde wire format must match `Display` exactly, including the
        // hand-tuned overrides where serde's mechanical kebab-case differs.
        for provider in [
            CdProvider::AwsCodedeploy,
            CdProvider::DigitalOceanAppPlatform,
            CdProvider::GoCD,
            CdProvider::Unknown,
        ] {
            let id = provider.to_string();
            assert_eq!(from_id(&id).unwrap(), provider, "id {id:?}");
        }
    }
}
