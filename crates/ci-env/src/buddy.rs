use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://buddy.works/docs/pipelines/environment-variables
// `BUDDY_EXECUTION_*` variables are deprecated aliases of `BUDDY_RUN_*`,
// kept as fallbacks for older Buddy instances
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("BUDDY_RUN_PR_BASE_BRANCH")
            .or_else(|| opt_var("BUDDY_EXECUTION_PULL_REQUEST_BASE_BRANCH")),
        base_revision: None,
        branch: opt_var("BUDDY_RUN_PR_HEAD_BRANCH")
            .or_else(|| opt_var("BUDDY_EXECUTION_PULL_REQUEST_HEAD_BRANCH"))
            .or_else(|| opt_var("BUDDY_RUN_BRANCH"))
            .or_else(|| opt_var("BUDDY_EXECUTION_BRANCH"))
            .unwrap_or_default(),
        env_prefix: Some("BUDDY_".into()),
        head_revision: None,
        id: var("BUDDY_PIPELINE_ID"),
        provider: CiProvider::Buddy,
        request_id: opt_var("BUDDY_RUN_PR_NO")
            .or_else(|| opt_var("BUDDY_EXECUTION_PULL_REQUEST_NO"))
            .or_else(|| opt_var("BUDDY_RUN_PR_ID"))
            .or_else(|| opt_var("BUDDY_EXECUTION_PULL_REQUEST_ID")),
        request_url: None,
        revision: opt_var("BUDDY_RUN_COMMIT")
            .or_else(|| opt_var("BUDDY_EXECUTION_REVISION"))
            .unwrap_or_default(),
        url: opt_var("BUDDY_PIPELINE_URL"),
    }
}
