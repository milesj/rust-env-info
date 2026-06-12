use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://docs.digitalocean.com/products/app-platform/how-to/use-environment-variables/
// App Platform doesn't inject these automatically; they're bindable variables
// ($APP_ID, ${_self.COMMIT_HASH}, etc) the user must map in their app spec
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        env_prefix: None,
        provider: CdProvider::DigitalOceanAppPlatform,
        revision: var("COMMIT_HASH"),
        service_id: opt_var("APP_ID"),
    }
}
