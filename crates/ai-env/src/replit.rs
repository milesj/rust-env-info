use crate::api::{self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// Replit (Agent / workspace) — sets REPL_ID. This marks the Replit
// environment, which may be human-driven as well as agent-driven.
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Replit,
        env_prefix: Some("REPL_".into()),
        id: self_id(),
        network: detect_network_policy(AiAgent::Replit),
        sandboxed: false,
        session_id: None,
    }
}
