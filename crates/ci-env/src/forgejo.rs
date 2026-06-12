use crate::api::{CiEnvironment, CiProvider};
use crate::github;

// https://forgejo.org/docs/latest/user/actions/reference/
// Forgejo Actions mirrors every `FORGEJO_*` variable as `GITHUB_*` for
// GitHub Actions compatibility, and sets `FORGEJO_ACTIONS=true`
pub fn create_environment() -> CiEnvironment {
    CiEnvironment {
        provider: CiProvider::ForgejoActions,
        ..github::create_environment()
    }
}
