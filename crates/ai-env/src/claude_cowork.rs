use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// Claude Code in Cowork mode — Claude Code with CLAUDE_CODE_IS_COWORK set
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::ClaudeCowork,
        env_prefix: Some("CLAUDE_CODE_".into()),
        id: self_id(),
        network: detect_network_policy(AiAgent::ClaudeCowork),
        sandboxed: false,
        session_id: opt_var("CLAUDE_CODE_SESSION_ID"),
    }
}
