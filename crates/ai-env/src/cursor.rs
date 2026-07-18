use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// Cursor editor — its integrated terminal sets CURSOR_TRACE_ID. Note this can
// be present for human-driven commands, not just agent-driven ones.
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
