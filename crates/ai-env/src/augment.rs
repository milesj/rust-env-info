use crate::api::{self_id, AiAgent, AiEnvironment};

// Augment agent — sets AUGMENT_AGENT
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Augment,
        env_prefix: Some("AUGMENT_".into()),
        id: self_id(),
        sandboxed: false,
        session_id: None,
    }
}
