use crate::api::{self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// Google Antigravity editor — sets ANTIGRAVITY_AGENT
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Antigravity,
        env_prefix: Some("ANTIGRAVITY_".into()),
        id: self_id(),
        network: detect_network_policy(AiAgent::Antigravity),
        sandboxed: false,
        session_id: None,
    }
}
