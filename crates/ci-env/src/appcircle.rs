use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://docs.appcircle.io/environment-variables/appcircle-specific-environment-variables/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("AC_GIT_TARGET_BRANCH"),
        base_revision: None,
        branch: var("AC_GIT_BRANCH"),
        env_prefix: Some("AC_".into()),
        head_revision: None,
        id: var("AC_WORKFLOW_ID"),
        provider: CiProvider::Appcircle,
        request_id: opt_var("AC_PULL_NUMBER"),
        request_url: None,
        revision: var("AC_GIT_COMMIT"),
        url: None,
    }
}
