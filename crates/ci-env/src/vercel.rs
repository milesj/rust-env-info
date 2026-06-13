use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://vercel.com/docs/environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: var("VERCEL_GIT_COMMIT_REF"),
        env_prefix: Some("VERCEL_".into()),
        head_revision: None,
        id: var("VERCEL_DEPLOYMENT_ID"),
        provider: CiProvider::Vercel,
        request_id: opt_var("VERCEL_GIT_PULL_REQUEST_ID"),
        request_url: None,
        revision: var("VERCEL_GIT_COMMIT_SHA"),
        url: None,
    }
}
