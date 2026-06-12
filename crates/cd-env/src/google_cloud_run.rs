use crate::api::{opt_var, CdEnvironment, CdProvider};

// https://cloud.google.com/run/docs/container-contract
// Jobs don't receive the `K_*` variables, they get `CLOUD_RUN_*` instead
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        env_prefix: Some("K_".into()),
        provider: CdProvider::GoogleCloudRun,
        revision: opt_var("K_REVISION")
            .or_else(|| opt_var("CLOUD_RUN_EXECUTION"))
            .unwrap_or_default(),
        service_id: opt_var("K_SERVICE").or_else(|| opt_var("CLOUD_RUN_JOB")),
    }
}
