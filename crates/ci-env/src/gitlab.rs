use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://docs.gitlab.com/ee/ci/variables/predefined_variables.html
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("CI_MERGE_REQUEST_TARGET_BRANCH_NAME")
            .or_else(|| opt_var("CI_EXTERNAL_PULL_REQUEST_TARGET_BRANCH_NAME")),
        base_revision: opt_var("CI_MERGE_REQUEST_DIFF_BASE_SHA")
            .or_else(|| opt_var("CI_EXTERNAL_PULL_REQUEST_TARGET_BRANCH_SHA")),
        branch: opt_var("CI_MERGE_REQUEST_SOURCE_BRANCH_NAME")
            .or_else(|| opt_var("CI_EXTERNAL_PULL_REQUEST_SOURCE_BRANCH_NAME"))
            .or_else(|| opt_var("CI_COMMIT_BRANCH"))
            .unwrap_or_default(),
        env_prefix: Some("CI_".into()),
        // only populated in merged results and merge train pipelines,
        // and is an empty string otherwise
        head_revision: opt_var("CI_MERGE_REQUEST_SOURCE_BRANCH_SHA")
            .or_else(|| opt_var("CI_EXTERNAL_PULL_REQUEST_SOURCE_BRANCH_SHA")),
        id: var("CI_PIPELINE_ID"),
        provider: CiProvider::Gitlab,
        // the IID is the per-project merge request number, which matches
        // the pull request number of other providers
        request_id: opt_var("CI_MERGE_REQUEST_IID")
            .or_else(|| opt_var("CI_EXTERNAL_PULL_REQUEST_IID")),
        request_url: None,
        revision: var("CI_COMMIT_SHA"),
        url: opt_var("CI_PIPELINE_URL"),
    }
}
