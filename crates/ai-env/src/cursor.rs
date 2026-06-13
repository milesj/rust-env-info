use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};

// Cursor editor — its integrated terminal sets CURSOR_TRACE_ID. Note this can
// be present for human-driven commands, not just agent-driven ones.
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Cursor,
        env_prefix: Some("CURSOR_".into()),
        id: self_id(),
        sandboxed: false,
        session_id: opt_var("CURSOR_TRACE_ID"),
    }
}
