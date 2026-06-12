use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://woodpecker-ci.org/docs/usage/environment
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("CI_COMMIT_TARGET_BRANCH"),
        base_revision: None,
        branch: opt_var("CI_COMMIT_SOURCE_BRANCH")
            .or_else(|| opt_var("CI_COMMIT_BRANCH"))
            .unwrap_or_default(),
        env_prefix: Some("CI_".into()),
        head_revision: None,
        // `CI_BUILD_*` were renamed to `CI_PIPELINE_*` in Woodpecker v1,
        // but are kept as fallbacks for older servers
        id: opt_var("CI_PIPELINE_NUMBER")
            .or_else(|| opt_var("CI_BUILD_NUMBER"))
            .unwrap_or_default(),
        provider: CiProvider::Woodpecker,
        request_id: opt_var("CI_COMMIT_PULL_REQUEST"),
        request_url: None,
        revision: var("CI_COMMIT_SHA"),
        url: opt_var("CI_PIPELINE_URL").or_else(|| opt_var("CI_BUILD_LINK")),
    }
}
