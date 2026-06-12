use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://docs.aws.amazon.com/codedeploy/latest/userguide/reference-appspec-file-structure-hooks.html
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: None,
        env_prefix: None,
        provider: CdProvider::AwsCodedeploy,
        // only set when the bundle comes from GitHub; S3 bundles
        // expose `BUNDLE_BUCKET`/`BUNDLE_KEY` instead of a commit
        revision: var("BUNDLE_COMMIT"),
        service_id: opt_var("DEPLOYMENT_GROUP_ID"),
    }
}
