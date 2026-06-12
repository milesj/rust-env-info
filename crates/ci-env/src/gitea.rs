use crate::api::{CiEnvironment, CiProvider};
use crate::github;

// https://docs.gitea.com/usage/actions/comparison
// Gitea Actions is GitHub Actions compatible and exposes the same `GITHUB_*`
// environment variables, plus `GITEA_ACTIONS=true` to tell them apart
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        provider: CiProvider::GiteaActions,
        ..github::create_environment()
    }
}
