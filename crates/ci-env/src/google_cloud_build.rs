use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://cloud.google.com/build/docs/configuring-builds/substitute-variable-values
// Cloud Build only provides these values as substitutions, not environment
// variables. They are only available here when the build config maps them
// through `env` or enables `automapSubstitutions`.
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("_BASE_BRANCH"),
        base_revision: None,
        branch: opt_var("_HEAD_BRANCH")
            .or_else(|| opt_var("BRANCH_NAME"))
            .unwrap_or_default(),
        env_prefix: None,
        head_revision: None,
        id: var("BUILD_ID"),
        provider: CiProvider::GoogleCloudBuild,
        request_id: opt_var("_PR_NUMBER"),
        request_url: opt_var("_HEAD_REPO_URL"),
        revision: opt_var("COMMIT_SHA")
            .or_else(|| opt_var("REVISION_ID"))
            .unwrap_or_default(),
        url: None,
    }
}
