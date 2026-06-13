use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://docs.aws.amazon.com/amplify/latest/userguide/environment-variables.html
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("AWS_PULL_REQUEST_DESTINATION_BRANCH"),
        base_revision: None,
        branch: opt_var("AWS_PULL_REQUEST_SOURCE_BRANCH")
            .or_else(|| opt_var("AWS_BRANCH"))
            .unwrap_or_default(),
        env_prefix: None,
        head_revision: None,
        id: var("AWS_JOB_ID"),
        provider: CiProvider::AwsAmplify,
        request_id: opt_var("AWS_PULL_REQUEST_ID"),
        request_url: None,
        // set to the literal "HEAD" for manual rebuilds
        revision: var("AWS_COMMIT_ID"),
        url: None,
    }
}
