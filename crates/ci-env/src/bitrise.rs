use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://devcenter.bitrise.io/en/references/available-environment-variables.html
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("BITRISEIO_GIT_BRANCH_DEST"),
        base_revision: None,
        // `BITRISEIO_PULL_REQUEST_HEAD_BRANCH` is a special ref (`pull/N/head`),
        // so prefer the real branch name
        branch: opt_var("BITRISE_GIT_BRANCH")
            .or_else(|| opt_var("BITRISEIO_PULL_REQUEST_HEAD_BRANCH"))
            .unwrap_or_default(),
        env_prefix: Some("BITRISE".into()),
        head_revision: None,
        // `BITRISEIO_PIPELINE_*` are only set for builds that are part of
        // a pipeline, so fall back to the standalone build values
        id: opt_var("BITRISEIO_PIPELINE_ID")
            .or_else(|| opt_var("BITRISE_BUILD_SLUG"))
            .unwrap_or_default(),
        provider: CiProvider::Bitrise,
        request_id: opt_var("BITRISE_PULL_REQUEST"),
        request_url: None,
        revision: var("BITRISE_GIT_COMMIT"),
        url: opt_var("BITRISEIO_PIPELINE_BUILD_URL").or_else(|| opt_var("BITRISE_BUILD_URL")),
    }
}
