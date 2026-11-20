use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// Cursor editor — its integrated terminal sets CURSOR_TRACE_ID for every
// command, including those a person types, so it's only detected when no other
// agent's marker is present (see `AiAgent::is_human_possible`).
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Cursor,
        env_prefix: Some("CURSOR_".into()),
        id: self_id(),
        network: detect_network_policy(AiAgent::Cursor),
        sandboxed: false,
        session_id: opt_var("CURSOR_TRACE_ID"),
    }
}
