use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://circleci.com/docs/variables/#built-in-environment-variables
pub fn create_environment() -> CiEnvironment {
    let request_url = opt_var("CIRCLE_PULL_REQUEST");

    // `CIRCLE_PR_NUMBER` is only set for forked pull requests, so extract
    // the number from the pull request URL for same-repository ones
    let request_id = opt_var("CIRCLE_PR_NUMBER").or_else(|| {
        request_url
            .as_deref()
            .and_then(|url| url.rsplit('/').next())
            .filter(|id| !id.is_empty() && id.bytes().all(|byte| byte.is_ascii_digit()))
            .map(|id| id.to_owned())
    });

    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: var("CIRCLE_BRANCH"),
        env_prefix: Some("CIRCLE_".into()),
        head_revision: None,
        id: var("CIRCLE_WORKFLOW_ID"),
        provider: CiProvider::CircleCI,
        request_id,
        request_url,
        revision: var("CIRCLE_SHA1"),
        url: opt_var("CIRCLE_BUILD_URL"),
    }
}
