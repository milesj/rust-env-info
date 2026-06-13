use crate::api::{self_id, AiAgent, AiEnvironment};

// Google Antigravity editor — sets ANTIGRAVITY_AGENT
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Antigravity,
        env_prefix: Some("ANTIGRAVITY_".into()),
        id: self_id(),
        sandboxed: false,
        session_id: None,
    }
}
