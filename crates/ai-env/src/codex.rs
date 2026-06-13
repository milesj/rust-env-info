use crate::api::{opt_var, self_id, AiAgent, AiEnvironment};

// OpenAI Codex — sets CODEX_SANDBOX (e.g. `seatbelt`) when sandboxing commands
// https://developers.openai.com/codex/environment-variables
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Codex,
        env_prefix: Some("CODEX_".into()),
        id: self_id(),
        sandboxed: opt_var("CODEX_SANDBOX").is_some(),
        session_id: None,
    }
}
