use crate::api::{self_id, AiAgent, AiEnvironment};

// v0 (Vercel) — identifies only via AI_AGENT=v0, no dedicated marker
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::V0,
        env_prefix: None,
        id: self_id(),
        sandboxed: false,
        session_id: None,
    }
}
