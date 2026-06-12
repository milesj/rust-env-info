use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://www.jenkins.io/doc/book/pipeline/jenkinsfile/#using-environment-variables
// `CHANGE_*` variables are only set for multibranch pull request builds
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("CHANGE_TARGET"),
        base_revision: None,
        branch: opt_var("CHANGE_BRANCH")
            .or_else(|| opt_var("GIT_BRANCH"))
            .or_else(|| opt_var("BRANCH_NAME"))
            .unwrap_or_default(),
        env_prefix: None,
        head_revision: None,
        id: var("BUILD_NUMBER"),
        provider: CiProvider::Jenkins,
        request_id: opt_var("CHANGE_ID"),
        request_url: opt_var("CHANGE_URL"),
        revision: var("GIT_COMMIT"),
        url: opt_var("BUILD_URL"),
    }
}
