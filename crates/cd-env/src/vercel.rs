use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://vercel.com/docs/concepts/projects/environment-variables#system-environment-variables
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: opt_var("VERCEL_GIT_COMMIT_REF"),
        env_prefix: Some("VERCEL_".into()),
        provider: CdProvider::Vercel,
        revision: var("VERCEL_GIT_COMMIT_SHA"),
        service_id: opt_var("VERCEL_PROJECT_ID"),
    }
}
