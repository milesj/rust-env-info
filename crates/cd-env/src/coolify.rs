use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://coolify.io/docs/knowledge-base/environment-variables
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: opt_var("COOLIFY_BRANCH"),
        env_prefix: Some("COOLIFY_".into()),
        provider: CdProvider::Coolify,
        // excluded from image builds by default; only available during the
        // build when "Include Source Commit in Build" is enabled
        revision: var("SOURCE_COMMIT"),
        service_id: opt_var("COOLIFY_RESOURCE_UUID"),
    }
}
