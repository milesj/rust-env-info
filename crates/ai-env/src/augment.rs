use crate::api::{self_id, AiAgent, AiEnvironment};
use crate::detect_network_policy;

// Augment agent — sets AUGMENT_AGENT
pub fn create_environment() -> AiEnvironment {
    AiEnvironment {
        agent: AiAgent::Augment,
        env_prefix: Some("AUGMENT_".into()),
        id: self_id(),
        network: detect_network_policy(AiAgent::Augment),
        sandboxed: false,
        session_id: None,
    }
}
