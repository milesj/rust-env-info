use crate::api::{self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// v0 (Vercel) — identifies only via AI_AGENT=v0, no dedicated marker
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::V0,
        env_prefix: None,
        id: self_id(),
        network: detect_network_policy(AiAgent::V0),
        sandboxed: false,
        session_id: None,
    }
}
