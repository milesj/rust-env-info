use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// opencode (SST) — sets OPENCODE / OPENCODE_CLIENT
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::OpenCode,
        env_prefix: Some("OPENCODE_".into()),
        id: self_id(),
        network: detect_network_policy(AiAgent::OpenCode),
        sandboxed: false,
        session_id: opt_var("OPENCODE_SESSION_ID"),
    }
}
