use crate::api::{var, CiEnvironment, CiProvider};

// https://developers.cloudflare.com/pages/configuration/build-configuration/#environment-variables
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: None,
        base_revision: None,
        branch: var("CF_PAGES_BRANCH"),
        env_prefix: Some("CF_PAGES_".into()),
        head_revision: None,
        // no build ID is exposed, and `CF_PAGES_URL` is the deployment
        // URL rather than a link to the build itself
        id: String::new(),
        provider: CiProvider::CloudflarePages,
        request_id: None,
        request_url: None,
        revision: var("CF_PAGES_COMMIT_SHA"),
        url: None,
    }
}
