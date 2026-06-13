use crate::api::{self_id, AiAgent, AiEnvironment};

// Devin (Cognition) — detected via the `/opt/.devin` filesystem marker rather
// than an environment variable
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Devin,
        env_prefix: None,
        id: self_id(),
        sandboxed: false,
        session_id: None,
    }
}
