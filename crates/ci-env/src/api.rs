use serde::{Deserialize, Serialize};
use std::env;

/// List of supported CI providers.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum CiProvider {
    Agola,
    AppCenter,
    Appcircle,
    AppVeyor,
    AwsAmplify,
    AwsCodebuild,
    Azure,
    Bamboo,
    Bitbucket,
    Bitrise,
    Buddy,
    Buildkite,
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
    TeamCity,
    TravisCI,
    Vela,
    Vercel,
    Woodpecker,
    XcodeCloud,
    XcodeServer,
    #[default]
    Unknown,
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
