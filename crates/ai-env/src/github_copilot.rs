use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// GitHub Copilot CLI — sets COPILOT_MODEL / COPILOT_ALLOW_ALL / COPILOT_GITHUB_TOKEN.
// The cloud agent variant also sets COPILOT_AGENT_SESSION_ID, and its egress
// firewall announces itself via COPILOT_AGENT_FIREWALL_* variables, which
// `detect_network_policy` reports as `Filtered`.
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::GithubCopilot,
        env_prefix: Some("COPILOT_".into()),
        id: self_id(),
        network: detect_network_policy(AiAgent::GithubCopilot),
        sandboxed: false,
        session_id: opt_var("COPILOT_AGENT_SESSION_ID"),
    }
}
