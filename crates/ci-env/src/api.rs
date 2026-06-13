use serde::{Deserialize, Serialize};
use std::{env, fmt};

/// List of supported CI providers.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum CiProvider {
    Agola,
    AppCenter,
    Appcircle,
    #[serde(rename = "appveyor")]
    AppVeyor,
    AwsAmplify,
    AwsCodebuild,
    Azure,
    Bamboo,
    Bitbucket,
    Bitrise,
    Buddy,
    Buildkite,
    #[serde(rename = "circleci")]
    CircleCI,
    Cirrus,
    CloudflarePages,
    Codefresh,
    Codemagic,
    Codeship,
    Drone,
    Eas,
    ForgejoActions,
    GiteaActions,
    GithubActions,
    Gitlab,
    GoogleCloudBuild,
    Harness,
    Heroku,
    Jenkins,
    JenkinsX,
    JetbrainsSpace,
    Netlify,
    Screwdriver,
    Scrutinizer,
    Semaphore,
    Sourcehut,
    #[serde(rename = "teamcity")]
    TeamCity,
    #[serde(rename = "travis-ci")]
    TravisCI,
    Vela,
    Vercel,
    Woodpecker,
    XcodeCloud,
    XcodeServer,
    #[default]
    Unknown,
}

impl fmt::Display for CiProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = match self {
            CiProvider::Agola => "agola",
            CiProvider::AppCenter => "app-center",
            CiProvider::Appcircle => "appcircle",
            CiProvider::AppVeyor => "appveyor",
            CiProvider::AwsAmplify => "aws-amplify",
            CiProvider::AwsCodebuild => "aws-codebuild",
            CiProvider::Azure => "azure",
            CiProvider::Bamboo => "bamboo",
            CiProvider::Bitbucket => "bitbucket",
            CiProvider::Bitrise => "bitrise",
            CiProvider::Buddy => "buddy",
            CiProvider::Buildkite => "buildkite",
            CiProvider::CircleCI => "circleci",
            CiProvider::Cirrus => "cirrus",
            CiProvider::CloudflarePages => "cloudflare-pages",
            CiProvider::Codefresh => "codefresh",
            CiProvider::Codemagic => "codemagic",
            CiProvider::Codeship => "codeship",
            CiProvider::Drone => "drone",
            CiProvider::Eas => "eas",
            CiProvider::ForgejoActions => "forgejo-actions",
            CiProvider::GiteaActions => "gitea-actions",
            CiProvider::GithubActions => "github-actions",
            CiProvider::Gitlab => "gitlab",
            CiProvider::GoogleCloudBuild => "google-cloud-build",
            CiProvider::Harness => "harness",
            CiProvider::Heroku => "heroku",
            CiProvider::Jenkins => "jenkins",
            CiProvider::JenkinsX => "jenkins-x",
            CiProvider::JetbrainsSpace => "jetbrains-space",
            CiProvider::Netlify => "netlify",
            CiProvider::Screwdriver => "screwdriver",
            CiProvider::Scrutinizer => "scrutinizer",
            CiProvider::Semaphore => "semaphore",
            CiProvider::Sourcehut => "sourcehut",
            CiProvider::TeamCity => "teamcity",
            CiProvider::TravisCI => "travis-ci",
            CiProvider::Vela => "vela",
            CiProvider::Vercel => "vercel",
            CiProvider::Woodpecker => "woodpecker",
            CiProvider::XcodeCloud => "xcode-cloud",
            CiProvider::XcodeServer => "xcode-server",
            CiProvider::Unknown => "unknown",
        };

        f.write_str(id)
    }
}

pub struct CiOutput {
    /// Denotes the closing of a log group.
    pub close_log_group: &'static str,

    /// Denotes the opening of a log group.
    pub open_log_group: &'static str,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CiEnvironment {
    /// Target branch of the pull/merge request.
    pub base_branch: Option<String>,

    /// Target revision of the pull/merge request.
    pub base_revision: Option<String>,

    /// Source branch that triggered the pipeline.
    pub branch: String,

    /// Prefix that all environment variables use.
    pub env_prefix: Option<String>,

    /// Source revision of the pull/merge request.
    pub head_revision: Option<String>,

    /// Unique ID of the current pipeline.
    pub id: String,

    /// Name of the provider.
    pub provider: CiProvider,

    /// ID of an associated pull/merge request.
    pub request_id: Option<String>,

    /// Link to the pull/merge request.
    pub request_url: Option<String>,

    /// Revision (commit, sha, etc) that triggered the pipeline.
    pub revision: String,

    /// Link to the pipeline.
    pub url: Option<String>,
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

    fn from_id(id: &str) -> Result<CiProvider, DeError> {
        let de: StrDeserializer<DeError> = id.into_deserializer();
        CiProvider::deserialize(de)
    }

    #[test]
    fn display_uses_kebab_case_ids() {
        assert_eq!(CiProvider::GithubActions.to_string(), "github-actions");
        assert_eq!(CiProvider::AwsCodebuild.to_string(), "aws-codebuild");
        assert_eq!(CiProvider::CircleCI.to_string(), "circleci");
        assert_eq!(CiProvider::Unknown.to_string(), "unknown");
    }

    #[test]
    fn serde_round_trips_display_ids() {
        // The serde wire format must match `Display` exactly, including the
        // hand-tuned overrides where serde's mechanical kebab-case differs.
        for provider in [
            CiProvider::AppVeyor,
            CiProvider::CircleCI,
            CiProvider::GithubActions,
            CiProvider::TeamCity,
            CiProvider::TravisCI,
            CiProvider::Unknown,
        ] {
            let id = provider.to_string();
            assert_eq!(from_id(&id).unwrap(), provider, "id {id:?}");
        }
    }
}
