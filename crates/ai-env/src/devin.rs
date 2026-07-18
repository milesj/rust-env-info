use crate::api::{self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// Devin (Cognition) — detected via the `/opt/.devin` filesystem marker rather
// than an environment variable
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Devin,
        env_prefix: None,
        id: self_id(),
        network: detect_network_policy(AiAgent::Devin),
        sandboxed: false,
        session_id: None,
    }
}
