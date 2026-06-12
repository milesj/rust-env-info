use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://jenkins-x.io/v3/develop/reference/variables/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("PULL_BASE_REF"),
        base_revision: opt_var("PULL_BASE_SHA"),
        branch: opt_var("PR_HEAD_REF")
            .or_else(|| opt_var("GIT_BRANCH"))
            .or_else(|| opt_var("BRANCH_NAME"))
            .unwrap_or_default(),
        env_prefix: None,
        head_revision: opt_var("PR_HEAD_SHA"),
        id: var("BUILD_ID"),
        provider: CiProvider::JenkinsX,
        request_id: opt_var("PULL_NUMBER"),
        request_url: None,
        // `PULL_PULL_SHA` is only set for pull requests, while `PULL_BASE_SHA`
        // is the SHA being built on branch/release pipelines
        revision: opt_var("PULL_PULL_SHA")
            .or_else(|| opt_var("PULL_BASE_SHA"))
            .unwrap_or_default(),
        url: opt_var("JENKINS_X_URL"),
    }
}
