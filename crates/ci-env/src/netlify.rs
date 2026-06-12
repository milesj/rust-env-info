use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://docs.netlify.com/configure-builds/environment-variables/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        // `BRANCH` is the branch being deployed; Netlify doesn't expose
        // the target branch of a pull request
        base_branch: None,
        base_revision: None,
        branch: opt_var("HEAD")
            .or_else(|| opt_var("BRANCH"))
            .unwrap_or_default(),
        env_prefix: None,
        head_revision: None,
        id: var("BUILD_ID"),
        provider: CiProvider::Netlify,
        request_id: opt_var("PULL_REQUEST").map(|_| var("REVIEW_ID")),
        request_url: None,
        revision: var("COMMIT_REF"),
        url: None,
    }
}
