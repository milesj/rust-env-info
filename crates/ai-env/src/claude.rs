use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};

// Claude Code (Anthropic) — sets CLAUDECODE=1
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Claude,
        env_prefix: Some("CLAUDE_CODE_".into()),
        id: self_id(),
        sandboxed: false,
        session_id: opt_var("CLAUDE_CODE_SESSION_ID"),
    }
}
