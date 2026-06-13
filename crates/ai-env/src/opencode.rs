use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};

// opencode (SST) — sets OPENCODE / OPENCODE_CLIENT
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::OpenCode,
        env_prefix: Some("OPENCODE_".into()),
        id: self_id(),
        sandboxed: false,
        session_id: opt_var("OPENCODE_SESSION_ID"),
    }
}
