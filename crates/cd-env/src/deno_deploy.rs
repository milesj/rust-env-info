use crate::api::{var, CdEnvironment, CdProvider};

// https://docs.deno.com/deploy/reference/env_vars_and_contexts/
// No git or service name variables are exposed; the deployment ID
// identifies the deployed version, like Cloud Run's `K_REVISION`
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        env_prefix: Some("DENO_".into()),
        provider: CdProvider::DenoDeploy,
        revision: var("DENO_DEPLOYMENT_ID"),
        service_id: None,
    }
}
