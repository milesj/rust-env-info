use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://man.sr.ht/builds.sr.ht/
// Build manifests are arbitrary, so no branch or revision is exposed
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: String::new(),
        env_prefix: None,
        head_revision: None,
        id: var("JOB_ID"),
        provider: CiProvider::Sourcehut,
        request_id: None,
        request_url: None,
        revision: String::new(),
        url: opt_var("JOB_URL"),
    }
}
