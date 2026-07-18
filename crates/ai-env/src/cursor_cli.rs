use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// Cursor CLI agent — sets CURSOR_AGENT when executing commands
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::CursorCli,
        env_prefix: Some("CURSOR_".into()),
        id: self_id(),
        network: detect_network_policy(AiAgent::CursorCli),
        sandboxed: false,
        session_id: opt_var("CURSOR_TRACE_ID"),
    }
}
