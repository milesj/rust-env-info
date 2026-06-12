use crate::api::{opt_var, var, CiEnvironment, CiProvider};

// https://go-vela.github.io/docs/reference/environment/variables/
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        base_branch: opt_var("VELA_PULL_REQUEST_TARGET"),
        base_revision: None,
        branch: opt_var("VELA_PULL_REQUEST_SOURCE")
            .or_else(|| opt_var("VELA_BUILD_BRANCH"))
            .unwrap_or_default(),
        env_prefix: Some("VELA_".into()),
        head_revision: None,
        id: var("VELA_BUILD_NUMBER"),
        provider: CiProvider::Vela,
        request_id: opt_var("VELA_PULL_REQUEST"),
        request_url: None,
        revision: var("VELA_BUILD_COMMIT"),
        url: opt_var("VELA_BUILD_LINK"),
    }
}
