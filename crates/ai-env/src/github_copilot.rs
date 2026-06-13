use crate::api::{self_id, AiAgent, AiEnvironment};

// GitHub Copilot CLI — sets COPILOT_MODEL / COPILOT_ALLOW_ALL / COPILOT_GITHUB_TOKEN
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::GithubCopilot,
        env_prefix: Some("COPILOT_".into()),
        id: self_id(),
        sandboxed: false,
        session_id: None,
    }
}
