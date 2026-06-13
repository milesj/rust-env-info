use crate::api::{opt_var, var, CdEnvironment, CdProvider};

// https://octopus.com/docs/projects/variables/system-variables
// Octopus exposes `Octopus.Release.*` variables to deployment scripts through
// `$OctopusParameters`/`#{}` substitution, not the process environment, so
// these are only present when a deployment process exports them explicitly
pub fn create_environment() -> CdEnvironment {
    CdEnvironment {
        branch: opt_var("OCTOPUS_RELEASE_GIT_BRANCHNAME"),
        env_prefix: Some("OCTOPUS_".into()),
        provider: CdProvider::Octopus,
        revision: var("OCTOPUS_RELEASE_GIT_COMMITHASH"),
        service_id: opt_var("OCTOPUS_PROJECT_ID"),
    }
}
