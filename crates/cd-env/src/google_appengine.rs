use crate::api::{opt_var, CdEnvironment, CdProvider};

// https://cloud.google.com/appengine/docs/standard/cloud-run-for-gae-customers
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        env_prefix: Some("GAE_".into()),
        provider: CdProvider::GoogleAppEngine,
        revision: opt_var("GAE_VERSION")
            .or_else(|| opt_var("GAE_DEPLOYMENT_ID"))
            .unwrap_or_default(),
        service_id: opt_var("GAE_SERVICE").or_else(|| opt_var("GAE_APPLICATION")),
    }
}
